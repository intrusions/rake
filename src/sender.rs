use reqwest::{blocking::{Client, Response}, Error};
use std::time::Duration;
use crate::args::Args;

pub struct Sender {
    pub client: Client,
}

impl Sender {
    pub fn new(args: &Args) -> Result<Self, Error> { 
        let client = reqwest::blocking::Client::builder()
            .timeout(Duration::from_millis(args.timeout))
            .build()?;
        
        Ok(Self {
            client: client,
        })
    }

    pub fn send(&self, url: &String) -> Result<Response, Error> {
        match self.client.get(url).send() {
            Ok(response) => Ok(response),
            Err(e) => Err(e),
        }
    }
}
