use std::{
    fs::File,
    io::{BufRead, BufReader},
};

pub enum ReaderError {
    EmptyChunk,
}

impl ReaderError {
    pub fn as_str(&self) -> &'static str {
        match self {
            ReaderError::EmptyChunk => "Empty chunk",
        }
    }
}

pub struct Reader {
    pub reader: BufReader<File>,
    pub chunk_size: usize,
    pub line_count: usize,
}

impl Reader {
    pub fn get_next_chunk(&mut self) -> Result<Vec<String>, ReaderError> {
        let mut line = String::new();
        let mut chunk = Vec::new();

        for _ in 0..self.chunk_size {
            line.clear();

            match self.reader.read_line(&mut line) {
                Ok(0) => break,
                Ok(_) => chunk.push(line.to_string()),
                Err(_) => break,
            }
        }

        match !chunk.is_empty() {
            true => Ok(chunk),
            false => Err(ReaderError::EmptyChunk),
        }
    }
}
