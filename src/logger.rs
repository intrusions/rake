use reqwest::blocking::Response;

use crate::args::Args;
use colored::*;

pub struct Logger { }

impl Logger {
    pub fn new(args: &Args) -> Self {
        println!("*=================================================*");
        println!();

        println!(":: URL            : {}", args.url);
        println!(":: Wordlist       : {}", args.wordlist);
        println!(":: Timing Level   : {}", args.timing);
        println!(":: Timeout        : {}", args.timeout);
        println!(":: User-Agent     : {}", args.user_agent);
        println!(":: Excluded Code  : {:?}", args.exclude_codes);
        println!();

        println!("*=================================================*");
        println!();

        Self { }
    }

    pub fn print_line(&self, response: Response, url: &String) {
        let status_code = response.status().as_u16();
        
        let colored_status = match response.status().as_u16() {
            200..=299 => format!("({})", status_code).green(),
            300..=399 => format!("({})", status_code).blue(),
            400..=499 => format!("({})", status_code).yellow(),
            500..=599 => format!("({})", status_code).red(),
            _ => format!("({})", status_code).white(),
        };
    
        println!("{} {}", colored_status, url);
    }
}
