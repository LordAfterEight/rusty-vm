use crate::opcodes;
use colored::Colorize;
use std::fs::OpenOptions;
use std::io::{Read, Write};

#[derive(Debug)]
pub struct Memory {
    pub memory: String,
}

impl Memory {
    /// Initializes the memory
    pub fn init() -> Self {
        let img = OpenOptions::new()
            .read(true)
            .open(format!("{}/../memory", env!("CARGO_MANIFEST_DIR")))
            .unwrap();

        let mut file = img;
        let mut buffer = &mut String::new();
        _ = file.read_to_string(&mut buffer);

        Self {
            memory: buffer.to_string(),
        }
    }

    pub fn update(&mut self) {
        let img = OpenOptions::new()
            .read(true)
            .open(format!("{}/../memory", env!("CARGO_MANIFEST_DIR")))
            .unwrap();

        let mut file = img;
        let mut buffer = &mut String::new();
        _ = file.read_to_string(&mut buffer);
        self.memory = buffer.to_string();
    }
}
