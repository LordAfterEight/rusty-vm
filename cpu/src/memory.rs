use crate::opcodes;
use colored::Colorize;
use std::fs::OpenOptions;
use std::io::{Read, Write};

#[derive(Debug)]
pub struct Memory {
    pub rom: String,
    pub ram: [u16;1024]
}

impl Memory {
    /// Initializes the memory
    pub fn init() -> Self {
        let img = OpenOptions::new()
            .read(true)
            .open(format!("{}/../ROM", env!("CARGO_MANIFEST_DIR")))
            .unwrap();

        let mut file = img;
        let mut buffer = &mut String::new();
        _ = file.read_to_string(&mut buffer);

        Self {
            rom: buffer.to_string(),
            ram: [0x000;1024]
        }
    }

    pub fn update(&mut self) {
        let img = OpenOptions::new()
            .read(true)
            .open(format!("{}/../ROM", env!("CARGO_MANIFEST_DIR")))
            .unwrap();

        let mut file = img;
        let mut buffer = &mut String::new();
        _ = file.read_to_string(&mut buffer);
        self.rom = buffer.to_string();
    }
}
