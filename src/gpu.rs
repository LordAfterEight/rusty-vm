use crate::opcodes::*;

pub struct GPU {
    pub buffer: u16,
    pub buf_ptr: u16,
    pub frame: Frame,
}

impl GPU {
    pub fn init() -> Self {
        Self {
            buffer: Default::default(),
            buf_ptr: 0x0300, // 0x0300 - 0x04FF => 768 - 1279, so 512 16-bit addresses
            frame: Frame::init()
        }
    }

    pub fn update(&mut self, memory: &crate::memory::Memory) {
        let instruction = memory.memory[self.buf_ptr as usize];

        match instruction {
            GPU_NO_OPERAT => {},
            _ => {}
        }
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

#[derive(Default)]
pub struct Frame {
    pub size_x: usize,
    pub size_y: usize,
    pub scale: usize,
    pub cursor_pos: (f32,f32),
}

impl Frame {
    pub fn init() -> Self {
        Self {
            size_x: 640,
            size_y: 480,
            ..Default::default()
        }
    }
}
