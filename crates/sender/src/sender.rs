use crate::SenderArgs;
use std::time::{Duration, SystemTime};
use reqwest::{
    blocking::{Client, Response},
    redirect::Policy,
    Error
};

pub struct Sender {
    client: Client,
    pub args: SenderArgs
}

#[derive(Debug)]
pub enum SenderError {
    Builder,
    Unreachable
}

impl SenderError {
    pub fn as_str(&self) -> &'static str {
        match self {
            SenderError::Builder => "The HTTP Client cannot be creat",
            SenderError::Unreachable => "The target address is not reachable",
        }
    }
}

impl Sender {
    pub fn new(args: SenderArgs) -> Result<Self, SenderError> { 
        let args = SenderArgs {
            user_agent: args.user_agent,
            request_timeout: args.request_timeout,
            url: args.url,
            follow_redirect: args.follow_redirect
        };
        
        let policy = match args.follow_redirect {
            false => Policy::none(),
            true => Policy::default()
        };

        let client = reqwest::blocking::Client::builder()
            .timeout(Duration::from_millis(args.request_timeout))
            .user_agent(&args.user_agent)
            .redirect(policy)
            .build()
            .map_err(|_| {
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

    pub fn send(&self, url: &String) -> Result<(Response, Duration), Error> {
        let now = SystemTime::now();
        let response = self.client.get(url).send()?;
        let elapsed = now.elapsed().unwrap();
        
        Ok((response, elapsed))
    }
}
