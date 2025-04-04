use crate::LoggerArgs;
use crate::filter::{
    ResponseFilter,
    StatusCodeFilter,
    ContentSizeFilter
};

use std::time::Duration;
use reqwest::blocking::Response;
use colored::*;

pub struct Logger {
    filters: Vec<Box<dyn ResponseFilter + Send + Sync>>,
    args: LoggerArgs
}

impl Logger {
    pub fn new(args: LoggerArgs) -> Self {
        Self::headers(&args);

        let mut logger = Self {
            filters: Vec::new(),
            args
        };

        logger.filters.push(Box::new(StatusCodeFilter::new(logger.args.exclude_codes.clone())));
        logger.filters.push(Box::new(ContentSizeFilter::new(logger.args.exclude_size.clone())));

        logger
    }

    pub fn headers(args: &LoggerArgs) {
        println!("*=================================================*");
        println!();

        println!("* {:<14} : {}", "URL".dimmed(), args.url);
        println!("* {:<14} : {}", "Wordlist".dimmed(), args.wordlist);
        println!("* {:<14} : {}", "Timing Level".dimmed(), args.timing);
        println!("* {:<14} : {}", "Timeout".dimmed(), args.timeout);
        println!("* {:<14} : {}", "User-Agent".dimmed(), args.user_agent);
        println!("* {:<14} : {:?}", "Excluded code".dimmed(), args.exclude_codes);
        println!("* {:<14} : {:?}", "Excluded size".dimmed(), args.exclude_size);
        println!();

        println!("*=================================================*");
        println!();
        println!("{:<6} {:<6} {:<8} URL", "STATUS", "TIME", "SIZE");
    }

    fn status_formatter(status: u16) -> ColoredString {
        match status {
            200..=299 => format!("({})", status).green(),
            300..=399 => format!("({})", status).blue(),
            400..=499 => format!("({})", status).yellow(),
            500..=599 => format!("({})", status).red(),
            _ => format!("({})", status).white(),
        }
    }

    fn size_formatter(size: u64) -> ColoredString {
        match size {
            0 => format!("{}", size).red().dimmed(),
            _ => format!("{}", size).yellow().dimmed(),
        }
    }

    fn time_formatter(time: Duration) -> ColoredString {
        let mut time = time.as_millis().to_string();
        time.push_str("ms");
        time.dimmed()
    }

    pub fn log_response(&self, response: Response, time: Duration, url: &str) {
        if self.filters.iter().any(|filter| filter.should_filter(&response)) {
            return;
        }

        let status_code = response.status().as_u16();
        let content_size = response.content_length().unwrap_or(0);

        let formated_status = Self::status_formatter(status_code);
        let formated_size = Self::size_formatter(content_size);
        let formated_time = Self::time_formatter(time);

        println!("{:<6} {:<6} {:<8} {}",
            formated_status,
            formated_time,
            formated_size,
            url.trim()
        );
    }
}
