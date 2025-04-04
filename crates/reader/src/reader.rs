use crate::ReaderArgs;
use std::{
    fs::File,
    io::{self, ErrorKind, BufRead, BufReader}
};

pub enum ReaderError {
    FileNotFound,
    Io(io::Error)
}

impl ReaderError {
    pub fn as_str(&self) -> &'static str {
        match self {
            ReaderError::FileNotFound => "Wordlist file not found",
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

#[allow(dead_code)]
pub struct Reader {
    reader: BufReader<File>,
    pub chunk: Vec<String>,
    args: ReaderArgs,
    chunk_size: usize,
}

impl Reader {
    pub fn new(args: ReaderArgs) -> Result<Self, ReaderError> {
        let file = File::open(&args.path)?;
        let line_count = Self::count_lines(&args.path)?;
        let chunk_size = std::cmp::max(1, line_count / args.threads as usize);
    
        let reader = Self {
            reader: BufReader::with_capacity(chunk_size * 50, file),
            chunk: Vec::new(),
            chunk_size,
            args,
        };
    
        Ok(reader)
    }

    pub fn load_next_chunk(&mut self) -> Result<bool, ReaderError> {
        let mut line = String::new();
        let mut chunk = Vec::new();
    
        for _ in 0..self.chunk_size {
            line.clear();
            
            let bytes = self.reader.read_line(&mut line)?;
            if bytes == 0 {
                break;
            }
            
            chunk.push(line.to_string());
        }
    
        self.chunk = chunk;
        Ok(!self.chunk.is_empty())
    }
    
    fn count_lines<P: AsRef<std::path::Path>>(path: P) -> Result<usize, ReaderError> {
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        Ok(reader.lines().count())
    }

    pub fn is_finished(&self) -> bool {
        self.chunk.is_empty()
    }

    pub fn get_by_index(&self, index: usize) -> Option<&str> {
        self.chunk.get(index).map(String::as_str)
    }

    pub fn current_chunk_len(&self) -> usize {
        self.chunk.len()
    }
}
