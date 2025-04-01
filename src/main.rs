mod args;
mod fuzzer;
mod sender;
mod wordlist;
mod logger;

use args::Args;
use fuzzer::Fuzzer;
use clap::Parser;

fn main() {
    let args = Args::parse();

    let mut fuzzer = match Fuzzer::new(&args) {
        Ok(fuzzer) => fuzzer,
        Err(e) => panic!("{e}"),
    };

    fuzzer.fuzz();
}
