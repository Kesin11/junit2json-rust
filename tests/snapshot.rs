use std::fs::File;
use std::io::BufReader;

#[test]
fn fixtures_snapshot() {
    insta::glob!("fixtures/*.xml", |path| {
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        let testsuites = junit2json::from_reader(reader).unwrap();

        insta::assert_json_snapshot!(testsuites)
    })
}
