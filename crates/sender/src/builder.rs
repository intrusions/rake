use crate::Sender;

use crate::SenderArgs;
use reqwest::{Method, redirect::Policy};
use std::{str::FromStr, time::Duration};

pub enum SenderBuilderError {
    InvalidHTTPMethod,
    Builder,
    HostUnreachable,
}

impl SenderBuilderError {
    pub fn as_str(&self) -> &'static str {
        match self {
            SenderBuilderError::InvalidHTTPMethod => "Method specified is not a valid HTTP method",
            SenderBuilderError::Builder => "TLS backend cannot be initialized, or the resolver cannot load the system configuration",
            SenderBuilderError::HostUnreachable => "Specified host is unreachable",
        }
    }
}

pub struct SenderBuilder {
    user_agent: String,
    request_timeout: u64,
    url: Option<String>,
    follow_redirect: bool,
    method: String,
}

impl SenderBuilder {
    pub fn new() -> Self {
        Self {
            user_agent: String::from("rake/1.0"),
            request_timeout: 5000,
            url: None,
            follow_redirect: false,
            method: String::from("GET"),
        }
    }

    pub fn with_user_agent(mut self, user_agent: &str) -> Self {
        self.user_agent = user_agent.to_string();
        self
    }

    pub fn with_request_timeout(mut self, request_timeout: u64) -> Self {
        self.request_timeout = request_timeout;
        self
    }

    pub fn with_url(mut self, url: String) -> Self {
        self.url = Some(url);
        self
    }

    pub fn with_follow_redirect(mut self, follow_redirect: bool) -> Self {
        self.follow_redirect = follow_redirect;
        self
    }

    pub fn with_method(mut self, method: &str) -> Self {
        self.method = method.to_string();
        self
    }

    pub fn build(&self) -> Result<Sender, SenderBuilderError> {
        let method = Method::from_str(self.method.as_str())
            .map_err(|_| SenderBuilderError::InvalidHTTPMethod)?;

        let policy = match self.follow_redirect {
            true => Policy::default(),
            false => Policy::none(),
        };

        let client = reqwest::blocking::Client::builder()
            .timeout(Duration::from_millis(self.request_timeout))
            .user_agent(&self.user_agent)
            .redirect(policy)
            .build()
            .map_err(|_| SenderBuilderError::Builder)?;

        let sender = Sender {
            client,
            method,
            args: SenderArgs {
                user_agent: self.user_agent.clone(),
                request_timeout: self.request_timeout,
                url: self.url.clone().unwrap(),
                follow_redirect: self.follow_redirect,
                method: self.method.clone(),
            },
        };

        match sender.is_reachable() {
            true => Ok(sender),
            false => Err(SenderBuilderError::HostUnreachable),
        }
    }
}
