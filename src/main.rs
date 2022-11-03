#[macro_use]
extern crate log;

use std::str::from_utf8;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author = "Ryan P.", version, about)]
/// Generates a json file containing a list
struct Args {
    /// Contains a file with list entries on each line
    /// Example:
    /// 1. foo
    /// 2. bar
    /// 3. buzz
    #[arg(short, long = "input-file")]
    input_file: String,

    /// The output file name
    #[arg(short, long = "output-file")]
    output_file: String,

    /// Pretty-print the final JSON
    #[arg(short, long)]
    pretty: bool,
}

fn main() {
    pretty_env_logger::init();

    let args = Args::parse();

    let raw_input = std::fs::read(&args.input_file).expect("Unable to read input file");
    let input = from_utf8(&raw_input).unwrap();

    let list: Vec<&str> = input.split_terminator("\n").collect();

    let json_list: String;
    if args.pretty {
        json_list = serde_json::to_string_pretty(&list).unwrap();
    } else {
        json_list = serde_json::to_string(&list).unwrap();
    }

    std::fs::write(&args.output_file, &json_list).expect("Unable to write to file");
    info!("Your file has been created at {}", &args.output_file);
}
