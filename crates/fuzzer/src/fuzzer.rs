use crate::FuzzerArgs;

use reader::{Reader, ReaderArgs};
use sender::{Sender, SenderArgs};
use logger::{Logger, LoggerArgs};
use std::sync::{Arc, Mutex};
use crossbeam::thread;

pub struct Fuzzer {
    pub reader: Reader,
    pub sender: Sender,
    pub logger: Logger,
}

impl Fuzzer {
    pub fn new(args: &FuzzerArgs) -> Self {
        
        let reader = Reader::new(ReaderArgs::from(args))
            .unwrap_or_else(|e| panic!("Fatal error: {}", e.as_str()));
        
        let sender = Sender::new(SenderArgs::from(args))
            .unwrap_or_else(|e| panic!("Fatal error: {}", e.as_str()));

        let logger = Logger::new(LoggerArgs::from(args));
        
        Self { reader, sender, logger }
    }

    pub fn fuzz(&mut self) {
        let reader = Arc::new(Mutex::new(&mut self.reader));
        let sender = Arc::new(&self.sender);
        let logger = Arc::new(&self.logger);
        let url_template = Arc::new(self.sender.args.url.clone());

        let num_threads = 60;

        thread::scope(|s| {
            for _ in 0..num_threads {
                let reader = Arc::clone(&reader);
                let sender = Arc::clone(&sender);
                let logger = Arc::clone(&logger);
                let url_template = Arc::clone(&url_template);

                s.spawn(move |_| {
                    while let Ok(mut wl) = reader.lock() {
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
