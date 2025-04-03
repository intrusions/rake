use reqwest::blocking::Response;
use crate::filter::ResponseFilter;

pub struct ContentSizeFilter {
    excluded_sizes: Vec<u64>,
}

impl ContentSizeFilter {
    pub fn new(excluded_sizes: Vec<u64>) -> Self {
        Self { excluded_sizes }
    }
}

impl ResponseFilter for ContentSizeFilter {
    fn should_filter(&self, response: &Response) -> bool {
        let size = response.content_length().unwrap_or(0);
        self.excluded_sizes.contains(&size)
    }
}
