pub mod filter;
pub mod args;

use crate::display::filter::{ContentSizeFilter, ResponseFilter, StatusCodeFilter};
use crate::DisplayArgs;

use std::{time::Duration, ops::Add, fmt};
use filter::WordFilter;
use reqwest::blocking::Response;
use indicatif::{ProgressBar, ProgressStyle};
use colored::*;

pub struct Display {
    filters: Vec<Box<dyn ResponseFilter + Send + Sync>>,
    args: DisplayArgs,
    progress_bar: ProgressBar,
}

impl Display {
    pub fn new(args: DisplayArgs, wl_lines_count: usize) -> Self {
        Self::headers(&args);

        let progress_bar = ProgressBar::new(wl_lines_count as u64);
        progress_bar.set_style(
            ProgressStyle::default_bar()
                .template(":: Progress: [{pos}/{len}][{percent}%] :: Duration: {elapsed_precise} :: {per_sec}")
                .unwrap()
        );

        let mut display = Self {
            filters: Vec::new(),
            args,
            progress_bar,
        };

        display.filters.push(Box::new(StatusCodeFilter::new(
            display.args.filtered_code.clone(),
            display.args.matched_code.clone(),
        )));

        display.filters.push(Box::new(ContentSizeFilter::new(
            display.args.filtered_size.clone(),
            display.args.matched_size.clone(),
        )));

        display.filters.push(Box::new(WordFilter::new(
            display.args.filtered_word.clone(),
            display.args.matched_word.clone(),
        )));

        display
    }

    pub fn headers(args: &DisplayArgs) {
        pub fn range_formatted<T>(range: &[T]) -> Vec<String>
        where
            T: Copy + PartialOrd + Add<Output = T> + fmt::Display + Ord + TryFrom<u64> + Into<u64>,
        {
            if range.is_empty() {
                return vec![];
            }
        
            let mut sorted = range.to_owned();
            sorted.sort();
        
            let mut result = Vec::new();
            let mut start = sorted[0];
            let mut prev = sorted[0];
            let mut count = 1;
        
            for &num in sorted.iter().skip(1) {
                if num.into() == prev.into() + 1 {
                    count += 1;
                } else {
                    if count >= 6 {
                        result.push(format!("{}-{}", start, prev));
                    } else {
                        for n in start.into()..=prev.into() {
                            result.push(n.to_string());
                        }
                    }
                    start = num;
                    count = 1;
                }
                prev = num;
            }
        
            if count >= 6 {
                result.push(format!("{}-{}", start, prev));
            } else {
                for n in start.into()..=prev.into() {
                    result.push(n.to_string());
                }
            }
        
            result
        }

        println!("*=================================================*");
        println!();

        println!("* {:<14} : {}", "URL".dimmed(), args.url);
        println!("* {:<14} : {}", "Wordlist".dimmed(), args.wordlist);
        println!("* {:<14} : {}", "Threads".dimmed(), args.threads);
        println!("* {:<14} : {}", "Timeout".dimmed(), args.timeout);
        println!("* {:<14} : {}", "User-Agent".dimmed(), args.user_agent);

        if !args.filtered_code.is_empty() {
            println!("* {:<14} : {:?}", "Filtered code".dimmed(), range_formatted(&args.filtered_code));
        }
        if !args.filtered_size.is_empty() {
            println!("* {:<14} : {:?}", "Filtered size".dimmed(), range_formatted(&args.filtered_size));
        }
        if !args.matched_code.is_empty() {
            println!("* {:<14} : {:?}", "Matched code".dimmed(), range_formatted(&args.matched_code));
        }
        if !args.matched_size.is_empty() {
            println!("* {:<14} : {:?}", "Matched size".dimmed(), range_formatted(&args.matched_size));
        }

        println!("* {:<14} : {}", "Method".dimmed(), args.method);
        println!();

        println!("*=================================================*");
        println!();
        println!("{:<6} {:<6} {:<8} URL", "STATUS", "SIZE", "TIME");
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
        let body = response.text().unwrap();

        if self
            .filters
            .iter()
            .any(|filter| filter.should_filter(status_code, content_size, &body))
        {
            return;
        }

        self.progress_bar.println(format!(
            "{:<6} {:<6} {:<8} {}",
            Self::status_formatter(status_code),
            Self::size_formatter(content_size),
            Self::time_formatter(time),
            url.trim()
        ));
    }

    pub fn bar_inc_progress(&self) {
        self.progress_bar.inc(1);
    }
}
