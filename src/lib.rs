//! # junit2json-rs
//!
//! junit2json-rs is a tool to convert JUnit XML format to JSON.
//! From a library perspective, it provides a function to serialize Junit XML to Struct.
//!
//! junit2json-rs is a reimplementation of [ts-junit2json](https://github.com/Kesin11/ts-junit2json) that is my previous work in TypeScript.
//!
//! # Purpose
//! junit2json-rs is designed for uploading test result data to BigQuery or any other DB that supports JSON.
//!
//! Many languages and test frameworks support to output test result data as JUnit XML format, which is de fact standard in today.
//! On the other hand, most DBs do not support to import XML but support JSON.
//!
//! For this purpose, junit2json-rs provides a simple JUnit XML to JSON converter.
//!
//! # Install
//! ```
//! cargo install junit2json
//! ```
//!
//! # Usage
//! ```
//! junit2json -p <junit_xml_file>
//! ```
//!
//! # Output example
//! ```json
//! {
//!   "testsuites": {
//!     "name": "gcf_junit_xml_to_bq_dummy",
//!     "time": 8.018,
//!     "tests": 12,
//!     "failures": 2,
//!     "testsuite": [
//!       {
//!         "name": "__tests__/gen_dummy_junit/dummy1.test.js",
//!         "tests": 4,
//!         "failures": 1,
//!         "errors": 0,
//!         "time": 4.772,
//!         "skipped": 0,
//!         "timestamp": "2020-01-12T16:33:13",
//!         "testcase": [
//!           {
//!             "name": "dummy1 Always success tests should be wait 0-2sec",
//!             "classname": "dummy1 Always success tests should be wait 0-2sec",
//!             "time": 0.414
//!           },
//!           {
//!             "name": "dummy1 Always success tests should be wait 1-3sec",
//!             "classname": "dummy1 Always success tests should be wait 1-3sec",
//!             "time": 1.344
//!           },
//!           {
//!             "name": "dummy1 Randomly fail tests should be wait 0-1sec and fail 50%",
//!             "classname": "dummy1 Randomly fail tests should be wait 0-1sec and fail 50%",
//!             "time": 0.673,
//!             "failure": {
//!               "inner": "Error: expect(received).toBeGreaterThan(expected)\n\nExpected: > 50\nReceived:   4.897277513425746\n    at Object.it (/Users/kesin/github/gcf_junit_xml_to_bq/__tests__/gen_dummy_junit/dummy1.test.js:22:17)"
//!             }
//!           },
//!           {
//!             "name": "dummy1 Randomly fail tests should be wait 1-2sec and fail 30%",
//!             "classname": "dummy1 Randomly fail tests should be wait 1-2sec and fail 30%",
//!             "time": 1.604
//!           }
//!         ]
//!       },
//!       {
//!         "name": "__tests__/gen_dummy_junit/dummy3.test.js",
//!         "tests": 4,
//!         "failures": 1,
//!         "errors": 0,
//!         "time": 6.372,
//!         "skipped": 0,
//!         "timestamp": "2020-01-12T16:33:13",
//!         "testcase": [
//!           {
//!             "name": "dummy3 Always success tests should be wait 0-2sec",
//!             "classname": "dummy3 Always success tests should be wait 0-2sec",
//!             "time": 1.328
//!           },
//!           {
//!             "name": "dummy3 Always success tests should be wait 1-3sec",
//!             "classname": "dummy3 Always success tests should be wait 1-3sec",
//!             "time": 2.598
//!           },
//!           {
//!             "name": "dummy3 Randomly fail tests should be wait 0-1sec and fail 30%",
//!             "classname": "dummy3 Randomly fail tests should be wait 0-1sec and fail 30%",
//!             "time": 0.455,
//!             "failure": {
//!               "inner": "Error: expect(received).toBeGreaterThan(expected)\n\nExpected: > 30\nReceived:   12.15901879426653\n    at Object.it (/Users/kesin/github/gcf_junit_xml_to_bq/__tests__/gen_dummy_junit/dummy3.test.js:22:17)"
//!             }
//!           },
//!           {
//!             "name": "dummy3 Randomly fail tests should be wait 1-2sec and fail 20%",
//!             "classname": "dummy3 Randomly fail tests should be wait 1-2sec and fail 20%",
//!             "time": 1.228
//!           }
//!         ]
//!       }
//!     ]
//!   }
//! }
//! ```
//!
//! # With `jq` examples
//! Show testsuites test count
//!
//! ```
//! junit2json <junit_xml_file> | jq .testsuites.tests
//! ```
//!
//! Show testsuite names
//!
//! ```
//! junit2json <junit_xml_file> | jq .testsuites.testsuite[].name
//! ```
//!
//! Show testcase classnames
//!
//! ```
//! junit2json <junit_xml_file> | jq .testsuites.testsuite[].testcase[].classname
//! ```
//!
//! # Notice
//! junit2json-rs has some major changes from ts-junit2json.
//! Most of the changes are to compliant with the JUnit XML Schema.
//!
//! - A `testsuites` or `testsuite` key appears in the root of JSON.
//! - `properties` has `property` array. ts-junit2json has `property` array of object directly.
//! - `skipped`, `error`, `failure` are object, not array of object.
//! - If XML has undefined tag, it will be ignored. ts-junit2json will be converted to JSON if possible.
//!
//! Referenced JUnit XML Schema:
//! - <https://llg.cubic.org/docs/junit/>
//! - <https://github.com/testmoapp/junitxml/tree/main>
//!
//! # WASI
//! junit2json-rs also provides WASI executable.
//!
//! If you have wasm runtime (ex. wasmtime), you can execute `junit2json.wasm` that can download from [GitHub Releases](https://github.com/Kesin11/junit2json-rs/releases) instead of native binary.
//!
//! ```
//! wasmtime junit2json.wasm --dir=. -- -p <junit_xml_file>
//! ```
//!

use cli::PossibleFilterTags;
use quick_xml::de;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::default;
use std::io;

pub mod cli;

fn trim_default_items<T: default::Default + PartialEq + Clone>(vec: &mut Option<Vec<T>>) {
    match vec {
        Some(v) => {
            *vec = v
                .iter()
                .filter(|&item| item != &Default::default())
                .cloned()
                .collect::<Vec<_>>()
                .into();
        }
        None => {}
    }
}

/// It corresponds to `<testsuites> or <testsuite>`
///
/// ```xml
/// <testsuites name="testsuites1" tests=1 time=0.1>
///     <tetssuite>
///     </testsuite>
/// </testsuites>
/// ```
///
/// ```xml
/// <testsuite name="testsuite1" tests=1 time=0.1>
/// </testsuite>
/// ```
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TestSuitesOrTestSuite {
    TestSuites(TestSuites),
    TestSuite(Box<TestSuite>),
}
impl TestSuitesOrTestSuite {
    /// Remove all `system-out` and `system-err` from each `testsuite` and `testcase`.
    ///
    /// # Examples
    /// ```
    /// use junit2json;
    ///
    /// let xml = r#"
    ///   <?xml version="1.0" encoding="UTF-8"?>
    ///   <testsuites>
    ///       <testsuite name="suite1">
    ///           <system-out>system out text</system-out>
    ///           <system-err>system error text</system-err>
    ///           <testcase name="case1">
    ///             <system-out>system out text</system-out>
    ///             <system-err>system error text</system-err>
    ///           </testcase>
    ///       </testsuite>
    ///   </testsuites>
    /// "#;
    /// let mut testsuites = junit2json::from_str(xml).unwrap();
    /// testsuites.filter_tags(&vec![
    ///   junit2json::cli::PossibleFilterTags::SystemOut,
    ///   junit2json::cli::PossibleFilterTags::SystemErr,
    /// ]);
    /// println!("{:#?}", testsuites);
    /// ```
    pub fn filter_tags(&mut self, tags: &[PossibleFilterTags]) {
        match self {
            TestSuitesOrTestSuite::TestSuites(ref mut testsuites) => {
                testsuites.filter_tags(tags);
            }
            TestSuitesOrTestSuite::TestSuite(ref mut testsuite) => {
                testsuite.filter_tags(tags);
            }
        }
    }
}

/// It corresponds to `<testsuites>`
///
/// ```xml
/// <testsuites name="testsuites1" tests=1 time=0.1>
/// </testsuites>
/// ```
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
pub struct TestSuites {
    #[serde(rename(deserialize = "@name"))]
    pub name: Option<String>,
    #[serde(rename(deserialize = "@time"))]
    pub time: Option<f32>,
    #[serde(rename(deserialize = "@tests"))]
    pub tests: Option<u32>,
    #[serde(rename(deserialize = "@failures"))]
    pub failures: Option<u32>,
    #[serde(rename(deserialize = "@errors"))]
    pub errors: Option<u32>,

    pub testsuite: Option<Vec<TestSuite>>,
}
impl TestSuites {
    pub fn trim_empty_items(&mut self) {
        match &mut self.testsuite {
            Some(testsuite) => testsuite
                .iter_mut()
                .for_each(|item| item.trim_empty_items()),
            None => {}
        }
    }
    pub fn filter_tags(&mut self, tags: &[PossibleFilterTags]) {
        match &mut self.testsuite {
            Some(testsuite) => testsuite.iter_mut().for_each(|item| item.filter_tags(tags)),
            None => {}
        }
    }
}

/// It corresponds to `<testsuite>`
///
/// ```xml
/// <testsuite name="testsuite1" tests=1 time=0.1>
/// </testsuite>
/// ```
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
pub struct TestSuite {
    #[serde(rename(deserialize = "@name"))]
    pub name: Option<String>,
    #[serde(rename(deserialize = "@tests"))]
    pub tests: Option<u32>,
    #[serde(rename(deserialize = "@failures"))]
    pub failures: Option<u32>,
    #[serde(rename(deserialize = "@errors"))]
    pub errors: Option<u32>,
    #[serde(rename(deserialize = "@group"))]
    pub group: Option<String>,
    #[serde(rename(deserialize = "@time"))]
    pub time: Option<f32>,
    #[serde(rename(deserialize = "@disabled"))]
    pub disabled: Option<u32>,
    #[serde(rename(deserialize = "@skipped"))]
    pub skipped: Option<u32>,
    #[serde(rename(deserialize = "@timestamp"))]
    pub timestamp: Option<String>,
    #[serde(rename(deserialize = "@hostname"))]
    pub hostname: Option<String>,
    #[serde(rename(deserialize = "@id"))]
    pub id: Option<String>,
    #[serde(rename(deserialize = "@package"))]
    pub package: Option<String>,
    #[serde(rename(deserialize = "@file"))]
    pub file: Option<String>,
    #[serde(rename(deserialize = "@log"))]
    pub log: Option<String>,
    #[serde(rename(deserialize = "@url"))]
    pub url: Option<String>,

    #[serde(rename = "system-out")]
    pub system_out: Option<Vec<String>>,
    #[serde(rename = "system-err")]
    pub system_err: Option<Vec<String>>,
    pub properties: Option<Properties>,
    pub testcase: Option<Vec<TestCase>>,
}
impl TestSuite {
    pub fn trim_empty_items(&mut self) {
        trim_default_items(&mut self.system_out);
        trim_default_items(&mut self.system_err);

        match &mut self.properties {
            Some(properties) => {
                properties.trim_empty_items();
                if properties.property.is_none() {
                    self.properties = None;
                }
            }
            None => {}
        }
        match &mut self.testcase {
            Some(testcase) => testcase.iter_mut().for_each(|item| item.trim_empty_items()),
            None => {}
        }
    }
    pub fn filter_tags(&mut self, tags: &[PossibleFilterTags]) {
        for tag in tags.iter() {
            match tag {
                PossibleFilterTags::SystemOut => self.system_out = None,
                PossibleFilterTags::SystemErr => self.system_err = None,
            }
        }
        match &mut self.testcase {
            Some(testcase) => testcase.iter_mut().for_each(|item| item.filter_tags(tags)),
            None => {}
        }
    }
}

/// It corresponds to `<testcase>`
///
/// ```xml
/// <testcase name="testcase1" time=0.1>
/// </testcase>
/// ```
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
pub struct TestCase {
    #[serde(rename(deserialize = "@name"))]
    pub name: Option<String>,
    #[serde(rename(deserialize = "@classname"))]
    pub classname: Option<String>,
    #[serde(rename(deserialize = "@assertions"))]
    pub assertions: Option<u32>,
    #[serde(rename(deserialize = "@time"))]
    pub time: Option<f32>,
    #[serde(rename(deserialize = "@status"))]
    pub status: Option<String>,
    #[serde(rename(deserialize = "@file"))]
    pub file: Option<String>,
    #[serde(rename(deserialize = "@line"))]
    pub line: Option<u32>,

    #[serde(rename = "system-out")]
    pub system_out: Option<Vec<String>>,
    #[serde(rename = "system-err")]
    pub system_err: Option<Vec<String>>,
    pub skipped: Option<Detail>,
    pub error: Option<Detail>,
    pub failure: Option<Detail>,
}
impl TestCase {
    pub fn trim_empty_items(&mut self) {
        trim_default_items(&mut self.system_out);
        trim_default_items(&mut self.system_err);
    }
    pub fn filter_tags(&mut self, tags: &[PossibleFilterTags]) {
        for tag in tags.iter() {
            match tag {
                PossibleFilterTags::SystemOut => self.system_out = None,
                PossibleFilterTags::SystemErr => self.system_err = None,
            }
        }
    }
}

/// It corresponds to `<skipped>, <error>, <failure>`
///
/// ```xml
/// <testcase>
///    <skipped message="foo" type="bar">Skipped</skipped>
///    <error message="foo" type="bar">Error</error>
///    <failure message="foo" type="bar">Failure</failure>
/// </testcase>
/// ```
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
pub struct Detail {
    #[serde(rename(deserialize = "@message"))]
    pub message: Option<String>,
    #[serde(rename(deserialize = "@type"))]
    pub r#type: Option<String>,
    #[serde(rename(deserialize = "$value"))]
    pub inner: Option<String>,
}

/// It corresponds to `<properties>`
///
/// ```xml
/// <properties>
///    <property name="foo" value="bar" />
/// </properties>
/// ```
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
pub struct Properties {
    pub property: Option<Vec<Property>>,
}
impl Properties {
    pub fn trim_empty_items(&mut self) {
        trim_default_items(&mut self.property);
    }
}

/// It corresponds to `<property>`
///
/// ```xml
/// <property name="foo" value="bar" />
/// ```
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Clone)]
pub struct Property {
    #[serde(rename(deserialize = "@name"))]
    pub name: Option<String>,
    #[serde(rename(deserialize = "@value"))]
    pub value: Option<String>,
}

/// Deserialize JUnit XML from a reader.
///
/// # Examples
/// ```
/// use junit2json;
/// use std::process;
/// use std::fs::File;
/// use std::io::BufReader;
///
/// let path = "tests/fixtures/cargo-nextest.xml";
/// let file = File::open(path).unwrap_or_else(|msg| {
///     eprintln!("File::open error: {}", msg);
///     process::exit(1);
/// });
/// let reader = BufReader::new(file);
/// let testsuites = junit2json::from_reader(reader).unwrap_or_else(|msg| {
///     eprintln!("junit2json::from_reader error: {}", msg);
///     process::exit(1);
/// });
/// println!("{:#?}", testsuites);
/// ```
pub fn from_reader<T>(reader: io::BufReader<T>) -> Result<TestSuitesOrTestSuite, quick_xml::DeError>
where
    T: io::Read,
{
    let mut root: TestSuitesOrTestSuite = de::from_reader(reader)?;
    match root {
        TestSuitesOrTestSuite::TestSuites(ref mut testsuites) => testsuites.trim_empty_items(),
        TestSuitesOrTestSuite::TestSuite(ref mut testsuite) => testsuite.trim_empty_items(),
    }
    Ok(root)
}

/// Deserialize JUnit XML from a string.
///
/// # Examples
/// ```
/// use junit2json;
/// use std::process;
///
/// let xml = r#"
///     <?xml version="1.0" encoding="UTF-8"?>
///     <testsuites>
///         <testsuite failures="1" tests="2">
///         </testsuite>
///     </testsuites>
/// "#;
/// let testsuites = junit2json::from_str(xml).unwrap_or_else(|msg| {
///     eprintln!("junit2json::from_str error: {}", msg);
///     process::exit(1);
/// });
/// println!("{:#?}", testsuites);
/// ```
pub fn from_str(s: &str) -> Result<TestSuitesOrTestSuite, quick_xml::DeError> {
    from_reader(io::BufReader::new(s.as_bytes()))
}
