use std::fs::OpenOptions;
use std::io::{Read, Seek, SeekFrom, Write};

use crate::opcodes;

#[derive(Debug)]
pub struct Memory {
    pub rom: [u16; 65536],
    pub ram: [u16; 1024],
}

impl Memory {
    /// Initializes the memory
    pub fn init() -> Self {
        let img = OpenOptions::new()
            .read(true)
            .open(format!("{}/ROM", env!("CARGO_MANIFEST_DIR")))
            .unwrap();

        let mut file = img;
        let mut buffer = &mut String::new();
        _ = file.read_to_string(&mut buffer);

        let mut rom = [0; 65536];
        let mut counter = 0;

        _ = buffer.trim_start();

        for line in buffer.lines() {
            rom[counter] = u16::from_str_radix(line, 2).unwrap();
            counter += 1;
        }

        Self {
            rom,
            ram: [opcodes::NO_OPERAT; 1024],
        }
    }

    pub fn update(&mut self, instr_ptr: u16) {
        let img = OpenOptions::new()
            .write(true)
            .open(format!("{}/ROM", env!("CARGO_MANIFEST_DIR")))
            .unwrap();

        let mut file = img;
        _ = file.seek(SeekFrom::Start(instr_ptr as u64 * 17)).unwrap();
        _ = file.write_all(format!("{:016b}\n", self.rom[instr_ptr as usize]).as_bytes());
    }
}
