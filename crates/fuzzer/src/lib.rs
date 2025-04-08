pub mod fuzzer;
use sender::SenderArgs;
use reader::ReaderArgs;
use logger::LoggerArgs;

#[derive(Clone)]
pub struct FuzzerArgs {
    pub url: String,
    pub wordlist: String,
    pub threads: u8,
    pub timeout: u64,
    pub user_agent: String,
    pub filtered_code: Vec<u16>,
    pub filtered_size: Vec<u64>,
    pub matched_code: Vec<u16>,
    pub matched_size: Vec<u64>,
    pub follow_redirect: bool,
    pub method: String
}

impl From<&FuzzerArgs> for SenderArgs {
    fn from(args: &FuzzerArgs) -> SenderArgs {
        SenderArgs {
            user_agent: args.user_agent.clone(),
            request_timeout: args.timeout,
            url: args.url.clone(),
            follow_redirect: args.follow_redirect,
            method: args.method.clone()
        }
    }
}
 
impl From<&FuzzerArgs> for ReaderArgs {
    fn from(args: &FuzzerArgs) -> ReaderArgs {
        ReaderArgs {
            path: args.wordlist.clone(),
            threads: args.threads
        }
    }
}

impl From<&FuzzerArgs> for LoggerArgs {
    fn from(args: &FuzzerArgs) -> LoggerArgs {
        LoggerArgs {
            url: args.url.clone(),
            wordlist: args.wordlist.clone(),
            threads: args.threads,
            timeout: args.timeout,
            user_agent: args.user_agent.clone(),
            filtered_code: args.filtered_code.clone(),
            filtered_size: args.filtered_size.clone(),
            matched_code: args.matched_code.clone(),
            matched_size: args.matched_size.clone(),
            method: args.method.clone()
        }
    }
}
