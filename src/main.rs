mod cpu;
mod gpu;
mod memory;
mod opcodes;

fn window_setup() -> macroquad::window::Conf {
    macroquad::window::Conf {
        window_title: "Rusty VM".to_string(),
        window_width: 640,
        window_height: 480,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_setup())]
async fn main() {
    let mut cpu = cpu::CPU::init();
    let mut gpu = gpu::GPU::init();
    let mut mem = memory::Memory::init();

    #[cfg(debug_assertions)]
    debug!(cpu);

    loop {
        cpu.update(&mut mem);
        gpu.update(&mut mem);
        macroquad::window::next_frame().await;
    }
}

/// General purpose debug macro
#[macro_export]
macro_rules! debug {
    ($val0:expr) => {
        use colored::Colorize;
        let a = format!("{:?}", $val0).cyan();
        println!("{}: {}", format!("[DEBUG]").green(), a);
    };
    ($val0:expr, $val1:expr) => {
        use colored::Colorize;
        let a = format!("{:?}", $val0).cyan();
        let b = format!("{:?}", $val1).yellow();
        println!("{}: {} | {}", format!("[DEBUG]").green(), a, b);
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
