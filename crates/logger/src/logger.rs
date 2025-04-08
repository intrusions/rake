use crate::LoggerArgs;
use crate::filter::{
    ResponseFilter,
    StatusCodeFilter,
    ContentSizeFilter
};

use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;
use reqwest::blocking::Response;
use colored::*;

pub struct Logger {
    filters: Vec<Box<dyn ResponseFilter + Send + Sync>>,
    args: LoggerArgs,
    progress_bar: ProgressBar
}

impl Logger {
    pub fn new(args: LoggerArgs, wl_lines_count: usize) -> Self {
        Self::headers(&args);

        let progress_bar = ProgressBar::new(wl_lines_count as u64);
        progress_bar.set_style(
            ProgressStyle::default_bar()
                .template(":: Progress: [{pos}/{len}][{percent}%] :: Duration: {elapsed_precise} :: {per_sec}")
                .unwrap()
        );

        let mut logger = Self {
            filters: Vec::new(),
            args,
            progress_bar
        };

        logger.filters.push(Box::new(StatusCodeFilter::new(logger.args.filtered_code.clone(), 
        logger.args.matched_code.clone())));
        
        logger.filters.push(Box::new(ContentSizeFilter::new(logger.args.filtered_size.clone(), 
        logger.args.matched_size.clone())));

        logger
    }

    pub fn headers(args: &LoggerArgs) {
        println!("*=================================================*");
        println!();

        println!("* {:<14} : {}", "URL".dimmed(), args.url);
        println!("* {:<14} : {}", "Wordlist".dimmed(), args.wordlist);
        println!("* {:<14} : {}", "Threads".dimmed(), args.threads);
        println!("* {:<14} : {}", "Timeout".dimmed(), args.timeout);
        println!("* {:<14} : {}", "User-Agent".dimmed(), args.user_agent);
        println!("* {:<14} : {:?}", "Filtered code".dimmed(), args.filtered_code);
        println!("* {:<14} : {:?}", "Filtered size".dimmed(), args.filtered_size);
        println!("* {:<14} : {}", "Method".dimmed(), args.method);
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
        self.bar_inc_progress();

        let status_code = response.status().as_u16();
        let content_size = response.content_length().unwrap_or(0);

        if self.filters.iter().any(|filter| filter.should_filter(&response)) {
            return;
        }
        
        let formated_status = Self::status_formatter(status_code);
        let formated_size = Self::size_formatter(content_size);
        let formated_time = Self::time_formatter(time);

        self.progress_bar.println(format!("{:<6} {:<6} {:<8} {}",
            formated_status,
            formated_time,
            formated_size,
            url.trim()
        ));
    }

    pub fn bar_inc_progress(&self) {
        self.progress_bar.inc(1);
    }
}
