use crate::SenderArgs;
use std::time::Duration;
use reqwest::{
    blocking::{
        Client, 
        Response,
    }, 
    Error
};

pub struct Sender {
    client: Client,
    pub args: SenderArgs
}

pub enum SenderError {
    Builder,
    Unreachable
}

impl SenderError {
    pub fn as_str(&self) -> &'static str {
        match self {
            SenderError::Builder => "builder err",
            SenderError::Unreachable => "addr err",
        }
    }
}

impl Sender {
    pub fn new(args: SenderArgs) -> Result<Self, SenderError> { 
        let args = SenderArgs {
            user_agent: args.user_agent,
            request_timeout: args.request_timeout,
            url: args.url
        };
        
        let client = reqwest::blocking::Client::builder()
            .timeout(Duration::from_millis(args.request_timeout))
            .user_agent(&args.user_agent)
            .build().map_err(|_| {
                SenderError::Builder
        })?;
        
        let sender = Self {
            client,
            args
        };

        match sender.ping() {
            true => Ok(sender),
            false => Err(SenderError::Unreachable)
        }
    }

    pub fn ping(&self) -> bool { 
        self.client.get(&self.args.url).send().is_ok()
    }

    pub fn send(&self, url: &String) -> Result<Response, Error> {
        self.client.get(url).send()
    }
}
