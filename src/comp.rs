use sdl2::render::WindowCanvas;
use sdl2::video::Window;

use crate::opcode::Opcode;
use crate::utils::str_to_u16;
use sdl2::{pixels::Color, rect::Rect, render::Canvas};
use std::collections::BTreeMap;
use std::fs;

pub struct Display {
    pixels: Vec<Vec<bool>>,
    width: usize,
    height: usize,
}

impl Display {
    pub fn set(&mut self, x: usize, y: usize) {
        self.pixels[y][x] = true;
    }

    pub fn clear(&mut self) {
        self.pixels = vec![vec![false; 64]; 32];
    }

    pub fn unset(&mut self, x: usize, y: usize) {
        self.pixels[y][x] = false;
    }

    pub fn toggle(&mut self, x: usize, y: usize) {
        self.pixels[y][x] = !self.pixels[y][x];
    }

    pub fn xor_set(&mut self, x: usize, y: usize, bit: bool) -> bool {
        let collision = self.pixels[y][x] && bit;
        self.pixels[y][x] ^= bit;
        collision
    }

    pub fn render_sprite(&mut self, x: usize, y: usize, sprite: Vec<u8>) -> bool {
        let mut collision = false;
        for (i, byte) in sprite.iter().enumerate() {
            for j in 0..8 {
                let bit = (byte >> (7 - j)) & 1 == 1;
                let x = (x + j) % 64;
                let y = (y + i) % 32;
                collision |= self.xor_set(x, y, bit);
            }
        }
        collision
    }

    pub fn show(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        for (y, row) in self.pixels.iter().enumerate() {
            for (x, &pixel) in row.iter().enumerate() {
                if pixel {
                    canvas.set_draw_color(Color::RGB(255, 255, 255));
                } else {
                    canvas.set_draw_color(Color::RGB(0, 0, 0));
                }

                let point_width = canvas.window().size().0 / self.pixels[0].len() as u32;
                let point_height = canvas.window().size().1 / self.pixels.len() as u32;

                let x = x as i32 * point_width as i32;
                let y = y as i32 * point_height as i32;
                let rect = Rect::new(x, y, point_width, point_height);
                canvas.fill_rect(rect).unwrap();
            }
        }

        canvas.present();
    }
}

impl Default for Display {
    fn default() -> Self {
        let pixels = vec![vec![false; 64]; 32];
        let width = pixels[0].len();
        let height = pixels.len();
        Self {
            pixels,
            width,
            height,
        }
    }
}

pub struct Memory {
    rom_start: u16,
    rom_end: u16,
    font_start: u16,
    data: Vec<u8>,
}

impl Default for Memory {
    fn default() -> Self {
        Self::new(0x200, 0x50)
    }
}

impl Memory {
    pub fn new(rom_start: u16, font_start: u16) -> Self {
        let mut font_map: BTreeMap<&str, Vec<u8>> = BTreeMap::new();

        font_map.insert("0", vec![0xF0, 0x90, 0x90, 0x90, 0xF0]);
        font_map.insert("1", vec![0x20, 0x60, 0x20, 0x20, 0x70]);
        font_map.insert("2", vec![0xF0, 0x10, 0xF0, 0x80, 0xF0]);
        font_map.insert("3", vec![0xF0, 0x10, 0xF0, 0x10, 0xF0]);
        font_map.insert("4", vec![0x90, 0x90, 0xF0, 0x10, 0x10]);
        font_map.insert("5", vec![0xF0, 0x80, 0xF0, 0x10, 0xF0]);
        font_map.insert("6", vec![0xF0, 0x80, 0xF0, 0x90, 0xF0]);
        font_map.insert("7", vec![0xF0, 0x10, 0x20, 0x40, 0x40]);
        font_map.insert("8", vec![0xF0, 0x90, 0xF0, 0x90, 0xF0]);
        font_map.insert("9", vec![0xF0, 0x90, 0xF0, 0x10, 0xF0]);
        font_map.insert("A", vec![0xF0, 0x90, 0xF0, 0x90, 0x90]);
        font_map.insert("B", vec![0xE0, 0x90, 0xE0, 0x90, 0xE0]);
        font_map.insert("C", vec![0xF0, 0x80, 0x80, 0x80, 0xF0]);
        font_map.insert("D", vec![0xE0, 0x90, 0x90, 0x90, 0xE0]);
        font_map.insert("E", vec![0xF0, 0x80, 0xF0, 0x80, 0xF0]);
        font_map.insert("F", vec![0xF0, 0x80, 0xF0, 0x80, 0x80]);

        let mut data = vec![0; 0xFFF];

        let mut font_pointer = font_start;
        for bytes in font_map.values() {
            for (i, byte) in bytes.iter().enumerate() {
                data[(font_pointer + i as u16) as usize] = *byte;
                font_pointer += 1;
            }
        }

        Self {
            rom_start,
            data,
            font_start,
            rom_end: rom_start,
        }
    }

    pub fn get_instruction_at(&mut self, address: u16) -> u16 {
        let address = address as usize;
        let instruction_bytes: &[u8] = &self.data[address..address + 2];
        let instruction = (instruction_bytes[0] as u16) << 8 | instruction_bytes[1] as u16;
        instruction
    }

    pub fn get_byte_at(&self, address: u16) -> u8 {
        let address = address as usize;
        let byte = self.data[address];
        byte
    }

    pub fn get_rom(&self) -> Vec<u8> {
        self.data[self.rom_start as usize..self.rom_end as usize].to_vec()
    }

    pub fn load_rom(&mut self, rom: Vec<u8>) {
        let mut rom_pointer = self.rom_start; //most chip8 programs start at 0x200
        for instruction in rom {
            if rom_pointer as usize > 4095 {
                panic!("rom too large to fit in memory");
            }
            self.data[rom_pointer as usize] = instruction;
            rom_pointer += 1;
        }
        self.rom_end = rom_pointer;
    }
}

#[derive(Debug)]
pub struct Cpu {
    v: Vec<u8>, //registers V0 - VF
    i: u16,     //I register
    pc: u16,    //Program Counter
    dt: u8,     //delay timer
    st: u8,     //sound timer
    stack: Vec<u16>,
}

impl Default for Cpu {
    fn default() -> Self {
        Self {
            v: vec![0; 16],
            i: 0,
            pc: 0x200,
            dt: 0,
            st: 0,
            stack: Vec::new(),
        }
    }
}

#[derive(Default)]
pub struct Chip8 {
    pub cpu: Cpu,
    pub memory: Memory,
    pub display: Display,
}

impl Chip8 {
    pub fn execute_instruction(&mut self, opcode: Opcode) {
        match opcode {
            Opcode::CLS => self.display.clear(),
            Opcode::JP(address) => self.cpu.pc = address,
            Opcode::LDx(register, byte) => self.cpu.v[register as usize] = byte,
            Opcode::LDxy(x, y) => self.cpu.v[x as usize] = self.cpu.v[y as usize],
            Opcode::ADDx(register, byte) => {
                self.cpu.v[register as usize] = self.cpu.v[register as usize].wrapping_add(byte)
            }

            Opcode::LDIx(x) => {
                let address = self.cpu.i;
                for index in 0..=x {
                    self.cpu.v[index as usize] = self.memory.get_byte_at(address + index as u16);
                }
            }

            Opcode::LDxI(x) => {
                let address = self.cpu.i;
                for index in 0..=x {
                    self.memory.data[(address + index as u16) as usize] =
                        self.cpu.v[index as usize];
                }
            }

            Opcode::LDBx(u8) => {
                let value = self.cpu.v[u8 as usize];
                let v = format!("{:03}", value);
            }

            Opcode::ADDIx(x) => {
                self.cpu.i += self.cpu.v[x as usize] as u16;
            }
            Opcode::LDI(address) => self.cpu.i = address,

            Opcode::SEx(x, byte) => {
                if self.cpu.v[x as usize] == byte {
                    self.cpu.pc += 1;
                }
            }
            Opcode::SNEx(x, byte) => {
                if self.cpu.v[x as usize] != byte {
                    self.cpu.pc += 1;
                }
            }

            Opcode::SExy(x, y) => {
                if self.cpu.v[x as usize] == self.cpu.v[y as usize] {
                    self.cpu.pc += 1;
                }
            }

            Opcode::SNExy(x, y) => {
                if self.cpu.v[x as usize] != self.cpu.v[y as usize] {
                    self.cpu.pc += 1;
                }
            }

            Opcode::ORxy(x, y) => {
                self.cpu.v[x as usize] |= self.cpu.v[y as usize];
            }

            Opcode::ANDxy(x, y) => {
                self.cpu.v[x as usize] &= self.cpu.v[y as usize];
            }

            Opcode::XORxy(x, y) => {
                self.cpu.v[x as usize] ^= self.cpu.v[y as usize];
            }

            Opcode::ADDxy(x, y) => {
                //and and check for carry
                let sum = self.cpu.v[x as usize] as u16 + self.cpu.v[y as usize] as u16;
                let value = (sum & 0xFF) as u8;
                let carry = (sum >> 8) as u8;
                if carry != 0 {
                    self.cpu.v[0xF] = carry;
                }
                self.cpu.v[x as usize] = value
            }

            Opcode::SUBxy(x, y) => {
                let x_val = self.cpu.v[x as usize];
                let y_val = self.cpu.v[y as usize];
                if x_val > y_val {
                    self.cpu.v[x as usize] += y_val;
                } else {
                    self.cpu.v[y as usize] += x_val;
                    self.cpu.v[0xF] = 1;
                }
            }

            Opcode::SHRxy(x, y) => {
                self.cpu.v[x as usize] >>= 1;
                self.cpu.v[0xF] = self.cpu.v[x as usize] & 1;
            }

            Opcode::SUBNxy(x, y) => {
                let x_val = self.cpu.v[x as usize];
                let y_val = self.cpu.v[y as usize];
                if y_val > x_val {
                    self.cpu.v[x as usize] = y_val - x_val;
                } else {
                    self.cpu.v[y as usize] = x_val - y_val;
                    self.cpu.v[0xF] = 1;
                }
            }

            Opcode::SHLxy(x, _) => {
                self.cpu.v[x as usize] <<= 1;
                self.cpu.v[0xF] = self.cpu.v[x as usize] >> 7;
            }

            Opcode::LDxDT(x) => self.cpu.v[x as usize] = self.cpu.dt,

            Opcode::DRWxy(x, y, n) => {
                let mut sprite: Vec<u8> = vec![];
                for i in self.cpu.i..self.cpu.i + n as u16 {
                    let byte = self.memory.get_byte_at(i);
                    sprite.push(byte);
                }
                let collision = self.display.render_sprite(
                    self.cpu.v[x as usize] as usize,
                    self.cpu.v[y as usize] as usize,
                    sprite,
                );
                self.cpu.v[0xF] = collision as u8;
            }
            _ => {}
        }
    }

    pub fn load_rom(&mut self, rom_location: &str) {
        let rom: Vec<u8> = fs::read(rom_location).unwrap();
        self.memory.load_rom(rom);
    }

    pub fn tick(&mut self) {
        //fetch next instruction
        let instruction = self.memory.get_instruction_at(self.cpu.pc);
        let opcode: Opcode = instruction.into();
        self.execute_instruction(opcode.clone());
        if !(matches!(opcode, Opcode::JP(_)) || matches!(opcode, Opcode::JPV0(_))) {
            self.cpu.pc += 2;
        }

        // println!("{:?}", opcode);
        // println!("{:?}", self.cpu);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_converting_opcode() {
        let rom: Vec<u8> = fs::read("./ibm.ch8").unwrap();
        let mut memory = Memory::new(0x200, 0x500);
        memory.load_rom(rom);
        let opcode = Opcode::from(memory.get_instruction_at(0x200));
        assert!(opcode == Opcode::CLS)
    }

    #[test]
    fn test_loading_rom() {
        let rom: Vec<u8> = fs::read("./ibm.ch8").unwrap();
        let mut memory = Memory::new(0x200, 0x500);
        memory.load_rom(rom);
        assert!(memory.get_instruction_at(0x200) == 0x00E0);
    }
}
