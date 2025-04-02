use reqwest::blocking::Response;
use crate::filter::ResponseFilter;

pub struct StatusCodeFilter {
    excluded_codes: Vec<u16>,
}

impl StatusCodeFilter {
    pub fn new(excluded_codes: Vec<u16>) -> Self {
        Self { excluded_codes }
    }
}

impl ResponseFilter for StatusCodeFilter {
    fn should_filter(&self, response: &Response) -> bool {
        self.excluded_codes.contains(&response.status().as_u16())
    }
}
