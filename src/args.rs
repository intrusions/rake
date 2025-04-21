use clap::Parser;
use fuzzer::FuzzerArgs;
use std::{
    convert::TryFrom,
    fmt::{Debug, Display},
    str::FromStr,
};

#[derive(Clone)]
pub enum RangeOrValue<T> {
    Single(T),
    Range(T, T),
}

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
    /// Default is `rake/1.0`
    #[arg(short = 'a', long = "user-agent")]
    #[arg(default_value = "rake/1.0", hide_default_value = true)]
    pub user_agent: String,

    /// List of HTTP status codes to ignore.
    ///
    /// Example: `200-300, 401` will filter responses with status beetwen 200 and 300, and 401.
    #[arg(short = 'c', long = "filter-code")]
    #[arg(num_args = 1.., value_delimiter = ',')]
    #[arg(value_parser(parse_range_or_value::<u16>))]
    pub filtered_code: Vec<RangeOrValue<u16>>,
    
    /// List of word contained in body to filter.
    ///
    /// Example: `admin,password` will match responses with body containing `admin` or `password`.
    #[arg(short = 'o', long = "filter-word")]
    #[arg(num_args = 1.., value_delimiter = ',')]
    pub filtered_word: Vec<String>,

    /// List of content size to ignore.
    ///
    /// Example: `1000-2000, 2777` will filter responses with content size beetwen 1000 and 2000, and 2777.
    #[arg(short = 's', long = "filter-size")]
    #[arg(num_args = 1.., value_delimiter = ',')]
    #[arg(value_parser(parse_range_or_value::<u64>))]
    pub filtered_size: Vec<RangeOrValue<u64>>,

    /// List of HTTP status codes to match.
    ///
    /// Example: `200-300, 401` will match responses with status beetwen 200 and 300, and 401.
    #[arg(short = 'C', long = "match-code")]
    #[arg(num_args = 1.., value_delimiter = ',')]
    #[arg(value_parser(parse_range_or_value::<u16>))]
    pub matched_code: Vec<RangeOrValue<u16>>,

    /// List of word contained in body to match.
    ///
    /// Example: `admin,password` will filter responses with body containing `admin` or `password`.
    #[arg(short = 'O', long = "match-word")]
    #[arg(num_args = 1.., value_delimiter = ',')]
    pub matched_word: Vec<String>,

    /// List of content size to match.
    ///
    /// Example: `1000-2000, 2777` will match responses with content size beetwen 1000 and 2000, and 2777.
    #[arg(short = 'S', long = "match-size")]
    #[arg(num_args = 1.., value_delimiter = ',')]
    #[arg(value_parser(parse_range_or_value::<u64>))]
    pub matched_size: Vec<RangeOrValue<u64>>,

    /// Follow redirects.
    /// Default is false
    #[arg(short = 'r', long = "follow-redirect")]
    #[arg(default_value_t = false)]
    pub follow_redirect: bool,

    /// HTTP method to use.
    /// Default is GET
    #[arg(short = 'X', long = "method")]
    #[arg(default_value = "GET", hide_default_value = true)]
    pub method: String,
}

pub fn parse_range_or_value<T>(s: &str) -> Result<RangeOrValue<T>, String>
where
    T: FromStr + PartialOrd + Copy,
    <T as FromStr>::Err: Display,
{
    if let Some((start, end)) = s.split_once('-') {
        let start: T = start
            .trim()
            .parse()
            .map_err(|e| format!("Invalid start of range: {}", e))?;
        let end: T = end
            .trim()
            .parse()
            .map_err(|e| format!("Invalid end of range: {}", e))?;

        if start > end {
            return Err("Start of range cannot be greater than end".into());
        }

        Ok(RangeOrValue::Range(start, end))
    } else {
        let single: T = s
            .trim()
            .parse()
            .map_err(|e| format!("Invalid number: {}", e))?;

        Ok(RangeOrValue::Single(single))
    }
}

pub fn expand_ranges<T>(input: Vec<RangeOrValue<T>>) -> Vec<T>
where
    T: Into<u64> + TryFrom<u64>,
    <T as TryFrom<u64>>::Error: Debug,
{
    input
        .into_iter()
        .flat_map(|r| match r {
            RangeOrValue::Single(v) => vec![v],
            RangeOrValue::Range(start, end) => {
                let start = start.into();
                let end = end.into();

                (start..=end).map(|v| T::try_from(v).unwrap()).collect()
            }
        })
        .collect()
}

impl From<ArgsSchema> for FuzzerArgs {
    fn from(args: ArgsSchema) -> FuzzerArgs {
        FuzzerArgs {
            url: args.url,
            wordlist: args.wordlist,
            threads: args.threads,
            timeout: args.timeout,
            user_agent: args.user_agent,
            filtered_code: expand_ranges(args.filtered_code),
            filtered_size: expand_ranges(args.filtered_size),
            filtered_word: args.filtered_word,
            matched_code: expand_ranges(args.matched_code),
            matched_size: expand_ranges(args.matched_size),
            matched_word: args.matched_word,
            follow_redirect: args.follow_redirect,
            method: args.method,
        }
    }
}
