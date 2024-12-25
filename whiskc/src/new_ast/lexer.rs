use std::io::Read;

#[derive(Debug)]
pub struct Lexer<R: Read> {
    rd: Reader<R>,
}
impl<R: Read> Lexer<R> {
    pub fn new(reader: R) -> Self {
        Self {
            rd: Reader::new(reader),
        }
    }
}

const BUFFER_SIZE: usize = 1024;

#[derive(Debug)]
struct Reader<R: Read> {
    rd: R,
    buf: [u8; BUFFER_SIZE],
    index: usize,
    eof: bool,
}
impl<R: Read> Reader<R> {
    pub fn new(reader: R) -> Self {
        Self {
            rd: reader,
            buf: [0; BUFFER_SIZE],
            index: BUFFER_SIZE,
            eof: false,
        }
    }

    pub fn is_eof(&self) -> bool {
        self.eof && self.index == self.buf.len()
    }

    fn fill_buffer(&mut self) -> Option<u8> {
        let req = self.index;
        for (to, from) in self
            .buf
            .iter_mut()
            .zip(self.buf.iter_mut().skip(self.index))
        {
            *to = from;
        }

        todo!()
    }
}
