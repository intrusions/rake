use clap::Parser;

#[derive(Parser)]
pub struct Args {
    /// Target URL to fuzz.
    /// The `{}` placeholder in the URL will be replaced by payloads.
    /// 
    /// Example: `www.rake.io/pages/{}`
    #[arg(short = 'u', long = "url")]
    #[arg(required = true)]
    pub url: String,

    /// Path to the wordlist file.
    #[arg(short = 'w', long = "wordlist")]
    #[arg(required = true)]
    pub wordlist: String,

    /// Timing level for request speed.
    /// Ranges from 1 (slow) to 5 (very fast).
    #[arg(short = 't', long = "timing")]
    #[arg(default_value_t = 3, hide_default_value = true, hide_possible_values = true)]
    #[arg(value_parser = clap::value_parser!(u8).range(1..=5))]
    pub timing: u8,

    /// Request timeout in milliseconds.
    /// Default is 5000 ms (5 seconds).
    #[arg(short = 'm', long = "timeout")]
    #[arg(default_value_t = 5000, hide_default_value = true)]
    pub timeout: u64,

    /// Custom User-Agent string.
    /// Default: `rake/1.0`
    #[arg(short = 'a', long = "user-agent")]
    #[arg(default_value = "rake/1.0", hide_default_value = true)]
    pub user_agent: String,

    /// List of HTTP status codes to ignore.
    /// 
    /// Example: `404,403` will exclude responses with status 404 or 403.
    #[arg(short = 'c', long = "exclude-code")]
    #[arg(num_args = 1.., value_delimiter = ',')]
    pub exclude_codes: Vec<u16>,

    /// List of content size to ignore.
    /// 
    /// Example: `1000,1001` will exclude responses with content size of 1000 or 1001.
    #[arg(short = 's', long = "exclude-size")]
    #[arg(num_args = 1.., value_delimiter = ',')]
    pub exclude_size: Vec<u64>,
}
