use std::default::Default;
use crate::opcodes::*;

#[derive(Debug)]
pub struct CPU {
    pub name: String,

    pub instr_ptr: u16,
    pub stack_ptr: u8,

    pub a_reg: u16,
    pub x_reg: u16,
    pub y_reg: u16,

    pub halt_flag: bool,

    pub clock_speed: usize, // in Hz
}

impl CPU {
    pub fn init() -> Self {
        Self {
            name: String::from("OwO CPU"),

            instr_ptr: 0xFFF0,
            stack_ptr: 0x00, // 0x00 - 0xFF => 0 - 255, so 256 16-bit addresses in the stack

            a_reg: Default::default(),
            x_reg: Default::default(),
            y_reg: Default::default(),

            halt_flag: false,

            clock_speed: 4, // in Hz
        }
    }

    pub fn increase_instr_ptr(&mut self) {
        match self.instr_ptr {
            0xFFFE => {
                self.instr_ptr = 0x0100;
                #[cfg(debug_assertions)]
                crate::debug!("Reached end of memory");
            },
            _ => self.instr_ptr += 1
        }
    }

    pub fn increase_stack_ptr(&mut self) {
        match self.stack_ptr {
            0xFF => self.stack_ptr = 0x00,
            _ => self.stack_ptr += 1
        }
    }

    pub fn decrease_stack_ptr(&mut self) {
        match self.stack_ptr {
            0x00 => self.stack_ptr = 0xFF,
            _ => self.stack_ptr -= 1
        }
    }

    /// Reads the value at the address the instruction pointer is pointing to and returns it
    pub fn read_word(&mut self, memory: &crate::memory::Memory) -> u16 {
        let return_value = memory.memory[self.instr_ptr as usize];
        self.increase_instr_ptr();
        return_value
    }

    /// Reads the value at the next address, returns it and continues
    pub fn fetch_word(&mut self, memory: &crate::memory::Memory) -> u16 {
        self.increase_instr_ptr();
        let return_value = memory.memory[self.instr_ptr as usize];
        return_value
    }

    pub fn update(&mut self, memory: &mut crate::memory::Memory) {
        #[cfg(debug_assertions)]
        crate::debug!(&self);
        let instruction = self.read_word(memory);
        match instruction {

            // --- Load the next value into one of the registers ---
            LOAD_AREG => {
                #[cfg(debug_assertions)]
                crate::debug!("Loading value into A Register: ", crate::hex!(memory.memory[self.instr_ptr as usize]));
                self.a_reg = self.read_word(memory);
            },
            LOAD_XREG => {
                #[cfg(debug_assertions)]
                crate::debug!("Loading value into X Register: ", crate::hex!(memory.memory[self.instr_ptr as usize]));
                self.x_reg = self.read_word(memory)
            },
            LOAD_YREG => {
                #[cfg(debug_assertions)]
                crate::debug!("Loading value into Y Register: ", crate::hex!(memory.memory[self.instr_ptr as usize]));
                self.y_reg = self.read_word(memory)
            },

            // --- Subroutine Things ---
            JMP_TO_SR => {
                #[cfg(debug_assertions)]
                crate::debug!("Jumping to Subroutine at: ", crate::hex!(memory.memory[self.instr_ptr as usize]));
                memory.memory[self.stack_ptr as usize] = self.instr_ptr;
                self.increase_stack_ptr();
                self.instr_ptr = self.read_word(memory)
            },
            RET_TO_OR => {
                self.decrease_stack_ptr();
                #[cfg(debug_assertions)]
                crate::debug!("Returning to:", crate::hex!(memory.memory[self.stack_ptr as usize]));
                self.instr_ptr = memory.memory[self.stack_ptr as usize];
                self.increase_instr_ptr();
            }
            _ => {}
        }

        std::thread::sleep(std::time::Duration::from_millis(1000 / self.clock_speed as u64));
    }
}
