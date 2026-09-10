pub struct OutputBuffer {
    data: Vec<u8>,
    capacity: usize,
}

impl OutputBuffer {
    pub fn new(capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
            capacity,
        }
    }

    pub fn push(&mut self, bytes: &[u8]) {
        self.data.extend_from_slice(bytes);
        if self.data.len() > self.capacity {
            let overflow = self.data.len() - self.capacity;
            self.data.drain(0..overflow);
        }
    }

    pub fn take(&mut self) -> Vec<u8> {
        std::mem::take(&mut self.data)
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }
}

impl Default for OutputBuffer {
    fn default() -> Self {
        Self::new(1 << 20)
    }
}
