mod args;
mod fuzzer;
mod sender;
mod wordlist;
mod logger;
mod filter;
mod url;

use args::Args;
use fuzzer::Fuzzer;
use clap::Parser;

fn main() {
    let args = Args::parse();

    let mut fuzzer = Fuzzer::new(&args);
    fuzzer.fuzz();
}
