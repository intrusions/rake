use args::ArgsSchema;
use clap::Parser;
use fuzzer::{FuzzerArgs, fuzzer::Fuzzer};

mod args;

fn main() {
    let args_schema = ArgsSchema::parse();

    let mut fuzzer = Fuzzer::new(&FuzzerArgs::from(args_schema));
    fuzzer.fuzz();
}
