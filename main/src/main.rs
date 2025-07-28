use std::process::Command;

fn main() {
    let path = format!(
        "{}/../cpu/target/debug/rusty-vm_cpu",
        env!("CARGO_MANIFEST_DIR")
    );

    #[cfg(not(target_os = "android"))] {
        #[cfg(debug_assertions)]
        debug!("Getting external process from: ", path);
        Some(Command::new(path).spawn().unwrap());
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
