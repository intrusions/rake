pub mod sender;
pub mod builder;

pub use sender::Sender;
pub use builder::SenderBuilder;

pub struct SenderArgs {
    pub user_agent: String,
    pub request_timeout: u64,
    pub url: String,
    pub follow_redirect: bool,
    pub method: String
}
