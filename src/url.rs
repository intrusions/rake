use std::time::Duration;
use reqwest::{
    blocking::Client,
    Error
};

pub struct Url {
    client: Client,

    pub str: String,
}

impl Url {
    pub fn new(str: &String) -> Result<Self, Error> {
        let str = str.clone();

        let client = reqwest::blocking::Client::builder()
            .timeout(Duration::from_millis(5000))
            .build()?;
        
        let url = Self { client, str };

        match url.is_reachable() {
            Ok(()) => Ok(url),
            Err(e) => Err(e),
        }
    }

    pub fn is_reachable(&self) -> Result<(), Error> { 
        match self.client.get(&self.str).send() {
            Ok(_) => Ok(()),
            Err(e) =>  Err(e),
        }
    }
}
