use crate::SenderArgs;
use reqwest::{
    Error, Method,
    blocking::{Client, Response},
};
use std::time::{Duration, SystemTime};

pub struct Sender {
    pub client: Client,
    pub args: SenderArgs,
    pub method: Method,
}

impl Sender {
    pub fn is_reachable(&self) -> bool {
        self.client.get(&self.args.url).send().is_ok()
    }

    pub fn send(&self, url: &String) -> Result<(Response, Duration), Error> {
        let now = SystemTime::now();
        let response = self.client.request(self.method.clone(), url).send()?;
        let elapsed = now.elapsed().unwrap();
        Ok((response, elapsed))
    }
}
