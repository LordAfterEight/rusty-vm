use colored::Colorize;
use std::char;
use std::io::{Seek, SeekFrom, Write};
use std::{fs::OpenOptions, io::Read};

use crate::opcodes::LOAD_AREG;
mod opcodes;

fn main() {
    let mut memory = [0; u16::MAX as usize];

    println!("Opening code file...");
    let mut code = OpenOptions::new()
        .read(true)
        .open(format!("{}/../code.rvmasm", env!("CARGO_MANIFEST_DIR")))
        .unwrap();

    println!("Opening memory file...");
    let mut img_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(format!("{}/../ROM", env!("CARGO_MANIFEST_DIR")))
        .unwrap();

    let mut code_string = String::new();
    _ = code.read_to_string(&mut code_string).unwrap().to_string();

    // --- Programming the memory ---
    //
    // NOTE: ASCII
    // Write A - Z to 0x0200
    // Write a - z to 0x0220
    for i in 0..26 {
        memory[0x0200 + i] = 0x0041 + i as u16;
        memory[0x0220 + i] = 0x0061 + i as u16;
    }

    for i in 0..9 {
        memory[0x0240 + i] = 0x0030 + i as u16;
    }

    memory[0x021A] = 0x0021; // !
    memory[0x021B] = 0x0022; // "
    memory[0x021C] = 0x0023; // #
    memory[0x021D] = 0x0024; // $
    memory[0x021E] = 0x005B; // [
    memory[0x021F] = 0x005C; // ]
    memory[0x023A] = 0x005D; // /
    memory[0x023B] = 0x003C; // <
    memory[0x023C] = 0x003E; // >
    memory[0x023D] = 0x003D; // =
    memory[0x023E] = 0x002D; // -
    memory[0x023F] = 0x007E; // ~
    memory[0x024A] = 0x003A; // :
    memory[0x024B] = 0x005F; // _
    memory[0x024C] = 0x007C; // |
    memory[0x024D] = 0x0026; // &
    memory[0x024E] = 0x003F; // ?
    memory[0x024F] = 0x0040; // @
    memory[0x0250] = 0x0020; // [SPACE]
    memory[0x0251] = 0x002E;

    //
    // NOTE: GPU BUFFER
    // Filling GPU buffer with GPU NoOps
    //
    for i in 0..512 {
        memory[0x0300 + i] = opcodes::GPU_NO_OPERAT;
    }

    // NOTE: PROGRAM
    // ------- Hardcoded Boot Message ------- //
    /*
    memory[0x0300] = opcodes::GPU_DRAW_LETT; // Draw OpCode

    memory[0x0301] = memory[0x0211]; // R
    memory[0x0302] = memory[0x0234]; // u
    memory[0x0303] = memory[0x0232]; // s
    memory[0x0304] = memory[0x0233]; // t
    memory[0x0305] = memory[0x0238]; // y
    memory[0x0306] = memory[0x023E]; // -
    memory[0x0307] = memory[0x0215]; // V
    memory[0x0308] = memory[0x020C]; // M
    memory[0x0309] = memory[0x0250]; // [SPACE]
    memory[0x030A] = memory[0x0215]; // V
    memory[0x030B] = memory[0x0240]; // 0
    memory[0x030C] = memory[0x0251]; // .
    memory[0x030D] = memory[0x0241]; // 1

    memory[0x030E] = 0x0060; // Escape character ("`")

    memory[0x030F] = opcodes::GPU_UPDATE;
    */
    // ---------------------------------------- //

    // NOTE: PROGRAM
    // --- Modify Memory at Runtime ---

    let mut instr_ptr: usize = 0x0500;
    let mut gpu_ptr: usize = 0x0300;

    let mut regs = vec![0,0,0];
    let mut eq_flag = false;

    let mut code_line = 1;

    println!("Programming memory...\n");
    for line in code_string.lines() {
        let instruction: Vec<&str> = line.split(' ').collect();
        match instruction[0] {
            "//" => {
                code_line += 1;
                continue;
            },
            _ => {
                match instruction[0] {
                    "load" => {
                        if instruction.len() < 3 {
                            panic("Missing Argument", &instruction, code_line, 0);
                        }
                        let reg = parse_regs(&instruction, code_line, 1);
                        let value = parse_hex_lit(&instruction, code_line, 2, 0);
                        let instr = match reg {
                            0x0041 => opcodes::LOAD_AREG,
                            0x0058 => opcodes::LOAD_XREG,
                            0x0059 => opcodes::LOAD_YREG,
                            _ => 0
                        };
                        memory[instr_ptr] = instr;
                        memory[instr_ptr + 1] = value;
                        match instr {
                            1 => regs[0] = value,
                            2 => regs[1] = value,
                            3 => regs[2] = value,
                            _ => {}
                        }
                        instr_ptr += 2;
                    },
                    "stor" => {
                        if instruction.len() < 3 {
                            panic("Missing Argument", &instruction, code_line, 0);
                        }
                        let reg = parse_regs(&instruction, code_line, 1);
                        let addr = parse_hex_lit(&instruction, code_line, 2, 0);
                        let instr = match reg {
                            0x0041 => opcodes::STOR_AREG,
                            0x0058 => opcodes::STOR_XREG,
                            0x0059 => opcodes::STOR_YREG,
                            _ => 0
                        };
                        memory[instr_ptr] = instr;
                        memory[instr_ptr + 1] = addr;
                    }
                    "jump" => {
                        if instruction.len() < 3 {
                            panic("Missing Argument", &instruction, code_line, 0);
                        }
                        memory[instr_ptr] = opcodes::JMP_TO_AD;
                        memory[instr_ptr + 1] = parse_hex_lit(&instruction, code_line, 1, 0);
                        instr_ptr += 2;
                    },
                    "jusr" => {
                        if instruction.len() < 2 {
                            panic("Missing Argument", &instruction, code_line, 0);
                        }
                        memory[instr_ptr] = opcodes::JMP_TO_SR;
                        memory[instr_ptr + 1] = parse_hex_lit(&instruction, code_line, 1, 0);
                        let new_addr = memory[instr_ptr + 1];
                        instr_ptr = new_addr as usize;
                    },
                    "jieq" => {
                        if instruction.len() < 2 {
                            panic("Missing Argument", &instruction, code_line, 0);
                        }
                        memory[instr_ptr] = opcodes::JUMP_IFEQ;
                        let address = parse_hex_lit(&instruction, code_line, 1, 0);
                        memory[instr_ptr + 1] = address;
                        if eq_flag {
                            instr_ptr = address as usize;
                        } else {
                            instr_ptr += 2;
                        }
                    },
                    "jine" => {
                        if instruction.len() < 2 {
                            panic("Missing Argument", &instruction, code_line, 0);
                        }
                        memory[instr_ptr] = opcodes::JUMP_INEQ;
                        let address = parse_hex_lit(&instruction, code_line, 1, 0);
                        memory[instr_ptr + 1] = address;
                        if eq_flag {
                            instr_ptr = address as usize;
                        } else {
                            instr_ptr += 2;
                        }
                    },
                    "rtor" => {
                        memory[instr_ptr] = opcodes::RET_TO_OR;
                    },
                    "noop" => {
                        memory[instr_ptr] = opcodes::NO_OPERAT;
                        instr_ptr += 1;
                    },
                    "setv" => {
                        if instruction.len() < 3 {
                            panic("Missing Argument", &instruction, code_line, 0);
                        }
                        let address = parse_hex_lit(&instruction, code_line, 1, 0);
                        let value = parse_hex_lit(&instruction, code_line, 3, 0);
                        memory[address as usize] = value;
                    },
                    "draw" => {
                        if instruction.len() < 3 {
                            panic("Missing Argument", &instruction, code_line, 0);
                        }
                        match instruction[1] {
                            "str" => {
                                memory[instr_ptr] = opcodes::LOAD_AREG;
                                memory[instr_ptr + 1] = opcodes::GPU_DRAW_LETT;
                                memory[instr_ptr + 2] = opcodes::STOR_AREG;
                                memory[instr_ptr + 3] = gpu_ptr as u16;
                                gpu_ptr += 1;
                                instr_ptr += 4;
                                for mut char in instruction[2].chars() {
                                    if char == '^' {
                                        char = char::from_u32(0x0020).unwrap();
                                    }
                                    memory[instr_ptr] = opcodes::LOAD_AREG;
                                    memory[instr_ptr + 1] = char as u16;
                                    memory[instr_ptr + 2] = opcodes::STOR_AREG;
                                    memory[instr_ptr + 3] = gpu_ptr as u16;
                                    gpu_ptr += 1;
                                    instr_ptr += 4;
                                }
                                memory[instr_ptr] = opcodes::LOAD_AREG;
                                memory[instr_ptr + 1] = 0x0060;
                                memory[instr_ptr + 2] = opcodes::STOR_AREG;
                                memory[instr_ptr + 3] = gpu_ptr as u16;
                                gpu_ptr += 1;
                                instr_ptr += 4;

                                memory[instr_ptr] = opcodes::LOAD_AREG;
                                memory[instr_ptr + 1] = opcodes::GPU_UPDATE;
                                memory[instr_ptr + 2] = opcodes::STOR_AREG;
                                memory[instr_ptr + 3] = gpu_ptr as u16;
                                gpu_ptr += 1;
                                instr_ptr += 4;
                            },
                            _ => panic("", &instruction, code_line, 1),
                        }
                    },
                    "gmov" => {
                        if instruction.len() < 2 {
                            panic("Missing Argument", &instruction, code_line, 0);
                        }
                        match instruction[1] {
                            "up" => memory[gpu_ptr] = opcodes::GPU_MV_C_UP,
                            "do" => memory[gpu_ptr] = opcodes::GPU_MV_C_DOWN,
                            "le" => memory[gpu_ptr] = opcodes::GPU_MV_C_LEFT,
                            "ri" => memory[gpu_ptr] = opcodes::GPU_MV_C_RIGH,
                            "nl" => memory[gpu_ptr] = opcodes::GPU_NEW_LINE,
                            _ => panic("", &instruction, code_line, 1),
                        }
                        memory[gpu_ptr + 1] = opcodes::GPU_UPDATE;
                        gpu_ptr += 2;
                    },
                    "comp" => {
                        if instruction.len() < 5 {
                            panic("Missing Argument", &instruction, code_line, 0);
                        }
                        let val_1 = match instruction[1] {
                            "reg" => parse_regs(&instruction, code_line, 2),
                            _ => parse_hex_lit(&instruction, code_line, 1, 0)
                        };
                        let val_2 = match instruction[3] {
                            "reg" => parse_regs(&instruction, code_line, 4),
                            _ => parse_hex_lit(&instruction, code_line, 3, 0)
                        };
                        memory[instr_ptr] = opcodes::COMP_REGS;
                        memory[instr_ptr + 1] = val_1;
                        memory[instr_ptr + 2] = val_2;
                        if val_1 == val_2 {
                            eq_flag = true;
                        }
                        instr_ptr += 3;
                    },
                    "radd" => {
                        if instruction.len() < 3 {
                            panic("Missing Argument", &instruction, code_line, 0);
                        }
                        let reg = parse_regs(&instruction, code_line, 1);
                        let value = parse_hex_lit(&instruction, code_line, 2, 0);
                        memory[instr_ptr] = opcodes::INC_REG_V;
                        memory[instr_ptr + 1] = reg;
                        memory[instr_ptr + 2] = value;
                        //regs[reg as usize - 1] += value;
                        instr_ptr += 3;
                    },
                    "halt" => {
                        memory[instr_ptr] = opcodes::HALT_LOOP;
                    }
                    "" => {},
                    _ => {
                        panic("", &instruction, code_line, 0);
                    }
                }
            }
        }
        code_line += 1;
    }

    // NOTE: WRITE MEMORY TO FILE
    for line in memory.iter() {
        _ = img_file.write_all(format!("{:016b}\n", line).as_bytes());
    }
}

fn parse_regs(instruction: &Vec<&str>, code_line: usize, arg_pos: usize) -> u16 {
    let ret = instruction[arg_pos].chars().next().unwrap() as u16;
    match instruction[arg_pos] {
        "A" | "X" | "Y" => {},
        _ => {
            panic("", &instruction, code_line, 1);
        }
    }
    ret
}

fn parse_hex_lit(instruction: &Vec<&str>, code_line: usize, arg_pos: usize, arg_mod: usize) -> u16 {
    let mut return_value = 0;
    match instruction[arg_pos - arg_mod] {
        "hex" => return_value = instruction[arg_pos - arg_mod + 1].to_string().chars().next().unwrap() as u16,
        "lit" => {
            if instruction[arg_pos - arg_mod + 1] > "F" {
                panic("", &instruction, code_line, arg_pos + 1);
            }
            return_value = u16::from_str_radix(instruction[arg_pos - arg_mod + 1].trim_start_matches("0x"), 16).unwrap()
        },
        "num" => {
            let value = instruction[arg_pos + arg_mod + 1].parse::<u32>().unwrap();
            if value > 65535 {
                panic("Value too big, must not be bigger than 65535", &instruction, code_line, arg_pos + 1);
            }
            return_value = value as u16;
        }
        _ => panic("", &instruction, code_line, arg_pos)
    }
    return return_value
}

fn panic(message: &str, instruction: &Vec<&str>, line: usize, instr: usize) {
    print!(
        "{}\n{}",
        message.red(),
        format!("Invalid Syntax: \"{}\"\n", instruction[instr]).red()
    );
    let mut offset = instr + 1;
    if instr > 0 {
        for x in 0..instr {
            offset += instruction[x].len();
        }
    }

    print!(
        "{}",
        format!(
            " --> At Line {} | Position {}\n",
            line,
            offset
        )
        .red()
    );
    panic!();
}
