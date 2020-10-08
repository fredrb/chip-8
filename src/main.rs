mod cpu;
mod instruction;
mod external;

use cpu::Chip8Machine;
use std::thread;
use std::time::Duration;
use std::env;

fn main() {
    let sleep_duration = Duration::from_millis(2);

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
        if keypad < 0 {
            chip8_machine.key_pressed = false;
        } else {
            chip8_machine.key_pressed = true;
            chip8_machine.key = keypad as u8;
        }

        chip8_machine.run();
        if chip8_machine.draw {
            display.draw(&chip8_machine.screen);
            chip8_machine.draw = false;
        }
        thread::sleep(sleep_duration);
    }
}
