mod logger;
mod filter;
pub use logger::Logger;

pub struct LoggerArgs {
    pub url: String,
    pub wordlist: String,
    pub threads: u8,
    pub timeout: u64,
    pub user_agent: String,
    pub exclude_codes: Vec<u16>,
    pub exclude_size: Vec<u64>,
}
