use colored::Colorize;
use std::char;
use std::io::{Seek, SeekFrom, Write};
use std::{fs::OpenOptions, io::Read};
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
        .open(format!("{}/../memory", env!("CARGO_MANIFEST_DIR")))
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
    // ---------------------------------------- //

    // NOTE: PROGRAM
    // --- Modify Memory at Runtime ---

    let mut instr_ptr: usize = 0x0500;
    let stack_ptr: usize = 0x0000; // NOTE: 0x000 - 0x01FF

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
                        let instr = parse_regs(&instruction, code_line);
                        let value = parse_hex_lit(&instruction, 2, code_line);
                        memory[instr_ptr] = instr;
                        memory[instr_ptr + 1] = value;
                        instr_ptr += 2;
                    },
                    "" => {},
                    _ => {
                        panic(&instruction, code_line, 0);
                    }
                }
            }
        }
        code_line += 1;
    }

    // NOTE: WRITE MEMORY TO FILE
    for i in 0..memory.len() {
        _ = img_file.write_all(format!("{:0b}\n", memory[i]).as_bytes());
    }
}

fn parse_regs(instruction: &Vec<&str>, code_line: usize) -> u16 {
    let mut ret = 0;
    match instruction[1] {
        "A" => ret = opcodes::LOAD_AREG,
        "X" => ret = opcodes::LOAD_XREG,
        "Y" => ret = opcodes::LOAD_YREG,
        _ => {
            panic(&instruction, code_line, 1);
        }
    }
    ret
}

fn parse_hex_lit(instruction: &Vec<&str>, arg_1: usize, code_line: usize) -> u16 {
    let mut return_value = 0;
    match instruction[arg_1] {
        "hex" => return_value = instruction[arg_1 + 1].to_string().chars().next().unwrap() as u16,
        "lit" => return_value = u16::from_str_radix(instruction[arg_1 + 1].trim_start_matches("0x"), 16).unwrap(),
        _ => panic(&instruction, code_line, 2)
    }
    return return_value
}

fn panic(instruction: &Vec<&str>, line: usize, instr: usize) {
    print!(
        "{}",
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
