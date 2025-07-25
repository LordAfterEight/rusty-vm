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

    pub clock_speed: usize, // in Hz

    pub memory: crate::memory::Memory,

    // --- GPU Child
    pub gpu: Option<Child>,
}

impl CPU {
    pub fn init() -> Self {
        let path = format!("{}/../gpu/target/debug/rusty-vm_gpu", env!("CARGO_MANIFEST_DIR"));
        #[cfg(debug_assertions)]
        crate::debug!("Getting external GPU process from: ", path);

        let gpu = Some(
            Command::new(path)
            .spawn()
            .unwrap(),
        );

        Self {
            name: String::from("OwO CPU"),

            instr_ptr: 0xFFF0,
            stack_ptr: 0x00, // 0x00 - 0x1FF => 0 - 511, so 512 16-bit addresses in the stack

            a_reg: Default::default(),
            x_reg: Default::default(),
            y_reg: Default::default(),

            halt_flag: false,

            clock_speed: 2, // in Hz

            memory: crate::memory::Memory::init(),

            gpu,
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
        let return_value = self.memory.memory[self.instr_ptr as usize];
        self.increase_instr_ptr();
        return_value
    }

    pub fn update(&mut self) {
        let instruction = self.read_word();
        match instruction {
            // --- Load the next value into one of the registers ---
            LOAD_AREG => {
                #[cfg(debug_assertions)]
                crate::debug!(
                    "Loading value into A Register: ",
                    crate::hex!(self.memory.memory[self.instr_ptr as usize])
                );
                self.a_reg = self.read_word();
            },
            LOAD_XREG => {
                #[cfg(debug_assertions)]
                crate::debug!(
                    "Loading value into X Register: ",
                    crate::hex!(self.memory.memory[self.instr_ptr as usize])
                );
                self.x_reg = self.read_word()
            },
            LOAD_YREG => {
                #[cfg(debug_assertions)]
                crate::debug!(
                    "Loading value into Y Register: ",
                    crate::hex!(self.memory.memory[self.instr_ptr as usize])
                );
                self.y_reg = self.read_word()
            },

            // --- Store the saved value into the memory at the following address ---
            STOR_AREG => {
                #[cfg(debug_assertions)]
                crate::debug!(
                    "Storing A register value to address: ",
                    crate::hex!(self.memory.memory[self.instr_ptr as usize])
                );
                let address = self.read_word();
                self.memory.memory[address as usize] = self.a_reg;
            },
            STOR_XREG => {
                #[cfg(debug_assertions)]
                crate::debug!(
                    "Storing X register value to address: ",
                    crate::hex!(self.memory.memory[self.instr_ptr as usize])
                );
                let address = self.read_word();
                self.memory.memory[address as usize] = self.x_reg;
            },
            STOR_YREG => {
                #[cfg(debug_assertions)]
                crate::debug!(
                    "Storing Y register value to address: ",
                    crate::hex!(self.memory.memory[self.instr_ptr as usize])
                );
                let address = self.read_word();
                self.memory.memory[address as usize] = self.y_reg;
            },

            // --- Subroutine Things ---
            JMP_TO_SR => {
                #[cfg(debug_assertions)]
                crate::debug!(
                    "Jumping to Subroutine at: ",
                    crate::hex!(self.memory.memory[self.instr_ptr as usize])
                );
                self.memory.memory[self.stack_ptr as usize] = self.instr_ptr;
                self.increase_stack_ptr();
                self.instr_ptr = self.read_word()
            },
            RET_TO_OR => {
                self.decrease_stack_ptr();
                #[cfg(debug_assertions)]
                crate::debug!(
                    "Returning to:",
                    crate::hex!(self.memory.memory[self.stack_ptr as usize])
                );
                self.instr_ptr = self.memory.memory[self.stack_ptr as usize];
                self.increase_instr_ptr();
            },

            // --- GPU Things ---
            GPU_UPDATE => {
                #[cfg(debug_assertions)]
                crate::debug!("Updating GPU");
            },
            _ => {}
        }

        self.memory.update();
        std::thread::sleep(std::time::Duration::from_micros(
            1_000_000 / self.clock_speed as u64,
        ));
    }
}
