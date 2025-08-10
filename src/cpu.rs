use crate::opcodes::*;
use std::default::Default;

#[derive(Debug)]
pub struct CPU {
    pub name: String,

    pub instr_ptr: u16,
    pub stack_ptr: u16,

    pub a_reg: u16,
    pub x_reg: u16,
    pub y_reg: u16,

    pub g_reg: u16,

    pub halt_flag: bool,
    pub eq_flag: bool,

    pub clock_speed: usize, // in Hz

    pub memory: crate::memory::Memory,
}

impl CPU {
    pub fn init() -> Self {
        Self {
            name: String::from("OwO CPU"),

            instr_ptr: 0x1000, // NOTE: Code space is 0x1000 - 0xFFFE => So 64254 spaces for programs
            stack_ptr: 0x0000, // NOTE: 0x00 - 0x3FF => 0 - 1023, so 1024 16-bit addresses in the stack

            a_reg: Default::default(),
            x_reg: Default::default(),
            y_reg: Default::default(),

            g_reg: Default::default(),

            halt_flag: false,
            eq_flag: false,

            clock_speed: 10_000_000, // in Hz

            memory: crate::memory::Memory::init(),
        }
    }

    pub fn increase_instr_ptr(&mut self) {
        match self.instr_ptr {
            0xFFFE => {
                self.instr_ptr = 0x1000;
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

    pub fn read_word(&mut self) -> u16 {
        let instruction = self
            .memory
            .rom[self.instr_ptr as usize];

        self.increase_instr_ptr();
        instruction
    }

    pub fn read_at(&mut self, address: u16) -> u16 {
        let instruction = self
            .memory
            .rom[address as usize];
        instruction
    }

    pub fn update(&mut self) {
        let instruction = self.read_word();
        #[cfg(debug_assertions)]
        crate::debug!("Read value: ", crate::hex!(instruction));
        match instruction {
            NO_OPERAT => {
                #[cfg(debug_assertions)]
                crate::debug!("Doing nothing");
            }

            // --- Load the next value into one of the registers ---
            LOAD_AREG => {
                self.a_reg = self.read_word();
                #[cfg(debug_assertions)]
                crate::debug!("Loaded value into A Register: ", crate::hex!(self.a_reg));
            }
            LOAD_XREG => {
                self.x_reg = self.read_word();
                #[cfg(debug_assertions)]
                crate::debug!("Loaded value into X Register: ", crate::hex!(self.x_reg));
            }
            LOAD_YREG => {
                self.y_reg = self.read_word();
                #[cfg(debug_assertions)]
                crate::debug!("Loaded value into Y Register: ", crate::hex!(self.y_reg));
            }
            LOAD_GREG => {
                self.g_reg = self.read_word();
                #[cfg(debug_assertions)]
                crate::debug!("Loaded value into G Register: ", crate::hex!(self.g_reg));
            }

            // --- Store a register's value to the following address. This copies the value and doesn't move it ---
            STOR_AREG => {
                let addr = self.read_word();
                self.memory.rom[addr as usize] = self.a_reg;
                #[cfg(debug_assertions)]
                crate::debug!("Storing A Register to : ", crate::hex!(addr));
                self.memory.update(addr);
            }
            STOR_XREG => {
                let addr = self.read_word();
                self.memory.rom[addr as usize] = self.x_reg;
                #[cfg(debug_assertions)]
                crate::debug!("Storing X Register to : ", crate::hex!(addr));
                self.memory.update(addr);
            }
            STOR_YREG => {
                let addr = self.read_word();
                self.memory.rom[addr as usize] = self.y_reg;
                #[cfg(debug_assertions)]
                crate::debug!("Storing Y Register to : ", crate::hex!(addr));
                self.memory.update(addr);
            }
            STOR_GREG => {
                let addr = self.read_word();
                self.memory.rom[addr as usize] = self.g_reg;
                #[cfg(debug_assertions)]
                crate::debug!("Storing G Register to : ", crate::hex!(addr));
                self.memory.update(addr);
            }

            // --- Subroutine Things ---
            JMP_TO_SR => {
                self.memory.ram[self.stack_ptr as usize] = self.instr_ptr;
                self.instr_ptr = self.read_word();
                #[cfg(debug_assertions)]
                crate::debug!("Jumping to Subroutine at: ", crate::hex!(self.instr_ptr));
                self.increase_stack_ptr();
            }
            JMP_TO_AD => {
                let address = self.read_word();
                self.instr_ptr = address;
                #[cfg(debug_assertions)]
                crate::debug!("Jumping to: ", crate::hex!(address));
            }
            RET_TO_OR => {
                self.decrease_stack_ptr();
                self.instr_ptr = self.memory.ram[self.stack_ptr as usize];
                #[cfg(debug_assertions)]
                crate::debug!("Returning to:", crate::hex!(self.instr_ptr));
                self.increase_instr_ptr();
            }
            COMP_REGS => {
                let text_1;
                let text_2;
                let mut val_1 = self.read_word();
                let mut val_2 = self.read_word();

                match val_1 {
                    0x0041 => {
                        text_1 = "Register A".to_string();
                        val_1 = self.a_reg;
                    },
                    0x0058 => {
                        text_1 = "Register X".to_string();
                        val_1 = self.x_reg;
                    },
                    0x0059 => {
                        text_1 = "Register Y".to_string();
                        val_1 = self.y_reg;
                    },
                    _ => text_1 = format!("Value {:#06X}", val_1)
                };

                match val_2 {
                    0x0041 => {
                        text_2 = "Register A".to_string();
                        val_2 = self.a_reg;
                    },
                    0x0058 => {
                        text_2 = "Register X".to_string();
                        val_2 = self.x_reg;
                    },
                    0x0059 => {
                        text_2 = "Register Y".to_string();
                        val_2 = self.y_reg;
                    },
                    _ => text_2 = format!("Value {:#06X}", val_2)
                };

                if val_1 == val_2 {
                    self.eq_flag = true;
                } else {
                    self.eq_flag = false;
                }

                #[cfg(debug_assertions)]
                crate::debug!(
                    format!("COMP: comparing {} with {}", text_1, text_2),
                    self.eq_flag
                );
            },
            JUMP_IFEQ => match self.eq_flag {
                true => {
                    self.instr_ptr = self.read_word();
                    #[cfg(debug_assertions)]
                    crate::debug!("JUMP_IFEQ: Jumping to: ", crate::hex!(self.instr_ptr));
                    self.eq_flag = false;
                }
                false => {
                    #[cfg(debug_assertions)]
                    crate::debug!("JUMP_IFEQ: Not jumping");
                    self.increase_instr_ptr();
                }
            },
            JUMP_INEQ => match self.eq_flag {
                true => {
                    #[cfg(debug_assertions)]
                    crate::debug!("JUMP_INEQ: Not jumping");
                    self.increase_instr_ptr();
                    self.eq_flag = false;
                }
                false => {
                    self.instr_ptr = self.read_word();
                    #[cfg(debug_assertions)]
                    crate::debug!("JUMP_INEQ: Jumping to: ", crate::hex!(self.instr_ptr));
                }
            },
            INC_REG_V => {
                let register = self.read_word();
                let value = self.read_word();
                let mut reg = ' ';

                match register {
                    0x0041 => {
                        self.a_reg += value;
                        reg = 'A'
                    }
                    0x0058 => {
                        self.x_reg += value;
                        reg = 'X'
                    }
                    0x0059 => {
                        self.y_reg += value;
                        reg = 'Y'
                    }
                    _ => {}
                }

                #[cfg(debug_assertions)]
                crate::debug!(
                    "Adding value to Register",
                    format!("Value: {} | register: {}", value, reg)
                );
            }
            DEC_REG_V => {
                let register = self.read_word();
                let value = self.read_word();
                let mut reg = ' ';

                match register {
                    0x0041 => {
                        self.a_reg -= value;
                        reg = 'A'
                    }
                    0x0058 => {
                        self.x_reg -= value;
                        reg = 'X'
                    }
                    0x0059 => {
                        self.y_reg -= value;
                        reg = 'Y'
                    }
                    _ => {}
                }

                #[cfg(debug_assertions)]
                crate::debug!(
                    "Subtracting value from register",
                    format!("Value: {} | Register: {}", value, reg)
                );
            }
            MUL_REG_V => {
                let register = self.read_word();
                let value = self.read_word();
                let mut reg = ' ';

                match register {
                    0x0041 => {
                        self.a_reg *= value;
                        reg = 'A'
                    }
                    0x0058 => {
                        self.x_reg *= value;
                        reg = 'X'
                    }
                    0x0059 => {
                        self.y_reg *= value;
                        reg = 'Y'
                    }
                    _ => {}
                }

                #[cfg(debug_assertions)]
                crate::debug!(
                    "Multiplying register value by",
                    format!("Value: {} | Register: {}", value, reg)
                );
            }
            DIV_REG_V => {
                let register = self.read_word();
                let value = self.read_word();
                let mut reg = ' ';

                match register {
                    0x0041 => {
                        self.a_reg /= value;
                        reg = 'A'
                    }
                    0x0058 => {
                        self.x_reg /= value;
                        reg = 'X'
                    }
                    0x0059 => {
                        self.y_reg /= value;
                        reg = 'Y'
                    }
                    _ => {}
                }

                #[cfg(debug_assertions)]
                crate::debug!(
                    "Dividing register value by",
                    format!("Value: {} | Register: {}", value, reg)
                );
            }
            HALT_LOOP => {
                self.halt_flag = true;
                #[cfg(debug_assertions)]
                crate::debug!("HALT: ", self.halt_flag);
            }
            _ => {}
        }

        std::thread::sleep(std::time::Duration::from_micros(
            1_000_000 / self.clock_speed as u64,
        ));
    }
}
