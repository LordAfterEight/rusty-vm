pub mod cpu;
pub mod memory;
pub mod opcodes;

pub const FONT_SIZE: f32 = 12.0;

// NOTE: MEMORY LAYOUT
// 0x0000 - 0x01FF | UNASSIGNED
// 0x0200 - 0x0219 | A-Z
// 0x021A - 0x021F | ! " # $ [ ]
// 0x0220 - 0x0239 | a-z
// 0x023A - 0x023F | / < > = - ~
// 0x0240 - 0x0249 | 0 - 9
// 0x024A - 0x024F | : _ | & ? @
// 0x0250          | EMPTY CHAR (0x0020)
// 0x0300 - 0x0FFF | GPU BUFFER (3328 16-bit / 6656B)

fn main() {
    #[cfg(target_os = "windows")]
    println!("Sucks to be on windows");

    let mut cpu = cpu::CPU::init();
    //_ = clearscreen::clear();

    loop {
        if cpu.halt_flag == false {
            println!();
            //_ = clearscreen::clear();
            #[cfg(debug_assertions)]
            debug!(
                "CPU instruction pointer: ",
                format!("{:#06X}", cpu.instr_ptr)
            );
            cpu.update();
        }
    }
}

/// General purpose debug macro
#[macro_export]
macro_rules! debug {
    ($val0:expr) => {
        {
            use colored::Colorize;
            let a = format!("{:?}", $val0).cyan();
            println!("{}: {}", format!("[CPU DEBUG]").green(), a);
        }
    };
    ($val0:expr, $val1:expr) => {
        {
            use colored::Colorize;
            let a = format!("{:?}", $val0).cyan();
            let b = format!("{:?}", $val1).yellow();
            println!("{}: {} | {}", format!("[CPU DEBUG]").green(), a, b);
        }
    }
}

/// converts any value into a string containing the hexadecimal version of that value
#[macro_export]
macro_rules! hex {
    ($($val:expr),+) => {
        $(
            format!("{:#06X}", $val)
        )*
    }
}
