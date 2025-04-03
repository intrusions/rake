use crate::args::FuzzerArgs;
use crate::logger::Logger;

use wordlist::{Wordlist, WordlistArgs};
use sender::{Sender, SenderArgs};
use std::sync::{Arc, Mutex};
use crossbeam::thread;

pub struct Fuzzer {
    pub wordlist: Wordlist,
    pub sender: Sender,
    pub logger: Logger,
}

impl Fuzzer {
    pub fn new(args: &FuzzerArgs) -> Self {
        
        let wordlist = Wordlist::new(WordlistArgs::from(args))
            .unwrap_or_else(|e| panic!("Fatal error: {}", e.as_str()));
        
        let sender = Sender::new(SenderArgs::from(args))
            .unwrap_or_else(|e| panic!("Fatal error: {}", e.as_str()));

        let logger = Logger::new(args);
        
        Self { wordlist, sender, logger }
    }

    pub fn fuzz(&mut self) {
        let wordlist = Arc::new(Mutex::new(&mut self.wordlist));
        let sender = Arc::new(&self.sender);
        let logger = Arc::new(&self.logger);
        let url_template = Arc::new(self.sender.args.url.clone());

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
