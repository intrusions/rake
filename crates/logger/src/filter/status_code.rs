use reqwest::blocking::Response;
use crate::filter::ResponseFilter;

pub struct StatusCodeFilter {
    filtered_code: Vec<u16>,
    matched_code: Vec<u16>
}

impl StatusCodeFilter {
    pub fn new(filtered_code: Vec<u16>, matched_code: Vec<u16>) -> Self {
        Self { filtered_code, matched_code }
    }
}

impl ResponseFilter for StatusCodeFilter {
    fn should_filter(&self, response: &Response) -> bool {
        let code = &response.status().as_u16();

        if !self.matched_code.is_empty() {
            return !self.matched_code.contains(code);
        }

        self.filtered_code.contains(code)
    }
}
