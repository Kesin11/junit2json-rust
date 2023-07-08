use std::env;
use std::process;
use std::fs::File;
use std::io::BufReader;
use junit2json::TestSuitesOrTestSuite;
use serde_xml_rs;

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("{:?}", args);
    let file = File::open(&args[1]).unwrap_or_else(|msg| {
        eprintln!("Application error: {}", msg);
        process::exit(1);
    });
    let reader = BufReader::new(file);
    let testsuites: TestSuitesOrTestSuite = serde_xml_rs::from_reader(reader).unwrap_or_else(|msg| {
        eprintln!("serde_xml_rs error: {}", msg);
        process::exit(1);
    });

    println!("{:#?}", testsuites);

    // file::write(&testsuites, &config).unwrap_or_else(|msg| {
    //     eprintln!("Write JSON error: {}", msg);
    //     process::exit(1);
    // });
}
