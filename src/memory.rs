use std::fs::OpenOptions;
use std::io::{Read, Seek, SeekFrom, Write};
use std::os::unix::fs::FileExt;

use crate::opcodes;

#[derive(Debug)]
pub struct Memory {
    pub rom: Vec<u16>,
    pub ram: [u16; 1024],
}

impl Memory {
    /// Initializes the memory
    pub fn init() -> Self {
        let img = OpenOptions::new()
            .read(true)
            .open(format!("{}/ROM.bin", env!("CARGO_MANIFEST_DIR")))
            .unwrap();

        let mut file = img;
        let mut buffer = Vec::new();
        _ = file.read_to_end(&mut buffer).unwrap();

        let rom = buffer.chunks_exact(2)
            .map(|chunk| u16::from_be_bytes([chunk[0], chunk[1]]))
            .collect();


        Self {
            rom,
            ram: [opcodes::NO_OPERAT; 1024],
        }
    }

    pub fn update(&mut self, instr_ptr: u16) {
        let img = OpenOptions::new()
            .write(true)
            .open(format!("{}/ROM.bin", env!("CARGO_MANIFEST_DIR")))
            .unwrap();

        let file = img;
        _ = file.write_at(&self.rom[instr_ptr as usize].to_be_bytes(), instr_ptr as u64 * 16);
    }
}
