use crate::SenderArgs;
use std::{
    time::{Duration, SystemTime},
    str::FromStr
};
use reqwest::{
    blocking::{Client, Response},
    redirect::Policy,
    Method,
    Error
};

pub struct Sender {
    client: Client,
    pub args: SenderArgs,
    pub method: Method
}

#[derive(Debug)]
pub enum SenderError {
    Builder,
    Unreachable,
    HttpMethod
}

impl SenderError {
    pub fn as_str(&self) -> &'static str {
        match self {
            SenderError::Builder => "The HTTP Client cannot be creat",
            SenderError::Unreachable => "The target address is not reachable",
            SenderError::HttpMethod => "The method specified by `-X` is invalid"
        }
    }
}

impl Sender {
    pub fn new(args: SenderArgs) -> Result<Self, SenderError> {        
        let method = Method::from_str(args.method.as_str())
            .map_err(|_| SenderError::HttpMethod)?;

        let policy = match args.follow_redirect {
            true => Policy::default(),
            false => Policy::none()
        };
    
        let client = reqwest::blocking::Client::builder()
            .timeout(Duration::from_millis(args.request_timeout))
            .user_agent(&args.user_agent)
            .redirect(policy)
            .build()
            .map_err(|_| SenderError::Builder)?;
        
        let sender = Self {
            client,
            method,
            args
        };
    
        if !sender.ping(){
            return Err(SenderError::Unreachable); 
        }

        Ok(sender)
    }
    
    fn ping(&self) -> bool { 
        self.client.get(&self.args.url).send().is_ok()
    }

    pub fn send(&self, url: &String) -> Result<(Response, Duration), Error> {
        let now = SystemTime::now();
        let response = self.client.request(self.method.clone(), url).send()?;
        let elapsed = now.elapsed().unwrap();
        Ok((response, elapsed))
    }
    
}
