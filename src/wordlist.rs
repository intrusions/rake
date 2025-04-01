use std::{
    fs::File,
    io::{BufRead, BufReader, Read, Error},
};

/// Maximum number of lines loaded into memory per chunk.
const CHUNK_SIZE: usize = 500;

/// A wordlist reader that loads a file line by line in chunks.
pub struct Wordlist {
    /// Buffered reader for efficient file reading.
    pub reader: BufReader<File>,
    /// Stores the current chunk of lines.
    pub chunk: Vec<String>,
}

impl Wordlist {
    /// Creates a new `Wordlist` instance.
    ///
    /// # Arguments
    /// * `path` - Path to the file.
    ///
    /// # Returns
    /// * `Ok(Self)` - On success.
    /// * `Err(e)` - If the file cannot be opened.
    pub fn new(path: &str) -> Result<Self, Error> {
        let file = File::open(path)?;
        Ok(Self {
            reader: BufReader::with_capacity(CHUNK_SIZE, file),
            chunk: Vec::new(),
        })
    }

    /// Loads the next chunk of lines into `chunk`.
    ///
    /// Clears the previous chunk and reads up to `CHUNK_SIZE` lines.
    ///
    /// # Returns
    /// * `Ok(())` - On success.
    /// * `Err(e)` - If reading fails.
    pub fn load_next_chunk(&mut self) -> Result<(), Error> {
        self.chunk.clear();

        let lines = self.reader.by_ref().lines();
        for line in lines.take(CHUNK_SIZE) {
            self.chunk.push(line?);
        }

        Ok(())
    }
}
