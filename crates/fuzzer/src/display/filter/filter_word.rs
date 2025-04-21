use crate::display::filter::ResponseFilter;

pub struct WordFilter {
    filtered_words: Vec<String>,
    matched_words: Vec<String>,
}

impl WordFilter {
    pub fn new(filtered_words: Vec<String>, matched_words: Vec<String>) -> Self {
        Self {
            filtered_words,
            matched_words,
        }
    }
}

impl ResponseFilter for WordFilter {
    fn should_filter(&self, _status: u16, _size: u64, body: &str) -> bool {
        if !self.matched_words.is_empty() {
            return !self.matched_words.iter().any(|w| body.contains(w));
        }

        self.filtered_words.iter().any(|w| body.contains(w))
    }
}
