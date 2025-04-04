use crate::ReaderArgs;
use std::{
    fs::File,
    io::{self, ErrorKind, BufRead, BufReader}
};

#[derive(Debug)]
pub enum ReaderError {
    FileNotFound,
    EmptyChunk,
    NonUTF8,
    Io(io::Error)
}

impl ReaderError {
    pub fn as_str(&self) -> &'static str {
        match self {
            ReaderError::FileNotFound => "Wordlist file not found",
            ReaderError::EmptyChunk => "Empty chunk",
            ReaderError::NonUTF8 => "Invalid non UTF8 character present in the file",
            ReaderError::Io(_) => "IO Error"
        }
    }
}

impl From<io::Error> for ReaderError {
    fn from(err: io::Error) -> Self {
        if err.kind() == ErrorKind::NotFound {
            ReaderError::FileNotFound
        } else {
            ReaderError::Io(err)
        }
    }
}

pub struct Reader {
    reader: BufReader<File>,
    chunk_size: usize,
}

const CHAR_PER_LINE: usize = 50;

impl Reader {
    pub fn new(args: ReaderArgs) -> Result<Self, ReaderError> {
        let file = File::open(&args.path)?;
        let line_count = Self::count_lines(&args.path)?;
        let chunk_size = std::cmp::max(1, line_count / args.threads as usize);
    
        let reader = Self {
            reader: BufReader::with_capacity(chunk_size * CHAR_PER_LINE, file),
            chunk_size
        };
    
        Ok(reader)
    }

    pub fn get_next_chunk(&mut self) -> Result<Vec<String>, ReaderError> {
        let mut line = String::new();
        let mut chunk = Vec::new();
    
        for _ in 0..self.chunk_size {
            line.clear();
            
            let size = match self.reader.read_line(&mut line) {
                Ok(size) => size,
                Err(_) => break
            };
            
            if size == 0 {
                break;
            }

            chunk.push(line.to_string());
        }
        
        match !chunk.is_empty() {
            true => Ok(chunk),
            false => Err(ReaderError::EmptyChunk)
        }
    }
    
    fn count_lines<P: AsRef<std::path::Path>>(path: P) -> Result<usize, ReaderError> {
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        
        Ok(reader.lines().count())
    }
}
