use crate::opcodes::{self, *};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, Read, Seek, SeekFrom, Write};

const FONT_SIZE: f32 = 16.0;

#[derive(Debug)]
pub struct GPU {
    pub buf_ptr: u16,
    pub memory: String,
    pub frame_buffer: [[Character; 40]; 63],
    pub cursor: Cursor,
    pub draw_mode: bool,
    pub clock_speed: usize,
    pub counter: usize,
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
            buf_ptr: 0x0300, // 0x0300 - 0x04FF => 768 - 1279, so 512 16-bit addresses
            memory: buffer.to_string(),
            frame_buffer: [[Character::new(' '); 40]; 63],
            cursor: Cursor::init(),
            draw_mode: false,
            clock_speed: 10, // In Hz
            counter: 0,
        }
    }

    pub fn increase_buf_ptr(&mut self) {
        if self.buf_ptr + 1 > 0x04FF {
            self.buf_ptr = 0x0300;
        } else {
            self.buf_ptr += 1;
        }
    }

    pub async fn draw_framebuffer(&mut self) {
        macroquad::text::draw_text(
            &format!("{}", "_") as &str,
            self.cursor.position.0 as f32 * 8.0,
            self.cursor.position.1 as f32 * 11.0 + 10.0,
            FONT_SIZE,
            macroquad::color::WHITE
        );
        for y in 0..40 {
            for x in 0..63 {
                macroquad::text::draw_text(
                    &format!("{}", self.frame_buffer[x][y].literal) as &str,
                    x as f32 * 8.0,
                    y as f32 * 11.0 + 10.0,
                    FONT_SIZE,
                    self.frame_buffer[x][y].color
                );
            }
        }
        macroquad::window::next_frame().await;
    }

    pub async fn update(&mut self) {

        self.counter += 1;

        if self.counter == 99 {
            if macroquad::input::is_key_pressed(macroquad::input::KeyCode::Escape) {
                std::process::exit(0);
            }
            self.draw_framebuffer().await;
            self.counter = 0;
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

            #[cfg(debug_assertions)]
            crate::debug!(
                format!("Address: {:#06X}", self.buf_ptr),
                format!("Instruction: {:#06X}", instruction)
            );

            // --- Handle GPU Instructions ---
            if self.draw_mode == true {
                #[cfg(debug_assertions)]
                crate::debug!(
                    "Appending letter to framebuffer: ",
                    crate::hex!(instruction)
                );

                // --- Handle chars ---
                match char::from(instruction as u8) {
                    '`' => {
                        #[cfg(debug_assertions)]
                        crate::debug!("Detected escape character: Exiting draw mode");
                        self.draw_mode = false;
                        self.increase_buf_ptr();
                    },
                    '\n' => {
                        self.cursor.position.0 = 0;
                        self.cursor.position.1 += 1;
                        self.increase_buf_ptr();
                    },
                    _ => {
                        self.frame_buffer[self.cursor.position.0][self.cursor.position.1] =
                            Character::new(char::from(instruction as u8));
                        if self.cursor.position.0 < 62 {
                            self.cursor.position.0 += 1;
                        } else {
                            self.cursor.position.0 = 0;
                            self.cursor.position.1 += 1;
                        }
                        self.increase_buf_ptr();
                    }
                }
            } else {
                match instruction {
                    opcodes::GPU_NO_OPERAT => {
                        #[cfg(debug_assertions)]
                        crate::debug!("NoOp");
                    },
                    opcodes::GPU_DRAW_LETT => {
                        #[cfg(debug_assertions)]
                        crate::debug!("Entering draw mode");
                        self.increase_buf_ptr();
                        self.draw_mode = true;
                    },
                    opcodes::GPU_RESET_PTR => {
                        #[cfg(debug_assertions)]
                        crate::debug!("Resetting buffer pointer");
                        self.buf_ptr = 0x0300;
                    },
                    opcodes::GPU_UPDATE => {
                        #[cfg(debug_assertions)]
                        crate::debug!("Redrawing the screen");
                        // --- Render FrameBuffer ---
                        self.draw_framebuffer().await;
                        self.increase_buf_ptr();
                    },
                    opcodes::GPU_RES_F_BUF => {
                        #[cfg(debug_assertions)]
                        crate::debug!("Clearing frame buffer");
                        for y in 0..40 {
                            for x in 0..63 {
                                self.frame_buffer[x][y].literal = ' ';
                                self.frame_buffer[x][y].color = macroquad::color::BLACK;
                            }
                        }
                        self.increase_buf_ptr();
                    },
                    opcodes::GPU_MV_C_DOWN => {
                        #[cfg(debug_assertions)]
                        crate::debug!("Moving cursor down");
                        self.cursor.position.1 += 1;
                        self.increase_buf_ptr();
                    },
                    opcodes::GPU_NEW_LINE => {
                        #[cfg(debug_assertions)]
                        crate::debug!("Inserting new line");
                        self.cursor.position.1 += 1;
                        self.cursor.position.0 = 0;
                        self.increase_buf_ptr();
                    },
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
    pub color:   macroquad::color::Color,
}

impl Character {
    pub fn new(char: char) -> Self {
        Self {
            literal: char,
            color: macroquad::color::WHITE
        }
    }
}

#[derive(Debug)]
pub struct Cursor {
    pub position: (usize, usize),
    pub shape: CursorShapes,
}

impl Cursor {
    pub fn init() -> Self {
        Self {
            position: Default::default(),
            shape: CursorShapes::Block,
        }
    }
}

#[derive(Debug)]
pub enum CursorShapes {
    Block,
    Underline,
    VertiBar,
}
