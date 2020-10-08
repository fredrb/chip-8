use std::io::Read;
use std::fs::File;

fn load_rom_file(path: &str) -> File {
    return match File::open(path) {
        Err(why) => panic!("Couldn't open {}: {}", path, why),
        Ok(f) => f, 
    };
}

fn load_rom_bytes_from_file(file: &mut File) -> Vec<u8> {
    let mut vec: Vec<u8> = Vec::new();
    return match file.read_to_end(&mut vec) {
        Err(why) => panic!("Couldn't read byte stream: {}", why),
        Ok(size) => {
            println!("Read {} bytes of data", size);
            vec
        },
    }
}

pub fn load_rom(path: &str) -> Vec<u8> {
    let mut file = load_rom_file(path);
    load_rom_bytes_from_file(&mut file)
}