use crate::args::Args;
use crate::wordlist::Wordlist;
use crate::sender::Sender;
use crate::logger::Logger;
use crate::url::Url;

use crossbeam::thread;
use std::sync::{
    Arc,
    Mutex
};

pub struct Fuzzer {
    pub wordlist: Wordlist,
    pub sender: Sender,
    pub url: Url,
    pub logger: Logger,
}

impl Fuzzer {
    pub fn new(args: &Args) -> Self {
        
        let wordlist = match Wordlist::new(&args.wordlist) {
            Ok(wl) => wl,
            Err(e) => panic!("Fatal error: {e}"),
        };

        let sender = match Sender::new(&args) {
            Ok(sender) => sender,
            Err(e) => panic!("Fatal error: {e}"),
        };

        let url = match Url::new(&args.url) {
            Ok(url) => url,
            Err(e) => panic!("Fatal error: {e}"),
        };

        let logger = Logger::new(&args);
        
        Self { wordlist, sender, url, logger }
    }

    pub fn fuzz(&mut self) {
        let wordlist = Arc::new(Mutex::new(&mut self.wordlist));
        let sender = Arc::new(&self.sender);
        let logger = Arc::new(&self.logger);
        let url_template = Arc::new(self.url.str.clone());

        let num_threads = 60;

        thread::scope(|s| {
            for _ in 0..num_threads {
                let wordlist = Arc::clone(&wordlist);
                let sender = Arc::clone(&sender);
                let logger = Arc::clone(&logger);
                let url_template = Arc::clone(&url_template);

                s.spawn(move |_| {
                    while let Ok(mut wl) = wordlist.lock() {
                        if wl.load_next_chunk().is_err() {
                            break;
                        }
                        let chunk = wl.chunk.clone();
                        drop(wl);
                        
                        for payload in chunk.iter() {
                            let url = url_template.replace("{}", payload);
                            
                            match sender.send(&url) {
                                Ok(response) => logger.log_response(response, &url),
                                Err(e) => eprintln!("Error: {}", e),
                            }
                        }
                    }
                });
            }
        }).unwrap();
    }
}
