mod gpu;
mod opcodes;

use std::default::Default;

fn window_config() -> macroquad::window::Conf {
    macroquad::window::Conf {
        window_width: 640,
        window_height: 480,
        window_resizable: false,
        ..Default::default()
    }
}

// --- Use Font Size of 20 to have 40 rows and 63 collumns
pub const FONT_SIZE: f32 = 20.0;

// NOTE: MEMORY LAYOUT
// 0x0000 - 0x01FF | STACK (512 16-bit / 1024B)
// 0x0200 - 0x0219 | A-Z
// 0x021A - 0x021F | ! " # $ [ ]
// 0x0220 - 0x0239 | a-z
// 0x023A - 0x023F | / < > = - ~
// 0x0240 - 0x0249 | 0 - 9
// 0x024A - 0x024F | : _ | & ? @
// 0x0250          | EMPTY CHAR (0x0020)
// 0x0300 - 0x04FF | GPU BUFFER (512 16-bit / 1024B)


#[macroquad::main(window_config())]
async fn main() {
    let mut gpu = gpu::GPU::init();
    #[cfg(debug_assertions)]
    debug!("GPU initialized");
    loop {
        //#[cfg(debug_assertions)]
        //debug!(gpu);

        if macroquad::input::is_key_pressed(macroquad::input::KeyCode::Q) {
            std::process::exit(0);
        }

        gpu.update();

        macroquad::window::next_frame().await;
    }
}

/// General purpose debug macro
#[macro_export]
macro_rules! debug {
    ($val0:expr) => {
        use colored::Colorize;
        let a = format!("{:?}", $val0).cyan();
        println!("{}: {}", format!("[GPU DEBUG]").magenta(), a);
    };
    ($val0:expr, $val1:expr) => {
        use colored::Colorize;
        let a = format!("{:?}", $val0).cyan();
        let b = format!("{:?}", $val1).yellow();
        println!("{}: {} | {}", format!("[GPU DEBUG]").magenta(), a, b);
    };
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
