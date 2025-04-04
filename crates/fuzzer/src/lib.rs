pub mod fuzzer;
use sender::SenderArgs;
use reader::ReaderArgs;
use logger::LoggerArgs;

pub struct FuzzerArgs {
    pub url: String,
    pub wordlist: String,
    pub timing: u8,
    pub timeout: u64,
    pub user_agent: String,
    pub exclude_codes: Vec<u16>,
    pub exclude_size: Vec<u64>,
}

impl From<&FuzzerArgs> for SenderArgs {
    fn from(args: &FuzzerArgs) -> SenderArgs {
        SenderArgs {
            user_agent: args.user_agent.clone(),
            request_timeout: args.timeout,
            url: args.url.clone()
        }
    }
}
 
impl From<&FuzzerArgs> for ReaderArgs {
    fn from(args: &FuzzerArgs) -> ReaderArgs {
        ReaderArgs {
            path: args.wordlist.clone()
        }
    }
}

impl From<&FuzzerArgs> for LoggerArgs {
    fn from(args: &FuzzerArgs) -> LoggerArgs {
        LoggerArgs {
            url: args.url.clone(),
            wordlist: args.wordlist.clone(),
            timing: args.timing,
            timeout: args.timeout,
            user_agent: args.user_agent.clone(),
            exclude_codes: args.exclude_codes.clone(),
            exclude_size: args.exclude_size.clone()
        }
    }
}
