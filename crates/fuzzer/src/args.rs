use sender::SenderArgs;
use wordlist::WordlistArgs;

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
 
impl From<&FuzzerArgs> for WordlistArgs {
    fn from(args: &FuzzerArgs) -> WordlistArgs {
        WordlistArgs {
            path: args.wordlist.clone()
        }
    }
}
