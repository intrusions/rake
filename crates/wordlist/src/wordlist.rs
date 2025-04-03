use crate::WordlistArgs;
use std::{
    fs::File,
    io::{ErrorKind, BufRead, BufReader, Read, Error}
};

const CHUNK_SIZE: usize = 500;

pub enum WordlistError {
    FileNotFound,
}

impl WordlistError {
    pub fn as_str(&self) -> &'static str {
        match self {
            WordlistError::FileNotFound => "Wordlist file not found",
        }
    }
}

#[allow(dead_code)]
pub struct Wordlist {
    reader: BufReader<File>,
    pub chunk: Vec<String>,
    args: WordlistArgs
}

impl Wordlist {
    pub fn new(args: WordlistArgs) -> Result<Self, WordlistError> {
        let args = WordlistArgs {
            path: args.path.clone()
        };

        let file = File::open(&args.path)
            .map_err(|err| match err.kind() {
                ErrorKind::NotFound => WordlistError::FileNotFound,
                _ => panic!("io error")
        })?;
        
        Ok(Self {
            reader: BufReader::with_capacity(CHUNK_SIZE, file),
            chunk: Vec::new(),
            args
        })
    }

    pub fn load_next_chunk(&mut self) -> Result<(), Error> {
        self.chunk.clear();

        let lines = self.reader.by_ref().lines();
        for line in lines.take(CHUNK_SIZE) {
            self.chunk.push(line?);
        }

        Ok(())
    }
}
