pub mod filter_content_size;
pub mod filter_status_code;
pub mod filter_word;

pub use filter_content_size::ContentSizeFilter;
pub use filter_status_code::StatusCodeFilter;
pub use filter_word::WordFilter;

pub trait ResponseFilter {
    fn should_filter(&self, status: u16, size: u64, body: &str) -> bool;
}
