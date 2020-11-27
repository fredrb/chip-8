mod cpu;
mod instruction;
mod external;

use cpu::Chip8Machine;
use std::thread;
use std::time::Duration;
use std::env;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

struct Logger {
    pub f: File,
}

impl Logger {
    fn new(path: &Path) -> Self {
        match File::create(path) {
            Ok(f) => Logger { f },
            Err(why) => panic!("Couldn't open/create file {}: {}", path.display(), why),
        }
    }

    fn log_machine(&mut self, m: &cpu::Machine, last_instruction: u16) {
        self.f.write(format!("\nINSTRUCTION: {:#x}", last_instruction).as_bytes());
        self.f.write_all(format!("\nI: {} |", m.i).as_bytes());
        self.f.write_all(format!("Timer: {} |", m.delay_timer).as_bytes());
        self.f.write_all(format!("pc: {} | ", m.pc).as_bytes());
        self.f.write_all(format!("sp: {} | ", m.sp).as_bytes());

        self.f.write_all(String::from("\nStack:").as_bytes());
        for i in m.stack.iter() {
            self.f.write(format!("{} | ", i).as_bytes());
        }

        self.f.write_all(String::from("\nRegisters:").as_bytes());
        for i in 0..15 {
            self.f.write(format!("{:#x}: {} | ", i, m.registers[i]).as_bytes());
        }
    }
}

fn main() {
    let sleep_duration = Duration::from_millis(2);

    let mut log = Logger::new(Path::new("./dump-machine.txt"));

    let args: Vec<String> = env::args().collect();
    let cartridge_filename = &args[1];

    let file_path: String = String::from(cartridge_filename);
    let bytes = external::load_rom(&file_path);

    let mut chip8_machine: cpu::Machine = cpu::Machine::new();
    chip8_machine.load_rom(&bytes).unwrap();
    

    let sdl_context = sdl2::init().unwrap();

    let mut display = external::Screen::new(&sdl_context);
    let mut input = external::Input::new(&sdl_context);

    while let Ok(keypad) = input.poll() {
        // if keypad < 0 {
        //     chip8_machine.key_pressed = false;
        // } else {
        //     chip8_machine.key_pressed = true;
        //     chip8_machine.key = keypad as u8;
        // }

        let i = chip8_machine.run(keypad);
        log.log_machine(&chip8_machine, i);
        if chip8_machine.draw {
            display.draw(&chip8_machine.screen);
            chip8_machine.draw = false;
        }
        thread::sleep(sleep_duration);
    }
}
