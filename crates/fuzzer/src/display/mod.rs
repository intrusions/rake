pub mod display;
pub mod filter;

pub use display::Display;

pub struct DisplayArgs {
    pub url: String,
    pub wordlist: String,
    pub threads: u8,
    pub timeout: u64,
    pub user_agent: String,
    pub filtered_code: Vec<u16>,
    pub filtered_size: Vec<u64>,
    pub matched_code: Vec<u16>,
    pub matched_size: Vec<u64>,
    pub method: String,
}
