use crate::opcodes;
use colored::Colorize;

pub struct Memory {
    pub memory: [u16;u16::MAX as usize]
}

impl Memory {
    /// Initializes the memory
    pub fn init() -> Self {
        let mut mem = Self {
            memory: [opcodes::NO_OPERAT;u16::MAX as usize]
        };

        for i in 0..26 {
            mem.memory[0x0200 + i] = 0x0041 + i as u16;
            mem.memory[0x0220 + i] = 0x0061 + i as u16;
        }

        mem.memory[0xFFFA] = opcodes::LOAD_AREG;
        mem.memory[0xFFFB] = 0x0069;
        mem.memory[0xFFFC] = opcodes::JMP_TO_SR;
        mem.memory[0xFFFD] = 0xE621;
        mem.memory[0xE627] = opcodes::RET_TO_OR;
        return mem
    }
    pub fn dump(&mut self) {
        for value in self.memory {
            println!("{:#06X} | {:#06X}", self.memory[value as usize], value);
        }
        println!("{}","\n[i] Dumped memory".yellow());
    }
}
