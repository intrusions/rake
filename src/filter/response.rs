use reqwest::blocking::Response;

pub trait ResponseFilter {
    fn should_filter(&self, response: &Response) -> bool;
}
