pub struct OutputBuffer {
    data: Vec<u8>,
}

impl OutputBuffer {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn push(&mut self, bytes: &[u8]) {
        self.data.extend_from_slice(bytes);
    }

    pub fn take(&mut self) -> Vec<u8> {
        std::mem::take(&mut self.data)
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}
