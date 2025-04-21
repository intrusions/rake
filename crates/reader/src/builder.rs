use crate::reader;
use reader::Reader;
use std::fs::File;
use std::io::{BufRead, BufReader};

const LINE_MAX_SIZE: usize = 50;

pub enum ReaderBuilderError {
    FileNotSpecified,
    FileNotFound,
}

impl ReaderBuilderError {
    pub fn as_str(&self) -> &'static str {
        match self {
            ReaderBuilderError::FileNotSpecified => "No wordlist file is specified",
            ReaderBuilderError::FileNotFound => "Specified file not found",
        }
    }
}

pub struct ReaderBuilder {
    pub path: Option<String>,
    pub threads: u8,
}

impl ReaderBuilder {
    pub fn new() -> Self {
        Self {
            path: None,
            threads: 40,
        }
    }

    pub fn with_path(mut self, path: String) -> Self {
        self.path = Some(path);
        self
    }

    pub fn with_threads(mut self, threads: u8) -> Self {
        self.threads = threads;
        self
    }

    pub fn build(&self) -> Result<Reader, ReaderBuilderError> {
        let path = match &self.path {
            None => return Err(ReaderBuilderError::FileNotSpecified),
            Some(path) => path,
        };

        let file = File::open(&path).map_err(|_| ReaderBuilderError::FileNotFound)?;

        let line_count = Self::count_lines(&path)?;
        let chunk_size = std::cmp::max(1, line_count / self.threads as usize);

        let reader = Reader {
            reader: BufReader::with_capacity(chunk_size * LINE_MAX_SIZE, file),
            chunk_size,
            line_count,
        };

        Ok(reader)
    }

    fn count_lines(path: &str) -> Result<usize, ReaderBuilderError> {
        let file = File::open(path).map_err(|_| ReaderBuilderError::FileNotFound)?;
        let reader = BufReader::new(file);

        Ok(reader.lines().count())
    }
}
