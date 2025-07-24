use crate::FONT_SIZE;
use crate::opcodes::{self, *};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, Read, Seek, SeekFrom, Write};

#[derive(Debug)]
pub struct GPU {
    pub buf_ptr: u16,
    pub memory: String,
    pub frame_buffer: [[char; 40]; 63],
    pub cursor: Cursor,
    pub draw_mode: bool,
    pub clock_speed: usize,
}

impl GPU {
    pub fn init() -> Self {
        let mut img = OpenOptions::new()
            .read(true)
            .open(format!("{}/../memory.img", env!("CARGO_MANIFEST_DIR")))
            .expect("Memory image missing");

        let mut file = img;
        let mut buffer = &mut String::new();
        _ = file.read_to_string(&mut buffer);

        Self {
            buf_ptr: 0x0300, // 0x0300 - 0x04FF => 768 - 1279, so 512 16-bit addresses
            memory: buffer.to_string(),
            frame_buffer: [[' '; 40]; 63],
            cursor: Cursor::init(),
            draw_mode: false,
            clock_speed: 4, // In Hz
        }
    }

    pub fn increase_buf_ptr(&mut self) {
        if self.buf_ptr + 1 > 0x04FF {
            self.buf_ptr = 0x0300;
        } else {
            self.buf_ptr += 1;
        }
    }

    pub fn update(&mut self) {
        if self.memory.lines().nth(self.buf_ptr as usize).is_some() {

            let instruction_string = self
                .memory
                .lines()
                .nth(self.buf_ptr as usize)
                .unwrap()
                .to_string();

            let trimmed = instruction_string.trim_start_matches("0x");

            let instruction = u16::from_str_radix(trimmed, 16).unwrap();

            #[cfg(debug_assertions)]
            crate::debug!(format!(
                "Address: {:#06X} | Instruction: {:#06X}",
                self.buf_ptr, instruction
            ));

            // --- Handle GPU Instructions ---
            if self.draw_mode == true {
                #[cfg(debug_assertions)]
                crate::debug!(
                    "Appending letter to framebuffer: ",
                    crate::hex!(instruction)
                );
                if char::from(instruction as u8) != '`' {
                    self.frame_buffer[self.cursor.position.0][self.cursor.position.1] =
                        char::from(instruction as u8);
                    if self.cursor.position.0 < 62 {
                        self.cursor.position.0 += 1;
                    } else {
                        self.cursor.position.0 = 0;
                        self.cursor.position.1 += 1;
                    }
                    self.increase_buf_ptr();
                } else {
                    #[cfg(debug_assertions)]
                    crate::debug!("Detected escape character: Exiting draw mode");
                    self.draw_mode = false;
                }
            } else {
                match instruction {
                    opcodes::GPU_NO_OPERAT => {
                        #[cfg(debug_assertions)]
                        crate::debug!("NoOp");
                    }
                    opcodes::GPU_DRAW_LETT => {
                        #[cfg(debug_assertions)]
                        crate::debug!("Entering draw mode");
                        self.increase_buf_ptr();
                        self.draw_mode = true;
                    }
                    _ => {}
                }
            }

            // --- Render FrameBuffer ---
            for y in 0..40 {
                for x in 0..63 {
                    macroquad::text::draw_text(
                        &format!("{}", self.frame_buffer[x][y]) as &str,
                        x as f32 * 11.0,
                        y as f32 * 13.0 + 12.0,
                        FONT_SIZE,
                        macroquad::color::WHITE
                    );
                }
            }
        }
        std::thread::sleep(std::time::Duration::from_micros(
            1_000_000 / self.clock_speed as u64,
        ));
    }
}

pub struct Line {
    text: String,
    offset: f32,
}

impl Line {
    pub fn from(text: &str) -> Self {
        Self {
            text: text.to_string(),
            offset: 17.0,
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
