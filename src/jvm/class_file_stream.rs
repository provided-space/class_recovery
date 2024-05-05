use std::ops::Add;

pub struct ClassFileStream<'a> {
    buffer: &'a Vec<u8>,
    current: usize,
}

impl<'a> ClassFileStream<'a> {
    pub fn new(buffer: &'a Vec<u8>, current: usize) -> Self {
        return ClassFileStream { buffer, current };
    }

    pub fn get_current(&self) -> usize {
        return self.current;
    }

    pub fn get_u1(&mut self) -> Option<u8> {
        if self.buffer.len() <= self.current {
            return None;
        }
        let result = self.buffer[self.current];
        self.current += 1;
        return Some(result);
    }

    pub fn get_u2(&mut self) -> Option<u16> {
        if self.buffer.len() <= self.current + 1 {
            return None;
        }
        let result = u16::from_be_bytes([
            self.buffer[self.current + 0],
            self.buffer[self.current + 1]
        ]);
        self.current += 2;
        return Some(result);
    }

    pub fn get_u4(&mut self) -> Option<u32> {
        if self.buffer.len() <= self.current + 3 {
            return None;
        }
        let result = u32::from_be_bytes([
            self.buffer[self.current + 0],
            self.buffer[self.current + 1],
            self.buffer[self.current + 2],
            self.buffer[self.current + 3],
        ]);
        self.current += 4;
        return Some(result);
    }

    pub fn get_u8(&mut self) -> Option<u64> {
        if self.buffer.len() <= self.current + 7 {
            return None;
        }
        let result = u64::from_be_bytes([
            self.buffer[self.current + 0],
            self.buffer[self.current + 1],
            self.buffer[self.current + 2],
            self.buffer[self.current + 3],
            self.buffer[self.current + 4],
            self.buffer[self.current + 5],
            self.buffer[self.current + 6],
            self.buffer[self.current + 7],
        ]);
        self.current += 8;
        return Some(result);
    }

    pub fn skip_u1(&mut self, len: usize) -> Option<()> {
        if self.buffer.len() <= self.current + len {
            return None;
        }
        self.current = self.current.add(len);
        return Some(());
    }

    pub fn skip_u2(&mut self, len: usize) -> Option<()> {
        if self.buffer.len() <= self.current + 2 * len {
            return None;
        }
        self.current = self.current.add(2 * len);
        return Some(());
    }

    pub fn skip_u4(&mut self, len: usize) -> Option<()> {
        if self.buffer.len() <= self.current + 4 * len {
            return None;
        }
        self.current = self.current.add(4 * len);
        return Some(());
    }
}
