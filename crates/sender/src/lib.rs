pub mod builder;
pub mod sender;

pub use builder::SenderBuilder;
pub use sender::Sender;

pub struct SenderArgs {
    pub user_agent: String,
    pub request_timeout: u64,
    pub url: String,
    pub follow_redirect: bool,
    pub method: String,
}
