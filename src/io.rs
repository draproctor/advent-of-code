use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

pub trait LineExtractor {
    fn lines(&self) -> Lines<BufReader<&File>>;
}

impl LineExtractor for File {
    /// Return an iterator for reading the lines of a given file.
    fn lines(&self) -> Lines<BufReader<&File>> {
        BufReader::new(self).lines()
    }
}
