use crate::ReaderArgs;
use std::{
    fs::File,
    io::{self, ErrorKind, BufRead, BufReader, Read}
};

const DEFAULT_CHUNK_SIZE: usize = 500;

#[derive(Debug)]
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
        Ok(Self {
            reader: BufReader::with_capacity(DEFAULT_CHUNK_SIZE * 100, file),
            chunk: Vec::new(),
            chunk_size: DEFAULT_CHUNK_SIZE,
            args,
        })
    }

    pub fn load_next_chunk(&mut self) -> Result<(), ReaderError> {
        self.chunk.clear();

        let lines = self.reader.by_ref().lines();
        for line in lines.take(self.chunk_size) {
            self.chunk.push(line?);
        }

        Ok(())
    }

    pub fn is_finished(&self) -> bool {
        self.chunk.is_empty()
    }

    pub fn get(&self, index: usize) -> Option<&str> {
        self.chunk.get(index).map(String::as_str)
    }

    pub fn current_chunk_len(&self) -> usize {
        self.chunk.len()
    }
}
