mod reader;
pub use reader::Reader;

pub struct ReaderArgs {
    pub path: String,
    pub threads: u8
}
