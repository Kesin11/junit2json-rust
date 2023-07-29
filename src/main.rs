use std::process;
use std::fs::File;
use std::io::BufReader;
use junit2json;
use junit2json::cli;
use clap::Parser;

fn main() {
    let args = cli::Args::parse();

    // println!("{:?}", args);
    let file = File::open(&args.path).unwrap_or_else(|msg| {
        eprintln!("File::open error: {}", msg);
        process::exit(1);
    });
    let reader = BufReader::new(file);
    let mut testsuites = junit2json::from_reader(reader).unwrap_or_else(|msg| {
        eprintln!("junit2json::from_reader error: {}", msg);
        process::exit(1);
    });
    // println!("{:#?}", testsuites);

    // Filter tags
    if let Some(tags) = args.filter_tags {
        if !tags.is_empty() {
            testsuites.filter_tags(&tags);
        }
    }

    // Convert to JSON string
    let json = match args.pretty {
        true => serde_json::to_string_pretty(&testsuites).unwrap_or_else(|msg| {
            eprintln!("serde_json::to_string_pretty error: {}", msg);
            process::exit(1);
        }),
        false => serde_json::to_string(&testsuites).unwrap_or_else(|msg| {
            eprintln!("serde_json::to_string error: {}", msg);
            process::exit(1);
        }),
    };

    println!("{}", json);
}
