use std::{
    fs::File,
    io::{
        BufRead,
        BufReader,
        Read,
        Error
    }
};

const CHUNK_SIZE: usize = 500;

pub struct Wordlist {
    pub reader: BufReader<File>,
    pub chunk: Vec<String>,
}

impl Wordlist {
    pub fn new(path: &str) -> Result<Self, Error> {
        let file = File::open(path)?;
        
        Ok(Self {
            reader: BufReader::with_capacity(CHUNK_SIZE, file),
            chunk: Vec::new(),
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
