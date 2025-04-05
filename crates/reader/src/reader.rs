use crate::ReaderArgs;
use std::{
    fs::File,
    io::{BufRead, BufReader}
};

#[derive(Debug)]
pub enum ReaderError {
    FileNotFound,
    EmptyChunk,
    NonUTF8,
}

impl ReaderError {
    pub fn as_str(&self) -> &'static str {
        match self {
            ReaderError::FileNotFound => "Wordlist file not found",
            ReaderError::EmptyChunk => "Empty chunk",
            ReaderError::NonUTF8 => "Invalid non UTF8 character present in the file",
        }
    }
}

pub struct Reader {
    reader: BufReader<File>,
    chunk_size: usize,
    pub line_count: usize
}

const LINE_MAX_SIZE: usize = 50;

impl Reader {
    pub fn new(args: ReaderArgs) -> Result<Self, ReaderError> {
        let file = File::open(&args.path).map_err(|_| ReaderError::FileNotFound)?;
        let line_count = Self::count_lines(&args.path)?;
        let chunk_size = std::cmp::max(1, line_count / args.threads as usize);
    
        let reader = Self {
            reader: BufReader::with_capacity(chunk_size * LINE_MAX_SIZE, file),
            chunk_size,
            line_count
        };
    
        Ok(reader)
    }

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
            false => Err(ReaderError::EmptyChunk)
        }
    }
    
    fn count_lines(path: &str) -> Result<usize, ReaderError> {
        let file = File::open(path).map_err(|_| ReaderError::FileNotFound)?;
        let reader = BufReader::new(file);
        
        Ok(reader.lines().count())
    }
}
