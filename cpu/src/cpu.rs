use crate::opcodes::*;
use std::process::{Child, Command};
use std::{
    default::Default,
    process::{ChildStdin, ChildStdout},
};

#[derive(Debug)]
pub struct CPU {
    // --- CPU ---
    pub name: String,

    pub instr_ptr: u16,
    pub stack_ptr: u8,

    pub a_reg: u16,
    pub x_reg: u16,
    pub y_reg: u16,

    pub halt_flag: bool,
    pub eq_flag: bool,

    pub clock_speed: usize, // in Hz

    pub memory: crate::memory::Memory,
}

impl CPU {
    pub fn init() -> Self {
        let path = format!("{}/../gpu/target/release/rusty-vm_gpu", env!("CARGO_MANIFEST_DIR"));
        #[cfg(debug_assertions)]

        /*
        crate::debug!("Getting external GPU process from: ", path);
        _ = Some(
            Command::new(path)
            .spawn()
            .unwrap(),
        );
        */

        Self {
            name: String::from("OwO CPU"),

            instr_ptr: 0x0500, // NOTE: Code space is 0x0500 - 0xFFFE => So 64254 spaces for programs
            stack_ptr: 0x00,   // NOTE: 0x00 - 0x1FF => 0 - 511, so 512 16-bit addresses in the stack

            a_reg: Default::default(),
            x_reg: Default::default(),
            y_reg: Default::default(),

            halt_flag: false,
            eq_flag: false,

            clock_speed: 4, // in Hz

            memory: crate::memory::Memory::init(),
        }
    }

    pub fn increase_instr_ptr(&mut self) {
        match self.instr_ptr {
            0xFFFE => {
                self.instr_ptr = 0x0500;
                #[cfg(debug_assertions)]
                crate::debug!("Reached end of memory");
            }
            _ => self.instr_ptr += 1,
        }
    }

    pub fn increase_stack_ptr(&mut self) {
        match self.stack_ptr {
            0xFF => self.stack_ptr = 0x00,
            _ => self.stack_ptr += 1,
        }
    }

    pub fn decrease_stack_ptr(&mut self) {
        match self.stack_ptr {
            0x00 => self.stack_ptr = 0xFF,
            _ => self.stack_ptr -= 1,
        }
    }

    /// Reads the value at the address the instruction pointer is pointing to and returns it
    pub fn read_word(&mut self) -> u16 {
        let instruction = 0;
        if self.memory.rom.lines().nth(self.instr_ptr as usize).is_some() {
            let instruction_string = self
                .memory
                .rom
                .lines()
                .nth(self.instr_ptr as usize)
                .unwrap()
                .to_string();

            let trimmed = instruction_string.trim_start_matches("");

            let instruction = u16::from_str_radix(trimmed, 2).unwrap();
            self.increase_instr_ptr();
            return instruction;
        }
        instruction
    }
 
    /// Reads the value at the address the instruction pointer is pointing to and returns it
    pub fn read_at(&mut self, address: u16) -> u16 {
        let instruction = 0;
        if self.memory.rom.lines().nth(address as usize).is_some() {
            let instruction_string = self
                .memory
                .rom
                .lines()
                .nth(self.instr_ptr as usize)
                .unwrap()
                .to_string();

            let trimmed = instruction_string.trim_start_matches("");

            let instruction = u16::from_str_radix(trimmed, 2).unwrap();
            return instruction;
        }
        #[cfg(debug_assertions)]
        crate::debug!("Unable to read value at address: ", format!("{}", address));
        return instruction;
    }

    pub fn update(&mut self) {
        let instruction = self.read_word();
        match instruction {
            // --- Load the next value into one of the registers ---
            LOAD_AREG => {
                self.a_reg = self.read_word();
                #[cfg(debug_assertions)]
                crate::debug!(
                    "Loaded value into A Register: ",
                    crate::hex!(self.a_reg)
                );
            },
            LOAD_XREG => {
                self.x_reg = self.read_word();
                #[cfg(debug_assertions)]
                crate::debug!(
                    "Loaded value into X Register: ",
                    crate::hex!(self.x_reg)
                );
            },
            LOAD_YREG => {
                self.y_reg = self.read_word();
                #[cfg(debug_assertions)]
                crate::debug!(
                    "Loaded value into Y Register: ",
                    crate::hex!(self.y_reg)
                );
            },

            // --- Subroutine Things ---
            JMP_TO_SR => {
                self.memory.ram[self.stack_ptr as usize] = self.instr_ptr;
                self.instr_ptr = self.read_word();
                #[cfg(debug_assertions)]
                crate::debug!(
                    "Jumping to Subroutine at: ",
                    crate::hex!(self.instr_ptr)
                );
                self.increase_stack_ptr();
            },
            RET_TO_OR => {
                self.decrease_stack_ptr();
                self.instr_ptr = self.memory.ram[self.stack_ptr as usize];
                #[cfg(debug_assertions)]
                crate::debug!(
                    "Returning to:",
                    crate::hex!(self.instr_ptr)
                );
                self.increase_instr_ptr();
            },
            COMP_REGS => {
                let reg_a = self.read_word();
                let reg_b = self.read_word();
                let regs = vec![self.a_reg, self.x_reg, self.y_reg];
                if regs[reg_a as usize - 1] == regs[reg_b as usize - 1] {
                    self.eq_flag = true;
                }
                #[cfg(debug_assertions)]
                crate::debug!("COMP: eq_flag = ", self.eq_flag);
            },
            JUMP_IFEQ => {
                match self.eq_flag {
                    true => {
                        self.instr_ptr = self.read_word();
                        #[cfg(debug_assertions)]
                        crate::debug!(
                            "JUMP_IFEQ: Jumping to: ",
                            crate::hex!(self.instr_ptr)
                        );
                    },
                    false => {
                        #[cfg(debug_assertions)]
                        crate::debug!("JUMP_IFEQ: Not jumping");
                        self.increase_instr_ptr();
                    }
                }
            },
            HALT_LOOP => {
                self.halt_flag = true;
                #[cfg(debug_assertions)]
                crate::debug!("HALT: ", self.halt_flag);
            }
            _ => {}
        }

        self.memory.update();
        std::thread::sleep(std::time::Duration::from_micros(
            1_000_000 / self.clock_speed as u64,
        ));
    }
}
