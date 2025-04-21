use crate::display::filter::ResponseFilter;

pub struct StatusCodeFilter {
    filtered_code: Vec<u16>,
    matched_code: Vec<u16>,
}

impl StatusCodeFilter {
    pub fn new(filtered_code: Vec<u16>, matched_code: Vec<u16>) -> Self {
        Self {
            filtered_code,
            matched_code,
        }
    }
}

impl ResponseFilter for StatusCodeFilter {
    fn should_filter(&self, status: u16, _size: u64, _body: &str) -> bool {
        if !self.matched_code.is_empty() {
            return !self.matched_code.contains(&status);
        }

        self.filtered_code.contains(&status)
    }
}
