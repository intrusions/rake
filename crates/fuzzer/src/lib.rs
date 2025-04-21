pub mod display;
pub mod fuzzer;

use crate::display::args::DisplayArgs;

#[derive(Clone)]
pub struct FuzzerArgs {
    pub url: String,
    pub wordlist: String,
    pub threads: u8,
    pub timeout: u64,
    pub user_agent: String,
    pub filtered_code: Vec<u16>,
    pub filtered_size: Vec<u64>,
    pub filtered_word: Vec<String>,
    pub matched_code: Vec<u16>,
    pub matched_size: Vec<u64>,
    pub matched_word: Vec<String>,
    pub follow_redirect: bool,
    pub method: String,
}

impl From<&FuzzerArgs> for DisplayArgs {
    fn from(args: &FuzzerArgs) -> DisplayArgs {
        DisplayArgs {
            url: args.url.clone(),
            wordlist: args.wordlist.clone(),
            threads: args.threads,
            timeout: args.timeout,
            user_agent: args.user_agent.clone(),
            filtered_code: args.filtered_code.clone(),
            filtered_size: args.filtered_size.clone(),
            filtered_word: args.filtered_word.clone(),
            matched_code: args.matched_code.clone(),
            matched_size: args.matched_size.clone(),
            matched_word: args.matched_word.clone(),
            method: args.method.clone(),
        }
    }
}
