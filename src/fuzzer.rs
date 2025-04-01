use crate::args::Args;
use crate::wordlist::Wordlist;
use crate::sender::Sender;
use crate::logger::Logger;

use std::sync::{Arc, Mutex};
use crossbeam::thread;

pub struct Fuzzer {
    /// Wordlist used to generate payloads.
    pub wordlist: Wordlist,
    /// Sender responsible for sending HTTP requests.
    pub sender: Sender,
    /// Logger to record responses.
    pub logger: Logger,
    /// Target URL template containing `{}` as a placeholder for payloads.
    pub url: String,
}

impl Fuzzer {
    /// Creates a new `Fuzzer` instance.
    ///
    /// # Arguments
    /// * `args` - Command-line arguments.
    ///
    /// # Returns
    /// * `Ok(Self)` - On success.
    /// * `Err(e)` - If initialization fails.
    pub fn new(args: &Args) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            wordlist: Wordlist::new(&args.wordlist)?,
            sender: Sender::new(&args)?,
            logger: Logger::new(&args),
            url: args.url.clone(),
        })
    }

    /// Starts the fuzzing process with multiple threads.
    ///
    /// Each thread retrieves a chunk of words from the wordlist, replaces `{}` in the URL,
    /// sends requests, and logs responses. When a thread finishes a chunk, it retrieves the next one.
    pub fn fuzz(&mut self) {
        // Shared wordlist wrapped in a thread-safe Mutex to allow controlled access.
        let wordlist = Arc::new(Mutex::new(&mut self.wordlist));
        // Shared references to sender and logger, as they are immutable.
        let sender = Arc::new(&self.sender);
        let logger = Arc::new(&self.logger);
        let url_template = Arc::new(self.url.clone());

        // Define the number of threads to use for fuzzing.
        let num_threads = 30;

        // Create a thread pool using `crossbeam::thread::scope`
        thread::scope(|s| {
            for _ in 0..num_threads {
                // Clone the shared references for each thread.
                let wordlist = Arc::clone(&wordlist);
                let sender = Arc::clone(&sender);
                let logger = Arc::clone(&logger);
                let url_template = Arc::clone(&url_template);

                s.spawn(move |_| {
                    // Each thread continuously fetches chunks until no more are available.
                    while let Ok(mut wl) = wordlist.lock() {
                        // Try to load the next chunk, stop if no more data.
                        if wl.load_next_chunk().is_err() {
                            break;
                        }
                        // Clone the chunk to avoid holding the lock while processing.
                        let chunk = wl.chunk.clone();
                        drop(wl); // Release lock before processing the chunk.

                        // Process each word in the chunk.
                        for payload in chunk.iter() {
                            // Replace `{}` in the URL with the current payload.
                            let url = url_template.replace("{}", payload);
                            
                            // Send the request and log the response.
                            match sender.send(&url) {
                                Ok(response) => logger.print_line(response, &url),
                                Err(e) => eprintln!("Error: {}", e),
                            }
                        }
                    }
                });
            }
        }).unwrap();
    }
}
