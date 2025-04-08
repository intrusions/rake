use clap::Parser;
use fuzzer::FuzzerArgs;

#[derive(Parser)]
pub struct ArgsSchema {
    /// Target URL to fuzz.
    /// The `{}` placeholder in the URL will be replaced by payloads.
    /// 
    /// Example: `https://rake.io/pages/{}`
    #[arg(short = 'u', long = "url")]
    #[arg(required = true)]
    pub url: String,

    /// Path to the wordlist file.
    #[arg(short = 'w', long = "wordlist")]
    #[arg(required = true)]
    pub wordlist: String,

    /// Number of threads.
    /// Default is 40
    #[arg(short = 't', long = "threads")]
    #[arg(default_value_t = 40, hide_default_value = true, hide_possible_values = true)]
    #[arg(value_parser = clap::value_parser!(u8).range(1..=120))]
    pub threads: u8,

    /// Request timeout in milliseconds.
    /// Default is 5000 ms (5 seconds).
    #[arg(short = 'T', long = "timeout")]
    #[arg(default_value_t = 5000, hide_default_value = true)]
    pub timeout: u64,

    /// Custom User-Agent string.
    /// Default: `rake/1.0`
    #[arg(short = 'a', long = "user-agent")]
    #[arg(default_value = "rake/1.0", hide_default_value = true)]
    pub user_agent: String,

    /// List of HTTP status codes to ignore.
    /// 
    /// Example: `404,403` will filter responses with status 404 or 403.
    #[arg(short = 'c', long = "filter-code")]
    #[arg(num_args = 1.., value_delimiter = ',')]
    pub filtered_code: Vec<u16>,

    /// List of HTTP status codes to match.
    /// 
    /// Example: `200` will match responses with status 200.
    #[arg(short = 'C', long = "match-code")]
    #[arg(num_args = 1.., value_delimiter = ',')]
    pub matched_code: Vec<u16>,
    
    /// List of content size to ignore.
    /// 
    /// Example: `1000,1001` will filter responses with content size of 1000 or 1001.
    #[arg(short = 's', long = "filter-size")]
    #[arg(num_args = 1.., value_delimiter = ',')]
    pub filtered_size: Vec<u64>,


    /// List of content size to match.
    /// 
    /// Example: `270` will filter responses with content size of 270.
    #[arg(short = 'S', long = "match-size")]
    #[arg(num_args = 1.., value_delimiter = ',')]
    pub matched_size: Vec<u64>,

    /// Follow redirects.
    /// Default is false
    #[arg(short = 'r', long = "follow-redirect")]
    #[arg(default_value_t = false)]
    pub follow_redirect: bool, 

    /// HTTP method to use.
    /// Default is `get`
    #[arg(short = 'X', long = "method")]
    #[arg(default_value = "GET")]
    pub method: String, 
}

impl From<ArgsSchema> for FuzzerArgs {
    fn from(args: ArgsSchema) -> FuzzerArgs {
        FuzzerArgs {
            url: args.url,
            wordlist: args.wordlist,
            threads: args.threads,
            timeout: args.timeout,
            user_agent: args.user_agent,
            filtered_code: args.filtered_code,
            filtered_size: args.filtered_size,
            matched_code: args.matched_code,
            matched_size: args.matched_size,
            follow_redirect: args.follow_redirect,
            method: args.method
        }
    }
}
