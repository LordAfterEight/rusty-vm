use crate::opcodes;
use std::fs::OpenOptions;
use std::io::Read;

use std::env;

#[cfg(target_os = "linux")]
use nix::{
    sys::signal::{Signal, kill},
    unistd::Pid,
};

const FONT_SIZE: f32 = 16.0;

#[derive(Debug)]
pub struct GPU {
    pub buf_ptr: u16,
    pub memory: String,
    pub frame_buffer: [[Character; 48]; 92],
    pub cursor: Cursor,
    pub cursor_visible: bool,
    pub draw_mode: bool,
    pub draw_color: macroquad::color::Color,
    pub clock_speed: usize,
    pub pri_counter: usize,
    pub sec_counter: usize,
    pub int_flag: bool,
}


impl GPU {
    pub fn init() -> Self {
        let img = OpenOptions::new()
            .read(true)
            .open(format!("{}/../ROM", env!("CARGO_MANIFEST_DIR")))
            .expect("Memory image missing");

        let mut file = img;
        let mut buffer = &mut String::new();
        _ = file.read_to_string(&mut buffer);

        Self {
            buf_ptr: 0x0300, // 0x0300 - 0x0FFF => 768 - 4096, so 3328 16-bit addresses
            memory: buffer.to_string(),
            frame_buffer: [[Character::new(' '); 48]; 92],
            cursor: Cursor::new(CursorShapes::Block),
            cursor_visible: false,
            draw_mode: false,
            draw_color: macroquad::color::WHITE,
            clock_speed: 10_000, // In Hz
            pri_counter: 0,
            sec_counter: 0,
            int_flag: false,
        }
    }

    pub fn increase_buf_ptr(&mut self) {
        if self.buf_ptr + 1 > 0x0FFF {
            self.buf_ptr = 0x0300;
        } else {
            self.buf_ptr += 1;
        }
    }

    pub async fn draw_framebuffer(&mut self) {
        if self.cursor_visible {
            let mut cursor = "_";
            match self.cursor.shape {
                CursorShapes::Block => cursor = "█",
                CursorShapes::VertiBar => cursor = "|",
                _ => {}
            }
            macroquad::text::draw_text(
                cursor,
                self.cursor.position.0 as f32 * 7.0 + 2.0,
                self.cursor.position.1 as f32 * 12.0 + 10.0,
                FONT_SIZE,
                self.draw_color
            );
        }
        for y in 0..40 {
            for x in 0..91 {
                macroquad::text::draw_text(
                    &format!("{}", self.frame_buffer[x][y].literal) as &str,
                    x as f32 * 7.0 + 2.0,
                    y as f32 * 12.0 + 10.0,
                    FONT_SIZE,
                    self.frame_buffer[x][y].color,
                );
            }
        }
        macroquad::window::next_frame().await;
    }

    pub async fn update(&mut self) {
        self.pri_counter += 1;

        if macroquad::input::is_quit_requested() {
            let args: Vec<String> = env::args().collect();

            #[cfg(target_os = "linux")]
            let parent_pid: i32 = args[1].parse().expect("Invalid PID");

            #[cfg(target_os = "windows")]
            let parent_pid: i32 = &args[1];

            #[cfg(target_os = "linux")]
            kill(Pid::from_raw(parent_pid), Signal::SIGKILL).expect("Failed to kill parent");

            #[cfg(target_os = "windows")]
            Command::new("taskkill")
                .args(&["/PID", parent_pid, "/F"])
                .spawn()
                .expect("Failed to kill parent process");
            std::process::exit(0);
        }

        if self.pri_counter == 99 {
            if macroquad::input::is_key_down(macroquad::input::KeyCode::Escape) {
                let args: Vec<String> = env::args().collect();

                #[cfg(target_os = "linux")]
                let parent_pid: i32 = args[1].parse().expect("Invalid PID");

                #[cfg(target_os = "windows")]
                let parent_pid: i32 = &args[1];

                #[cfg(target_os = "linux")]
                kill(Pid::from_raw(parent_pid), Signal::SIGKILL).expect("Failed to kill parent");

                #[cfg(target_os = "windows")]
                Command::new("taskkill")
                    .args(&["/PID", parent_pid, "/F"])
                    .spawn()
                    .expect("Failed to kill parent process");

                std::process::exit(0);
            }
            self.draw_framebuffer().await;
            self.sec_counter += 1;
            if self.sec_counter == 10 {
                self.cursor_visible = !self.cursor_visible;
                self.sec_counter = 0;
            }
            self.pri_counter = 0;
        }

        let img = OpenOptions::new()
            .read(true)
            .open(format!("{}/../ROM", env!("CARGO_MANIFEST_DIR")))
            .expect("Memory image missing");

        let mut file = img;
        let mut buffer = &mut String::new();
        _ = file.read_to_string(&mut buffer);

        self.memory = buffer.to_string();

        if self.memory.lines().nth(self.buf_ptr as usize).is_some() {
            let instruction_string = self
                .memory
                .lines()
                .nth(self.buf_ptr as usize)
                .unwrap()
                .to_string();

            let trimmed = instruction_string.trim_start_matches("");

            let instruction = u16::from_str_radix(trimmed, 2).unwrap();

            /*
            #[cfg(debug_assertions)]
            crate::debug!(
                format!("Address: {:#06X}", self.buf_ptr),
                format!("Instruction: {:#06X}", instruction)
            );
            */

            // --- Handle GPU Instructions ---
            if self.draw_mode == true {
                #[cfg(debug_assertions)]
                crate::debug!(
                    "Appending letter to framebuffer: ",
                    crate::hex!(instruction)
                );

                let instr = instruction as u8;

                if self.int_flag {

                }

                // --- Handle chars ---
                match char::from(instr) {
                    '`' => {
                        #[cfg(debug_assertions)]
                        crate::debug!("Detected escape character: Exiting draw mode");
                        self.draw_mode = false;
                        self.increase_buf_ptr();
                    }
                    _ => match instruction {
                        0xA000 => {},
                        0x00..=0xFF7A => {
                            let mut char = Character::new(char::from(instruction as u8));

                            let color_byte = (instruction >> 8) as u8;
                            let char_byte = (instruction & 0xFF) as u8;

                            #[cfg(debug_assertions)]
                            crate::debug!(
                                format!("Color Byte: {:#04X}", color_byte),
                                format!("Character Byte: {:#04X}", char_byte)
                            );

                            match CharColors::from_u8(color_byte).unwrap() {
                                CharColors::White => char.color = macroquad::color::WHITE,
                                CharColors::Red => char.color = macroquad::color::RED,
                                CharColors::Green => char.color = macroquad::color::GREEN,
                                CharColors::Blue => char.color = macroquad::color::BLUE,
                                CharColors::Cyan => char.color = macroquad::color::Color::new(0.5,0.9,1.0,1.0),
                                CharColors::Magenta => char.color = macroquad::color::MAGENTA,
                            }

                            self.frame_buffer[self.cursor.position.0][self.cursor.position.1] = char;
                            if self.cursor.position.0 < 91 {
                                self.cursor.position.0 += 1;
                            } else {
                                self.cursor.position.0 = 0;
                                if self.cursor.position.1 < 48 {
                                    self.cursor.position.1 += 1;
                                } else {
                                    self.cursor.position.1 = 0;
                                }
                            }
                            self.increase_buf_ptr();
                        }
                        _ => {}
                    },
                }
            } else {
                match instruction {
                    opcodes::GPU_NO_OPERAT => {
                        /*
                        #[cfg(debug_assertions)]
                        crate::debug!("NoOp");
                        */
                    }
                    opcodes::GPU_DRAW_LETT => {
                        #[cfg(debug_assertions)]
                        crate::debug!("Entering draw mode");
                        self.increase_buf_ptr();
                        self.draw_mode = true;
                    }
                    opcodes::GPU_DRAW_VALU => {
                        #[cfg(debug_assertions)]
                        crate::debug!("Entering draw mode");
                        self.increase_buf_ptr();
                        self.draw_mode = true;
                        self.int_flag = true;
                    }
                    opcodes::GPU_RESET_PTR => {
                        #[cfg(debug_assertions)]
                        crate::debug!("Resetting buffer pointer");
                        self.buf_ptr = 0x0300;
                    }
                    opcodes::GPU_UPDATE => {
                        #[cfg(debug_assertions)]
                        crate::debug!("Redrawing the screen");
                        // --- Render FrameBuffer ---
                        self.draw_framebuffer().await;
                        self.increase_buf_ptr();
                    }
                    opcodes::GPU_RES_F_BUF => {
                        #[cfg(debug_assertions)]
                        crate::debug!("Clearing frame buffer");
                        for y in 0..48 {
                            for x in 0..91 {
                                self.frame_buffer[x][y].literal = ' ';
                                self.frame_buffer[x][y].color = macroquad::color::BLACK;
                            }
                        }
                        self.cursor.position = (0,0);
                        self.increase_buf_ptr();
                    }
                    opcodes::GPU_MV_C_DOWN => {
                        #[cfg(debug_assertions)]
                        crate::debug!("Moving cursor down");
                        self.cursor.position.1 += 1;
                        self.increase_buf_ptr();
                    }
                    opcodes::GPU_NEW_LINE => {
                        #[cfg(debug_assertions)]
                        crate::debug!("Inserting new line");
                        self.cursor.position.1 += 1;
                        self.cursor.position.0 = 0;
                        self.increase_buf_ptr();
                    }
                    _ => {}
                }
            }
        }
        std::thread::sleep(std::time::Duration::from_micros(
            1_000_000 / self.clock_speed as u64,
        ));
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Character {
    pub literal: char,
    pub color: macroquad::color::Color,
}

impl Character {
    pub fn new(char: char) -> Self {
        Self {
            literal: char,
            color: macroquad::color::WHITE,
        }
    }
}

#[derive(Debug)]
pub struct Cursor {
    pub position: (usize, usize),
    pub shape: CursorShapes,
}

impl Cursor {
    pub fn new(shape: CursorShapes) -> Self {
        Self {
            position: Default::default(),
            shape
        }
    }
}

#[derive(Debug)]
pub enum CursorShapes {
    Block,
    Underline,
    VertiBar,
}

#[repr(u8)]
pub enum CharColors {
    White = 0x0A,
    Red = 0x0B,
    Green = 0x0C,
    Blue = 0x0D,
    Cyan = 0x0E,
    Magenta = 0x0F
}

impl CharColors {
    fn from_u8(value: u8) -> Option<Self> {
        match value {
            0x00 => Some(CharColors::White),
            0x0A => Some(CharColors::White),
            0x0B => Some(CharColors::Red),
            0x0C => Some(CharColors::Green),
            0x0D => Some(CharColors::Blue),
            0x0E => Some(CharColors::Cyan),
            0x0F => Some(CharColors::Magenta),
            _ => Some(CharColors::White),
        }
    }
}
