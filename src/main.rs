use args::ArgsSchema;
use fuzzer::{fuzzer::Fuzzer, FuzzerArgs};
use clap::Parser;

mod args;

fn main() {
    let args_schema = ArgsSchema::parse();

    let mut fuzzer = Fuzzer::new(&FuzzerArgs::from(args_schema));
    fuzzer.fuzz();
}
