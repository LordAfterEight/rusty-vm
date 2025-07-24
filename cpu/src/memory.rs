use crate::opcodes;
use colored::Colorize;
use std::fs::OpenOptions;
use std::io::{Read, Write, Seek, SeekFrom};
use std::mem;

pub struct Memory {
    pub memory: [u16;u16::MAX as usize]
}

impl Memory {
    /// Initializes the memory
    pub fn init() -> Self {
        let mut mem = Self {
            memory: [opcodes::NO_OPERAT;u16::MAX as usize]
        };

        let mut img = OpenOptions::new()
            .read(true)
            .write(true)
            .open(format!("{}/../memory.img", env!("CARGO_MANIFEST_DIR")))
            .unwrap();

        for i in 0..26 {
            mem.memory[0x0200 + i] = 0x0041 + i as u16;
            mem.memory[0x0220 + i] = 0x0061 + i as u16;
        }

        for i in 0..512 {
            mem.memory[0x0300 + i] = opcodes::GPU_NO_OPERAT;
        }

        // --- Hardcoded "Hello World!" ---
        mem.memory[0x0300] = opcodes::GPU_DRAW_LETT; // Draw OpCode

        mem.memory[0x0301] = mem.memory[0x0207]; // H
        mem.memory[0x0302] = mem.memory[0x0224]; // e
        mem.memory[0x0303] = mem.memory[0x022B]; // l
        mem.memory[0x0304] = mem.memory[0x022B]; // l
        mem.memory[0x0305] = mem.memory[0x022E]; // o

        mem.memory[0x0306] = 0x0020;

        mem.memory[0x0307] = mem.memory[0x0216]; // W
        mem.memory[0x0308] = mem.memory[0x022E]; // o
        mem.memory[0x0309] = mem.memory[0x0231]; // r
        mem.memory[0x030A] = mem.memory[0x022B]; // l
        mem.memory[0x030B] = mem.memory[0x0223]; // d

        mem.memory[0x030C] = 0x0021;

        mem.memory[0x030D] = 0x0060;

        for i in 0..mem.memory.len() {
            _ = img.write_all(format!("{:#06X}\n", mem.memory[i]).as_bytes());
        }

        return mem
    }
    pub fn dump(&mut self) {
        for value in self.memory {
            println!("{:#06X} | {:#06X}", self.memory[value as usize], value);
        }
        println!("{}","\n[i] Dumped memory".yellow());
    }
}
