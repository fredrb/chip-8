use crate::instruction::*;

use std::{thread, time};
use rand::Rng;

pub static FONT_SET: [u8; 80] = [
  0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
  0x20, 0x60, 0x20, 0x20, 0x70, // 1
  0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
  0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
  0x90, 0x90, 0xF0, 0x10, 0x10, // 4
  0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
  0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
  0xF0, 0x10, 0x20, 0x40, 0x40, // 7
  0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
  0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
  0xF0, 0x90, 0xF0, 0x90, 0x90, // A
  0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
  0xF0, 0x80, 0x80, 0x80, 0xF0, // C
  0xE0, 0x90, 0x90, 0x90, 0xE0, // D
  0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
  0xF0, 0x80, 0xF0, 0x80, 0x80  // F
];

pub trait Chip8Machine {
    fn new() -> Self;

    fn load_rom(&mut self, rom: &[u8]) -> Result<(), &'static str>;

    fn run(&mut self);
}

trait DebugMachine {
    fn dump_registers(&self);
}

trait InstructionMachine {
    fn fetch_instruction(&self) -> Chip8Instruction;

    fn increment_pc(&mut self);
}

pub struct Machine {
    memory: [u8;4096],
    registers: [u8;16],
    i: u16,
    delay: u8,
    timer: u8,
    pc: usize,
    sp: i16,
    stack: Vec<u16>,

    pub cls: bool,
    pub draw: bool,
    pub screen: [[bool;32];64],
    pub key_pressed: bool,
    pub key: u8,
}

impl InstructionMachine for Machine {
    fn increment_pc(&mut self) {
        self.pc += 2;
    }

    fn fetch_instruction(&self) -> Chip8Instruction {
        let left: u8 = self.memory[self.pc];
        let right: u8 = self.memory[self.pc+1];
        Chip8Instruction::new(left, right)
    }
}

impl Chip8Machine for Machine {

    fn new() -> Self {
        let mut m = Machine {
            memory: [0;4096],
            registers: [0;16],
            i: 0,
            delay: 0,
            timer: 0,
            pc: 0,
            sp: -1,
            stack: vec!(0;16),
            cls: false,
            draw: false,
            screen: [[false;32];64],
            key_pressed: false,
            key: 0,
        };
        // Copy fontset
        for i in 0..80 {
            m.memory[i] = FONT_SET[i];
        }
        return m;
    }


    fn load_rom(&mut self, rom: &[u8]) -> Result<(), &'static str> {
        let mem_prefix = 0x200;
        let mut i = 0;
        loop {
            if rom.len() <= i { break; }
            let instruction = rom[i];
            self.memory[mem_prefix + i] = instruction;
            i += 1;
        }
        self.pc = 0x200;
        Ok(())
    }

    fn run(&mut self) {
        // let sleep_time = time::Duration::from_millis(1);
        // thread::sleep(sleep_time);
        if self.timer > 0 {
            self.timer -= 1;
        }
        let instruction = self.fetch_instruction();
        self.run_opcode(instruction);
    }
}

pub trait OpCodes {

    fn run_opcode(&mut self, instruction: Chip8Instruction);

    // 0x00E0
    fn clear_screen(&mut self);

    // 0x00EE
    fn ret(&mut self);

    // 0x1NNN
    fn jump(&mut self, nnn: u16);

    // 0x2NNN
    fn call(&mut self, nnn: u16);

    // 0x3XNN
    fn cond_equals(&mut self, x: u8, equals: u8);

    // 0x4XNN
    fn cond_not_equals(&mut self, x: u8, equals: u8);

    // 0x5XY0
    fn cond_reg_equals(&mut self, x: u8, y: u8);

    // 0x6XNN
    fn set_register(&mut self, reg: u8, nn: u8);

    // 0x7XNN
    fn add_value(&mut self, reg: u8, nn: u8);

    // 0x9xy0
    fn cond_reg_not_equals(&mut self, x: u8, y: u8);

    // 0x8xyN Operations (Arithmentic)
    fn set_reg(&mut self, x: u8, y: u8); // 0
    fn bitwise_or(&mut self, x: u8, y: u8); // 1
    fn bitwise_and(&mut self, x: u8, y: u8); // 2
    fn bitwise_xor(&mut self, x: u8, y: u8); // 3
    fn add_reg(&mut self, x: u8, y: u8); // 4
    fn sub_reg(&mut self, x: u8, y: u8); // 5
    fn shr(&mut self, x: u8, y: u8); // 6
    fn subn(&mut self, x: u8, y: u8); // 7
    fn shl(&mut self, x: u8, y: u8); // E

    // 0xANNN
    fn set_index (&mut self, index: u16);

    // 0xBNNN
    fn jump_plus(&mut self, index: u16);

    // 0xCNNN
    fn random(&mut self, x: u8, nn: u8);

    // 0xDXYN
    fn display (&mut self, x: u8, u: u8, height: u8);

    // 0xEX9E
    fn if_key (&mut self, x: u8);

    // 0xEXA1
    fn if_not_key (&mut self, x: u8);

    // 0xFX07
    fn get_delay(&mut self, x: u8);

    // 0xFX0A
    fn get_key(&mut self, x: u8);

    // 0xFX15
    fn delay_timer(&mut self, x: u8);

    // 0xFX18
    fn sound_timer(&mut self, x: u8);

    // 0xFX1E
    fn memadd(&mut self, x: u8);

    // 0xFX29
    fn fontset(&mut self, x: u8);

    // 0xFX55
    fn reg_dump(&mut self, x: u8);

    // 0xFX65
    fn reg_load(&mut self, x: u8);

    // 0xFX33
    fn bcd(&mut self, x: u8);

    fn noop (&self);
}

impl OpCodes for Machine {

    fn run_opcode(&mut self, instruction: Chip8Instruction) {
        match instruction {
            _ if instruction.raw == 0x00E0 => self.clear_screen(),
            _ if instruction.raw == 0x00EE => self.ret(),
            i if instruction.match_masked(0xF000, 0x1000) => self.jump(i.raw & 0x0FFF),
            i if i.match_masked(0xF000, 0xB000) => self.jump_plus(i.raw & 0x0FFF),
            i if i.match_masked(0xF000, 0xC000) => self.random(i.parts[1], (i.raw & 0x00FF) as u8),
            i if i.match_masked(0xF000, 0x6000) => self.set_register(i.parts[1], (i.raw & 0x00FF) as u8),
            i if i.match_masked(0xF000, 0x7000) => self.add_value(i.parts[1], (i.raw & 0x00FF) as u8),
            i if i.match_masked(0xF000, 0xA000) => self.set_index(i.raw & 0x0FFF),
            i if i.match_masked(0xF000, 0xD000) => self.display(i.parts[1], i.parts[2], i.parts[3]),
            i if i.match_masked(0xF0FF, 0xE09E) => self.if_key(i.parts[1]),
            i if i.match_masked(0xF0FF, 0xE0A1) => self.if_not_key(i.parts[1]),
            i if i.match_masked(0xF000, 0x3000) => self.cond_equals(i.parts[1], (i.raw & 0x00FF) as u8),
            i if i.match_masked(0xF000, 0x4000) => self.cond_not_equals(i.parts[1], (i.raw & 0x00FF) as u8),
            i if i.match_masked(0xF00F, 0x5000) => self.cond_reg_equals(i.parts[1], i.parts[2]),
            i if i.match_masked(0xF000, 0x8000) => {
                match (i.raw & 0x000F) as u8 {
                    0x00 => self.set_reg(i.parts[1], i.parts[2]),
                    0x01 => self.bitwise_or(i.parts[1], i.parts[2]),
                    0x02 => self.bitwise_and(i.parts[1], i.parts[2]),
                    0x03 => self.bitwise_xor(i.parts[1], i.parts[2]),
                    0x04 => self.add_reg(i.parts[1], i.parts[2]),
                    0x05 => self.sub_reg(i.parts[1], i.parts[2]),
                    0x06 => self.shr(i.parts[1], i.parts[2]),
                    0x07 => self.subn(i.parts[1], i.parts[2]),
                    0x0E => self.shl(i.parts[1], i.parts[2]),
                    _ => self.noop(),
                }
            },
            i if i.match_masked(0xF000, 0xF000) => {
                match (i.raw & 0x00FF) as u8 {
                    0x07 => self.get_delay(i.parts[1]),
                    0x0A => self.get_key(i.parts[1]),
                    0x15 => self.delay_timer(i.parts[1]),
                    0x18 => self.sound_timer(i.parts[1]),
                    0x1E => self.memadd(i.parts[1]),
                    0x29 => self.fontset(i.parts[1]),
                    0x55 => self.reg_dump(i.parts[1]),
                    0x65 => self.reg_load(i.parts[1]),
                    0x33 => self.bcd(i.parts[1]),
                    _ => self.noop(),
                }
            }
            i if i.match_masked(0xF00F, 0x9000) => self.cond_reg_not_equals(i.parts[1], i.parts[2]),
            i if i.match_masked(0xF000, 0x2000) => self.call(i.raw & 0x0FFF),
            _ => self.noop()
        }
    }

    fn clear_screen(&mut self) {
        for x in 0..self.screen.len() {
            for y in 0..self.screen[x].len() {
                self.screen[x][y] = false;
            }
        }
        self.increment_pc();
    }

    fn jump(&mut self, target: u16) {
        self.pc = target as usize;
    }

    fn call(&mut self, target: u16) {
        self.sp += 1;
        self.stack[self.sp as usize] = self.pc as u16;
        self.pc = target as usize;
    }

    fn ret(&mut self) {
        if self.sp < 0 {
            panic!("Return when stack pointer not yet set")
        }
        self.pc = self.stack[self.sp as usize] as usize;
        self.sp += 1;
        self.increment_pc();
    }

    fn set_register(&mut self, x: u8, nn: u8) {
        self.registers[x as usize] = nn;
        self.increment_pc();
    }

    fn jump_plus(&mut self, index: u16) {
        self.pc = (index + self.registers[0x0] as u16) as usize;
    }

    fn random(&mut self, x: u8, nn: u8) {
        let mut rng = rand::thread_rng();
        let random_number: u8 = rng.gen();
        self.registers[x as usize] = random_number & nn;
        self.increment_pc();
    }

    fn add_value(&mut self, x: u8, nn: u8) {
        let result: u16 = (self.registers[x as usize] as u16) + (nn as u16);
        self.registers[x as usize] = result as u8;
        self.increment_pc();
    }

    fn set_index(&mut self, nnn: u16) {
        self.i = nnn;
        self.increment_pc();
    }

    fn cond_equals(&mut self, x: u8, equals: u8) {
        if self.registers[x as usize] == equals {
            self.increment_pc();
        }
        self.increment_pc();
    }

    fn cond_not_equals(&mut self, x: u8, equals: u8) {
        if self.registers[x as usize] != equals {
            self.increment_pc();
        }
        self.increment_pc();
    }

    fn cond_reg_equals(&mut self, x: u8, y: u8) {
        if self.registers[x as usize] == self.registers[y as usize] {
            self.increment_pc();
        }
        self.increment_pc();
    }

    fn cond_reg_not_equals(&mut self, x: u8, y: u8) {
        if self.registers[x as usize] != self.registers[y as usize] {
            self.increment_pc();
        }
        self.increment_pc();
    }

    fn set_reg(&mut self, x: u8, y: u8) {
        self.registers[x as usize] = self.registers[y as usize];
        self.increment_pc();
    }

    fn bitwise_or(&mut self, x: u8, y: u8) {
        self.registers[x as usize] = self.registers[x as usize] | self.registers[y as usize];
        self.increment_pc();
    }

    fn bitwise_and(&mut self, x: u8, y: u8) {
        self.registers[x as usize] = self.registers[x as usize] & self.registers[y as usize];
        self.increment_pc();
    }

    fn bitwise_xor(&mut self, x: u8, y: u8) {
        self.registers[x as usize] = self.registers[x as usize] ^ self.registers[y as usize];
        self.increment_pc();
    }

    fn add_reg(&mut self, x: u8, y: u8) {
        let result: u16 = (self.registers[x as usize] as u16) + (self.registers[y as usize] as u16);
        if result > 0xFF {
            self.registers[0xF] = 0x1;
        } else {
            self.registers[0xF] = 0x0;
        }
        self.registers[x as usize] = result as u8;
        self.increment_pc();
    }

    fn sub_reg(&mut self, x: u8, y: u8) {
        if self.registers[x as usize] > self.registers[y as usize] {
            self.registers[0xF] = 0x1;
        } else {
            self.registers[0xF] = 0x0;
        }
        self.registers[x as usize] = i16::abs(self.registers[x as usize] as i16 - self.registers[y as usize] as i16) as u8;
        self.increment_pc();
    }

    fn shr(&mut self, x: u8, _: u8) {
        self.registers[0xF] = self.registers[x as usize] & 0x01;
        self.registers[x as usize] = self.registers[x as usize] >> 1;
        self.increment_pc();
    }

    fn subn(&mut self, x: u8, y: u8) {
        if self.registers[y as usize] > self.registers[x as usize] {
            self.registers[0xF] = 0x1;
        } else {
            self.registers[0xF] = 0x0;
        }
        self.registers[y as usize] = (self.registers[y as usize] - self.registers[x as usize]) as u8;
        self.increment_pc();
    }

    fn shl(&mut self, x: u8, _: u8) {
        self.registers[0xF] = (self.registers[x as usize] >> 7) & 0x01;
        self.registers[x as usize] = self.registers[x as usize] << 1;
        self.increment_pc();
    }

    fn get_delay(&mut self, x: u8) {
        self.registers[x as usize] = self.delay;
        self.increment_pc();
    }

    fn get_key(&mut self, x: u8) {
        if self.key_pressed {
            self.registers[x as usize] = (self.key & 0x0F) as u8;
            self.increment_pc();
        }
    }

    // 0xEX9E
    fn if_key (&mut self, x: u8) {
        if self.key_pressed && self.registers[x as usize] == self.key {
            self.increment_pc();
        }
        self.increment_pc();
    }

    // 0xEXA1
    fn if_not_key (&mut self, x: u8) {
        if !self.key_pressed || self.registers[x as usize] != self.key {
            self.increment_pc();
        }
        self.increment_pc();
    }

    fn delay_timer(&mut self, x: u8) {
        self.timer = self.registers[x as usize];
        self.increment_pc();
    }

    fn sound_timer(&mut self, _: u8) {
        // ignore for now
        self.increment_pc();
    }

    fn memadd(&mut self, x: u8) {
        self.memory[self.i as usize] = 
            (self.memory[self.i as usize] + self.registers[x as usize]) as u8;
        self.increment_pc();
    }

    fn fontset(&mut self, x: u8) {
        self.i = (x * 5) as u16;
        self.increment_pc();
    }

    fn reg_dump(&mut self, x: u8) {
        for i in 0..((x+1) as usize) {
            self.memory[(self.i + i as u16) as usize] = self.registers[i];
        }
        self.increment_pc();
    }

    fn reg_load(&mut self, x: u8) {
        for i in 0..((x+1) as usize) {
            self.registers[i] = self.memory[(self.i + i as u16) as usize];
        }
        self.increment_pc();
    }

    fn bcd(&mut self, x: u8) {
        let i = self.registers[x as usize];
        self.memory[self.i as usize] = i / 100;
        self.memory[(self.i+1) as usize] = (i / 10) % 10;
        self.memory[(self.i+2) as usize] = i % 10;
        self.increment_pc();
    }

    fn display(&mut self, x: u8, y: u8, height: u8) {
        let x_start = self.registers[x as usize] % 64;
        let y_start = self.registers[y as usize] % 32;

        let sprite = &self.memory[self.i as usize .. (self.i + (height as u16)) as usize];

        for y in 0..sprite.len() {
            let sprite_byte = sprite[y];
            for x in 0..8 {
                let x_index = ((x_start + (x as u8)) % 64) as usize;
                let y_index = ((y_start + (y as u8)) % 32) as usize;
                if ((sprite_byte >> (7 - x)) & 0x01) == 0x01 {
                    if self.screen[x_index][y_index] {
                        self.registers[0xF] = 0x1;
                    }
                    self.screen[x_index][y_index] = !self.screen[x_index][y_index];
                }
            } 
        }
        self.draw = true;
        self.increment_pc();
    }

    fn noop(&self) {
        println!("Executing OpCode => {}", format!("{:#x}", self.fetch_instruction().raw));
        println!("Noop")
    }
}
