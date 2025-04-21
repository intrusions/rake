use crate::display::filter::ResponseFilter;
use reqwest::blocking::Response;

pub struct ContentSizeFilter {
    filtered_size: Vec<u64>,
    matched_size: Vec<u64>,
}

impl ContentSizeFilter {
    pub fn new(filtered_size: Vec<u64>, matched_size: Vec<u64>) -> Self {
        Self {
            filtered_size,
            matched_size,
        }
    }
}

impl ResponseFilter for ContentSizeFilter {
    fn should_filter(&self, response: &Response) -> bool {
        let size = response.content_length().unwrap_or(0);

        if !self.matched_size.is_empty() {
            return !self.matched_size.contains(&size);
        }

        self.filtered_size.contains(&size)
    }
}
