use args::ArgsSchema;
use fuzzer::{fuzzer::Fuzzer, args::FuzzerArgs};
use clap::Parser;

mod args;

fn main() {
    let args_schema = ArgsSchema::parse();

    let mut fuzzer = Fuzzer::new(&FuzzerArgs::from(args_schema));
    fuzzer.fuzz();
}
