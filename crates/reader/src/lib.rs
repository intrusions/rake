pub mod reader;
pub mod builder;

pub use reader::Reader;

pub struct ReaderArgs {
    pub path: String,
    pub threads: u8
}
