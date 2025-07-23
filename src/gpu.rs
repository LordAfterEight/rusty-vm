pub struct GPU {
    buffer: u8
}

impl GPU {
    pub fn init() -> Self {
        Self {
            buffer: Default::default()
        }
    }

    pub fn update(&mut self, memory: &crate::memory::Memory) {
    }
}
