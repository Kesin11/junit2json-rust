use std::env;
use std::process;
use std::fs::File;
use std::io::BufReader;
use junit2json;

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("{:?}", args);
    let file = File::open(&args[1]).unwrap_or_else(|msg| {
        eprintln!("File::open error: {}", msg);
        process::exit(1);
    });
    let reader = BufReader::new(file);

    let testsuites = junit2json::from_reader(reader).unwrap_or_else(|msg| {
        eprintln!("junit2json::from_reader error: {}", msg);
        process::exit(1);
    });

    println!("{:#?}", testsuites);

    let json = serde_json::to_string_pretty(&testsuites).unwrap_or_else(|msg| {
        eprintln!("serde_json::to_string_pretty error: {}", msg);
        process::exit(1);
    });
    println!("{}", json);

    // file::write(&testsuites, &config).unwrap_or_else(|msg| {
    //     eprintln!("Write JSON error: {}", msg);
    //     process::exit(1);
    // });
}
