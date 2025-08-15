use colored::Colorize;
use std::char;
use std::io::Write;
use std::{fs::OpenOptions, io::Read};

const ROM_SIZE: usize = 65536; // 64 KiB

// TODO:

mod opcodes;
mod fs;

fn main() {
    let mut memory = [0; ROM_SIZE];

    let in_path = std::env::args()
        .skip(1)
        .next()
        .ok_or("No input file provided")
        .unwrap();

    let out_path = std::env::args()
        .skip(2)
        .next()
        .ok_or("No output directory provided");

    println!(
        "Assembling: {}",
        format!("{}/{}", env!("CARGO_MANIFEST_DIR"), in_path)
    );

    let mut code = OpenOptions::new()
        .read(true)
        .open(format!("{}", in_path))
        .unwrap();

    std::thread::sleep(std::time::Duration::from_millis(100));

    let mut img_file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(format!("{}", out_path.unwrap()))
        .expect("ROM file must exist");

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

    // NOTE: GPU BUFFER
    // Filling GPU buffer with GPU NoOps
    for i in 0..3328 {
        memory[0x0300 + i] = opcodes::GPU_NO_OPERAT;
    }

    let mut instr_ptr: usize = 0x1002;
    let mut gpu_ptr: usize = 0x0300;

    let mut code_line = 1;

    let mut mode = Mode::Normal;
    let mut fs_ptr = 0;
    let mut routine_ptr = 0;
    let mut file_systems = Vec::<fs::FileSystem>::new();
    let mut routines = Vec::<Routine>::new();
    let mut routine_addresses = Vec::<u16>::new();


    for line in code_string.lines() {
        let instruction: Vec<&str> = line.split(' ').collect();
        match mode {
            Mode::DefineFileSystem => {
                match instruction[0] {
                    "size" => {
                        match instruction.len() {
                            1 => panic("Missing size definition", &instruction, code_line, 0),
                            2 => panic("Expected argument of type: num/lit/hex", &instruction, code_line, 1),
                            3 => panic("Expected argument of type: num/lit/hex. Maybe the type annotation is missing?", &instruction, code_line, 2),
                            _ => {}
                        }
                        match instruction[1] {
                            "=" => {
                                let size = parse_hex_lit_num(&instruction, code_line, 2, 0) * 2;
                                routines[routine_ptr].length = size;
                                file_systems[fs_ptr].size = size as usize / 2;
                                println!("  -> Preallocating {} Bytes / {} Addresses", size, size / 2);
                            }
                            _ => panic("Expected \"=\"", &instruction, code_line, 1)
                        }
                    }
                    "end" => {
                        routines[routine_ptr].length = routines[routine_ptr].instructions.len() as u16;
                        routine_addresses.push(routines[routine_ptr].address);
                        instr_ptr += file_systems[fs_ptr].size as usize + 1;
                        mode = Mode::Normal;
                        routine_ptr += 1;
                        fs_ptr += 1;
                    }
                    "   " | "" | "//" => code_line += 1,
                    _ => panic("\nMissing indentation",&instruction, code_line, 0)
                }
            }
            Mode::DefineRoutine => {
                routines[routine_ptr].address = instr_ptr as u16;
                match instruction[0] {
                    "load" => {
                        let register = parse_regs(&instruction, code_line, 1);
                        let instr = match register {
                            0x0041 => opcodes::LOAD_AREG,
                            0x0042 => opcodes::LOAD_BREG,
                            0x0043 => opcodes::LOAD_CREG,
                            0x0044 => opcodes::LOAD_DREG,
                            _ => 0
                        };
                        let value = parse_hex_lit_num(&instruction, code_line, 2, 0);
                        routines[routine_ptr].instructions.push(instr);
                        routines[routine_ptr].instructions.push(value);
                    }
                    "stor" => {
                        let register = parse_regs(&instruction, code_line, 1);
                        let instr = match register {
                            0x0041 => opcodes::STOR_AREG,
                            0x0042 => opcodes::STOR_BREG,
                            0x0043 => opcodes::STOR_CREG,
                            0x0044 => opcodes::STOR_DREG,
                            _ => 0
                        };
                        let addr = parse_hex_lit_num(&instruction, code_line, 2, 0);
                        routines[routine_ptr].instructions.push(instr);
                        routines[routine_ptr].instructions.push(addr);
                    }
                    "draw" => {
                        match instruction[1] {
                            "str" => {
                                let mut color_byte = 0x0A;
                                if instruction.len() > 3 {
                                    match instruction[3] {
                                        "col" => {
                                            color_byte = match instruction[4] {
                                                "red" => 0x0B,
                                                "green" => 0x0C,
                                                "blue" => 0x0D,
                                                "cyan" => 0x0E,
                                                "magenta" => 0x0F,
                                                "white" | _ => 0x0A
                                            };
                                        },
                                        _ => {}
                                    }
                                }
                                routines[routine_ptr].instructions.push(opcodes::LOAD_GREG);
                                routines[routine_ptr].instructions.push(opcodes::GPU_DRAW_TEXT);
                                routines[routine_ptr].instructions.push(opcodes::STOR_GREG);
                                gpu_ptr += 1;

                                let string = instruction[2];

                                for mut char_byte in string.chars() {
                                    if char_byte == '^' {
                                        char_byte = char::from(0x20);
                                    }
                                    let out_word = ((color_byte << 8) as u16) | (char_byte as u16);
                                    routines[routine_ptr].instructions.push(opcodes::LOAD_GREG);
                                    routines[routine_ptr].instructions.push(out_word);
                                    routines[routine_ptr].instructions.push(opcodes::STOR_GREG);
                                    gpu_ptr += 1;
                                }

                                routines[routine_ptr].instructions.push(opcodes::LOAD_GREG);
                                routines[routine_ptr].instructions.push(0x60);
                                routines[routine_ptr].instructions.push(opcodes::STOR_GREG);
                                gpu_ptr += 1;
                            }
                            _ => panic("", &instruction, code_line, 1)
                        }
                    }
                    "cmov" => {
                        let mut instr = 0xA000;
                        match instruction[1] {
                            "up" => instr = opcodes::GPU_MV_C_UP,
                            "do" => instr = opcodes::GPU_MV_C_DOWN,
                            "le" => instr = opcodes::GPU_MV_C_LEFT,
                            "ri" => instr = opcodes::GPU_MV_C_RIGH,
                            "nl" => instr = opcodes::GPU_NEW_LINE,
                            _ => panic("Unknown direction", &instruction, code_line, 2),
                        }
                        routines[routine_ptr].instructions.push(opcodes::LOAD_GREG);
                        routines[routine_ptr].instructions.push(instr);
                        routines[routine_ptr].instructions.push(opcodes::STOR_GREG);

                        routines[routine_ptr].instructions.push(opcodes::LOAD_GREG);
                        routines[routine_ptr].instructions.push(opcodes::GPU_UPDATE);
                        routines[routine_ptr].instructions.push(opcodes::STOR_GREG);
                        gpu_ptr += 2;
                    }
                    "ctrl" => {
                        match instruction[1] {
                            "gpu" => {
                                let mut instr = 0xA000;
                                match instruction[2] {
                                    "clear" => instr = opcodes::GPU_RES_F_BUF,
                                    "reset" => instr = opcodes::GPU_RESET_PTR,
                                    "update" => instr = opcodes::GPU_UPDATE,
                                    _ => panic("Unknown GPU control", &instruction, code_line, 2),
                                }
                                routines[routine_ptr].instructions.push(opcodes::LOAD_GREG);
                                routines[routine_ptr].instructions.push(instr);
                                routines[routine_ptr].instructions.push(opcodes::STOR_GREG);
                                gpu_ptr += 1;
                            }
                            "cpu" => {
                                let mut instr = 0xA000;
                                match instruction[2] {
                                    "reset" => instr = opcodes::NO_OPERAT,
                                    "halt" => instr = opcodes::HALT_LOOP,
                                    _ => panic("Unknown CPU control", &instruction, code_line, 2),
                                }
                                routines[routine_ptr].instructions.push(instr);
                            }
                            _ => panic("Unknown control", &instruction, code_line, 2),
                        }
                    }
                    "radd" => {
                        let register = parse_regs(&instruction, code_line, 1);
                        let value = parse_hex_lit_num(&instruction, code_line, 2, 0);
                        routines[routine_ptr].instructions.push(opcodes::INC_REG_V);
                        routines[routine_ptr].instructions.push(register);
                        routines[routine_ptr].instructions.push(value);
                    }
                    "rsub" => {
                        let register = parse_regs(&instruction, code_line, 1);
                        let value = parse_hex_lit_num(&instruction, code_line, 2, 0);
                        routines[routine_ptr].instructions.push(opcodes::DEC_REG_V);
                        routines[routine_ptr].instructions.push(register);
                        routines[routine_ptr].instructions.push(value);
                    }
                    "rmul" => {
                        let register = parse_regs(&instruction, code_line, 1);
                        let value = parse_hex_lit_num(&instruction, code_line, 2, 0);
                        routines[routine_ptr].instructions.push(opcodes::MUL_REG_V);
                        routines[routine_ptr].instructions.push(register);
                        routines[routine_ptr].instructions.push(value);
                    }
                    "rdiv" => {
                        let register = parse_regs(&instruction, code_line, 1);
                        let value = parse_hex_lit_num(&instruction, code_line, 2, 0);
                        routines[routine_ptr].instructions.push(opcodes::DIV_REG_V);
                        routines[routine_ptr].instructions.push(register);
                        routines[routine_ptr].instructions.push(value);
                    }
                    "jusr" => {
                        let subroutine_name = instruction [1];
                        let new_address = return_routine_address(subroutine_name, &mut routines);
                        routines[routine_ptr].instructions.push(opcodes::JMP_TO_SR);
                        routines[routine_ptr].instructions.push(new_address);
                    }
                    "jump" => {
                        let subroutine_name = instruction [1];
                        let new_address = return_routine_address(subroutine_name, &mut routines);
                        routines[routine_ptr].instructions.push(opcodes::JMP_TO_AD);
                        routines[routine_ptr].instructions.push(new_address);
                    }
                    "juie" => {
                        let subroutine_name = instruction [1];
                        let new_address = return_routine_address(subroutine_name, &mut routines);
                        routines[routine_ptr].instructions.push(opcodes::JUMP_IFEQ);
                        routines[routine_ptr].instructions.push(new_address);
                    }
                    "juin" => {
                        let subroutine_name = instruction [1];
                        let new_address = return_routine_address(subroutine_name, &mut routines);
                        routines[routine_ptr].instructions.push(opcodes::JUMP_INEQ);
                        routines[routine_ptr].instructions.push(new_address);
                    }
                    "comp" => {
                        let val_a;
                        let val_b;
                        if instruction[1] == "reg" {
                            val_a = parse_regs(&instruction, code_line, 2);
                        } else {
                            val_a = parse_hex_lit_num(&instruction, code_line, 2, 0);
                        }
                        if instruction[3] == "reg" {
                            val_b = parse_regs(&instruction, code_line, 4);
                        } else {
                            val_b = parse_hex_lit_num(&instruction, code_line, 3, 0);
                        }
                        routines[routine_ptr].instructions.push(opcodes::COMP_REGS);
                        routines[routine_ptr].instructions.push(val_a);
                        routines[routine_ptr].instructions.push(val_b);
                        if val_a == val_b {
                        }
                    }
                    "rtor" => {
                        routines[routine_ptr].instructions.push(opcodes::RET_TO_OR);
                    }
                    "end" => {
                        routines[routine_ptr].length = routines[routine_ptr].instructions.len() as u16;
                        routine_addresses.push(routines[routine_ptr].address);
                        instr_ptr += routines[routine_ptr].length as usize + 1;
                        mode = Mode::Normal;
                        if routines[routine_ptr].name != "entry" {
                            routine_ptr += 1;
                        }
                        continue;
                    }
                    "   " | "" | "//" => code_line += 1,
                    _ => panic("\nMissing indentation",&instruction, code_line, 0)
                }
            },
            Mode::Normal => {
                match instruction[0] {
                    "routine:" => {
                        mode = Mode::DefineRoutine;
                        routines.push(Routine::new(instruction[1].to_string(), instr_ptr as u16));
                        println!("{} \"{}\" @ {}", "Building routine".green(), routines[routine_ptr].name.cyan(), format!("{:#06X}", instr_ptr).yellow());
                    },
                    "filesys:" => {
                        mode = Mode::DefineFileSystem;
                        routines.push(Routine::new("Filesystem".to_string(), instr_ptr as u16));
                        file_systems.push(fs::FileSystem::new(instr_ptr));
                        println!("{} \"{}\" @ {}", "Building filesystem".magenta(), routines[routine_ptr].name.cyan(), format!("{:#06X}", instr_ptr).yellow());
                    }
                    "#" | "" | "   " => {
                        continue;
                    },
                    _ => panic("",&instruction, code_line, 0)
                }
                code_line += 1;
            }
        }
        code_line += 1;
    }

    let mut addr_used = 0;
    instr_ptr = 0x1000;

    println!();

    memory[0x1000] = opcodes::JMP_TO_SR;
    memory[0x1001] = routine_addresses[routine_addresses.len() - 1];
    instr_ptr += 2;

    for mut filesystem in file_systems {
        for content in &filesystem.content {
            memory[instr_ptr + filesystem.offset_ptr] = content.words[filesystem.offset_ptr];
            filesystem.offset_ptr += 1;
        }
        instr_ptr += filesystem.size as usize;
        addr_used += filesystem.size as usize;
    }

    for mut routine in routines {
        for instruction in &routine.instructions {
            memory[instr_ptr + routine.offset_ptr] = *instruction;
            routine.offset_ptr += 1;
        }
        instr_ptr += routine.length as usize + 1;
    }

    // NOTE: WRITE MEMORY TO FILE
    for line in memory.iter() {
        _ = img_file.write_all(&line.to_be_bytes());
        if (*line != 0x0000) && (*line != 0xA000) {
            addr_used += 1;
        }
    }

    let mut size = addr_used as f32 * 2.0;

    let size_suffix = if addr_used as f32 * 2.0 > 1024.0 { "KiB" } else { "B" };

    if size_suffix == "KiB" {
        size /= 1024.0;
    }

    if size_suffix == "KiB" {
        print!("ROM usage: {:.2}{}/128KiB", size, size_suffix);
    } else {
        print!("ROM usage: {}{}/128KiB", size as usize, size_suffix);
    }

    println!(" | ~{:.2}%", (addr_used as f32 / 65536.0) * 100.0);
}

fn return_routine_address(routine_name: &str, routines: &Vec<Routine>) -> u16 {
    let mut return_address = 0;
    for routine in routines.iter() {
        if routine_name == routine.name {
            return_address = routine.address;
        }
    }
    return return_address
}

fn parse_regs(instruction: &Vec<&str>, code_line: usize, arg_pos: usize) -> u16 {
    let ret = instruction[arg_pos].chars().next().unwrap() as u16;
    match instruction[arg_pos] {
        "A" | "B" | "C" | "D" => {}
        _ => {
            panic("", &instruction, code_line, 1);
        }
    }
    ret
}

fn parse_hex_lit_num(instruction: &Vec<&str>, code_line: usize, arg_pos: usize, arg_mod: usize) -> u16 {
    let mut return_value = 0;
    match instruction[arg_pos - arg_mod] {
        "hex" => {
            return_value = instruction[arg_pos - arg_mod + 1]
                .to_string()
                .chars()
                .next()
                .unwrap() as u16
        }
        "lit" => {
            if instruction[arg_pos - arg_mod + 1] > "F" {
                panic("", &instruction, code_line, arg_pos + 1);
            }
            return_value = u16::from_str_radix(
                instruction[arg_pos - arg_mod + 1].trim_start_matches("0x"),
                16,
            )
            .unwrap()
        }
        "num" => {
            let value = instruction[arg_pos + arg_mod + 1].parse::<u32>().unwrap();
            if value > 65535 {
                panic(
                    "Value too big, must not be bigger than 65535",
                    &instruction,
                    code_line,
                    arg_pos + 1,
                );
            }
            return_value = value as u16;
        }
        _ => panic("", &instruction, code_line, arg_pos),
    }
    return return_value;
}

#[derive(Clone)]
pub struct Statement {
    pub name: String,
    pub address: u16,
    pub offset_ptr: usize,
    pub instructions: Vec<u16>,
    pub length: u16
}

#[derive(Clone)]
pub struct Routine {
    pub name: String,
    pub address: u16,
    pub offset_ptr: usize,
    pub instructions: Vec<u16>,
    pub length: u16
}

impl Routine {
    pub fn new(name: String, ptr: u16) -> Self {
        Self {
            name,
            address: ptr,
            offset_ptr: Default::default(),
            instructions: Vec::new(),
            length: Default::default()
        }
    }
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
        format!(" --> At Line {} | Position {}\n", line, offset).red()
    );
    panic!();
}

enum Mode {
    Normal,
    DefineRoutine,
    DefineFileSystem,
}
