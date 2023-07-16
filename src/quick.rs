use std::default;
use std::io;
use std::fs;
use quick_xml;
use quick_xml::de;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum TestSuitesOrTestSuite {
    TestSuites(TestSuites),
    TestSuite(TestSuite),
}

// Reference JUnit XML Schema:
// - https://llg.cubic.org/docs/junit/
// - https://github.com/testmoapp/junitxml/tree/main
#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Default)]
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

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Default)]
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
    pub properties: Option<Vec<Property>>,
    pub testcase: Option<Vec<TestCase>>,
}
impl TestSuite {
    pub fn trim_empties(&mut self) {
        trim_default_items(&mut self.properties)
    }
}

fn trim_default_items<T: default::Default + PartialEq + Clone>(vec: &mut Option<Vec<T>>) {
    match vec {
        Some(v) => {
            *vec = v
                .iter()
                .filter(|&item| item != &Default::default())
                .map(|item| item.clone())
                .collect::<Vec<_>>()
                .into();
        },
        None => {},
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Default)]
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
    pub skipped: Option<Skipped>,
    pub error: Option<Details>,
    pub failure: Option<Details>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Skipped {
    #[serde(rename(deserialize = "@message"))]
    pub message: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Details {
    #[serde(rename(deserialize = "@message"))]
    pub message: Option<String>,
    #[serde(rename(deserialize = "@type"))]
    pub r#type: Option<String>,
    #[serde(rename(deserialize = "$value"))]
    pub inner: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Default, PartialEq, Clone)]
pub struct Property {
    #[serde(rename(deserialize = "@name"))]
    pub name: Option<String>,
    #[serde(rename(deserialize = "@value"))]
    pub value: Option<String>,
}

pub fn from_reader(reader: io::BufReader<fs::File>) -> Result<TestSuitesOrTestSuite, quick_xml::DeError> {
    let mut root: TestSuitesOrTestSuite = de::from_reader(reader)?;
    match root {
        TestSuitesOrTestSuite::TestSuite(ref mut ts) => { ts.trim_empties() },
        (_) => {},
    }
    Ok(root)
}
