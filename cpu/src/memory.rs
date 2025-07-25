use crate::opcodes;
use colored::Colorize;
use std::fs::OpenOptions;
use std::io::Write;

#[derive(Debug)]
pub struct Memory {
    pub memory: [u16;u16::MAX as usize],
    pub file: std::fs::File
}

impl Memory {
    /// Initializes the memory
    pub fn init() -> Self {
        let img = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(format!("{}/../memory.img", env!("CARGO_MANIFEST_DIR")))
            .unwrap();

        let mut mem = Self {
            memory: [opcodes::NO_OPERAT;u16::MAX as usize],
            file: img
        };

        // --- Programming the memory ---
        //
        // NOTE: ASCII
        // Write A - Z to 0x0200
        // Write a - z to 0x0220
        //
        for i in 0..26 {
            mem.memory[0x0200 + i] = 0x0041 + i as u16;
            mem.memory[0x0220 + i] = 0x0061 + i as u16;
        }
        //
        // NOTE: GPU BUFFER
        // Filling GPU buffer with GPU NoOps
        //
        for i in 0..512 {
            mem.memory[0x0300 + i] = opcodes::GPU_NO_OPERAT;
        }

        // NOTE: PROGRAM
        // --- Hardcoded "Hello World!" ---
        mem.memory[0x0300] = opcodes::GPU_DRAW_LETT; // Draw OpCode

        mem.memory[0x0301] = mem.memory[0x0207]; // H
        mem.memory[0x0302] = mem.memory[0x0224]; // e
        mem.memory[0x0303] = mem.memory[0x022B]; // l
        mem.memory[0x0304] = mem.memory[0x022B]; // l
        mem.memory[0x0305] = mem.memory[0x022E]; // o

        mem.memory[0x0306] = 0x000A;

        mem.memory[0x0307] = mem.memory[0x0216]; // W
        mem.memory[0x0308] = mem.memory[0x022E]; // o
        mem.memory[0x0309] = mem.memory[0x0231]; // r
        mem.memory[0x030A] = mem.memory[0x022B]; // l
        mem.memory[0x030B] = mem.memory[0x0223]; // d

        mem.memory[0x030C] = 0x0021;             // !

        mem.memory[0x030D] = 0x0060;             // Escape character ("`")

        mem.memory[0x030E] = opcodes::GPU_UPDATE;
        mem.memory[0x0310] = opcodes::GPU_UPDATE;


        // NOTE: PROGRAM
        // --- Modify Memory at Runtime
        mem.memory[0xFFFD] = opcodes::JMP_TO_SR;
        mem.memory[0xFFFE] = 0x0500;
        mem.memory[0x0501] = opcodes::LOAD_AREG;
        mem.memory[0x0502] = opcodes::GPU_RES_F_BUF;
        mem.memory[0x0503] = opcodes::STOR_AREG;
        mem.memory[0x0504] = 0x030F;

        // mem.memory[0x030E] = opcodes::GPU_RESET_PTR;

        // NOTE: WRITE MEMORY TO FILE
        for i in 0..mem.memory.len() {
            _ = mem.file.write_all(format!("{:#06X}\n", mem.memory[i]).as_bytes());
        }
        return mem
    }

    pub fn update(&mut self) {
        let img = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(format!("{}/../memory.img", env!("CARGO_MANIFEST_DIR")))
            .unwrap();
        self.file = img;
        for i in 0..self.memory.len() {
            _ = self.file.write_all(format!("{:#06X}\n", self.memory[i]).as_bytes());
        }
    }

    pub fn dump(&mut self) {
        for value in self.memory {
            println!("{:#06X} | {:#06X}", self.memory[value as usize], value);
        }
        println!("{}","\n[i] Dumped memory".yellow());
    }
}
