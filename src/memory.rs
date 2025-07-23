use crate::opcodes;

pub struct Memory {
    pub memory: [u16;u16::MAX as usize]
}

impl Memory {
    /// Initializes the memory
    pub fn init() -> Self {
        let mut mem = Self {
            memory: [0x0000;u16::MAX as usize]
        };

        mem.memory[0xFFFA] = opcodes::LOAD_AREG;
        mem.memory[0xFFFB] = 0x0069;
        mem.memory[0xFFFC] = opcodes::JMP_TO_SR;
        mem.memory[0xFFFD] = 0xE621;
        mem.memory[0xE627] = opcodes::RET_TO_OR;
        return mem
    }
}
