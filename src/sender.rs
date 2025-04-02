use crate::args::Args;

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
}

impl Sender {
    pub fn new(args: &Args) -> Result<Self, Error> { 
        let client = reqwest::blocking::Client::builder()
            .timeout(Duration::from_millis(args.timeout))
            .user_agent(&args.user_agent)
            .build()?;
        
        Ok(Self { client })
    }

    pub fn send(&self, url: &String) -> Result<Response, Error> {
        self.client.get(url).send()
    }
}
