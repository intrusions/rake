use crate::FuzzerArgs;

use reader::{builder::ReaderBuilder, Reader};
use sender::{builder::SenderBuilder, Sender};

use logger::{Logger, LoggerArgs};
use std::sync::{Arc, Mutex};
use crossbeam::thread;

pub struct Fuzzer {
    pub reader: Reader,
    pub sender: Sender,
    pub logger: Logger,
    args: FuzzerArgs
}

impl Fuzzer {
    pub fn new(args: &FuzzerArgs) -> Self {
        let reader = ReaderBuilder::new()
            .with_path(args.wordlist.clone())
            .with_threads(args.threads)
            .build()
            .unwrap_or_else(|e| panic!("{}", e.as_str()));
        
        let sender = SenderBuilder::new()
            .with_url(args.url.clone())
            .with_user_agent(&args.user_agent)
            .with_follow_redirect(args.follow_redirect)
            .build()
            .unwrap_or_else(|e| panic!("{}", e.as_str()));

        let logger = Logger::new(LoggerArgs::from(args), reader.line_count);
        
        Self {
            reader,
            sender,
            logger,
            args: args.clone()
        }
    }

    pub fn fuzz(&mut self) {
        let reader = Arc::new(Mutex::new(&mut self.reader));
        let sender = Arc::new(&self.sender);
        let logger = Arc::new(&self.logger);
        let url_template = Arc::new(self.sender.args.url.clone());

        let num_threads = self.args.threads;

        thread::scope(|s| {
            for _ in 0..num_threads {
                let reader = Arc::clone(&reader);
                let sender = Arc::clone(&sender);
                let logger = Arc::clone(&logger);
                let url_template = Arc::clone(&url_template);

                s.spawn(move |_| {
                    while let Ok(mut reader) = reader.lock() {
                        let chunk = match reader.get_next_chunk() {
                            Ok(chunk) => chunk,
                            Err(_) => {
                                drop(reader);
                                break;
                            }
                        };

                        drop(reader);
                        
                        for word in chunk.iter() {
                            let url = url_template.replace("{}", word);
                            for _ in 0..3 {
                                match sender.send(&url) {
                                    Ok((response, time)) => {
                                        logger.log_response(response, time, &url);
                                        break;
                                    }
                                    Err(_) => {
                                        std::thread::sleep(std::time::Duration::from_millis(100))
                                    },
                                }
                            }
                        }
                    }
                });
            }
        }).unwrap();
    }
}
