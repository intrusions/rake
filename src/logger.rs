use reqwest::blocking::Response;
use colored::*;
use std::sync::Arc;
use crate::args::Args;

pub struct Logger {
    /// List of filters applied to HTTP responses.
    filters: Vec<Arc<dyn Fn(&Response) -> bool + Send + Sync>>,
}

impl Logger {
    /// Creates a new `Logger` instance with predefined filters.
    ///
    /// # Arguments
    /// * `args` - CLI arguments containing filters like status codes and content sizes.
    ///
    /// # Returns
    /// * `Self` - A configured `Logger` instance.
    pub fn new(args: &Args) -> Self {
        Self::headers(args);

        let mut logger = Self {
            filters: Vec::new(),
        };
        
        // Filter: Exclude specific HTTP status codes
        let excluded_codes = args.exclude_codes.clone();
        logger.add_filter(move |response| {
            excluded_codes.contains(&response.status().as_u16())
        });

        // Filter: Exclude specific content sizes
        let excluded_sizes = args.exclude_size.clone();
        logger.add_filter(move |response| {
            let size = response.content_length().unwrap_or(0);
            excluded_sizes.contains(&size)
        });

        logger
    }

    /// Displays the header containing configuration settings.
    ///
    /// # Arguments
    /// * `args` - CLI arguments containing configuration options.
    pub fn headers(args: &Args) {
        println!("*=================================================*");
        println!();

        println!(":: URL            : {}", args.url);
        println!(":: Wordlist       : {}", args.wordlist);
        println!(":: Timing Level   : {}", args.timing);
        println!(":: Timeout        : {}", args.timeout);
        println!(":: User-Agent     : {}", args.user_agent);
        println!(":: Excluded Code  : {:?}", args.exclude_codes);
        println!(":: Excluded Size  : {:?}", args.exclude_size);
        println!();

        println!("*=================================================*");
        println!();
        println!("{:<6} {:<8} {}", "STATUS", "SIZE", "URL");
    }

    /// Adds a new filter function to the logger.
    ///
    /// # Arguments
    /// * `filter` - A closure that takes a `Response` and returns `true` if it should be filtered.
    pub fn add_filter<F>(&mut self, filter: F)
    where
        F: Fn(&Response) -> bool + Send + Sync + 'static,
    {
        self.filters.push(Arc::new(filter));
    }

    /// Formats the HTTP status code with colors.
    ///
    /// # Arguments
    /// * `status` - The HTTP status code.
    ///
    /// # Returns
    /// * `ColoredString` - The formatted and colored status code.
    fn status_formatter(status: u16) -> ColoredString {
        match status {
            200..=299 => format!("({})", status).green(),
            300..=399 => format!("({})", status).blue(),
            400..=499 => format!("({})", status).yellow(),
            500..=599 => format!("({})", status).red(),
            _ => format!("({})", status).white(),
        }
    }

    /// Formats the content size with colors.
    ///
    /// # Arguments
    /// * `size` - The size of the HTTP response content.
    ///
    /// # Returns
    /// * `ColoredString` - The formatted and colored size.
    fn size_formatter(size: u64) -> ColoredString {
        match size {
            0 => format!("{}", size).red().dimmed(),
            _ => format!("{}", size).white().dimmed(),
        }
    }

    /// Logs the HTTP response if it does not match any filter.
    ///
    /// # Arguments
    /// * `response` - The HTTP response to log.
    /// * `url` - The URL associated with the response.
    pub fn log_response(&self, response: Response, url: &str) {
        let status_code = response.status().as_u16();
        let content_size = response.content_length().unwrap_or(0);

        // Apply filters; skip logging if any filter matches.
        if self.filters.iter().any(|f| f(&response)) {
            return;
        }

        let colored_status = Self::status_formatter(status_code);
        let colored_size = Self::size_formatter(content_size);

        println!("{:<6} {:<8} {}", colored_status, colored_size, url);
    }
}
