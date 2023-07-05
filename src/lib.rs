use serde::{Deserialize, Serialize};

// Reference JUnit XML Schema:
// - https://llg.cubic.org/docs/junit/
// - https://github.com/testmoapp/junitxml/tree/main
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct TestSuites {
    // #[serde(skip_serializing_if = "Option::is_none")]
    pub testsuite: Option<Vec<TestSuite>>,
    pub name: Option<String>,
    pub time: Option<f32>,
    pub tests: Option<u32>,
    pub failures: Option<u32>,
    pub errors: Option<u32>,
}
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct TestSuite {
    // #[serde(skip_serializing_if = "Option::is_none")]
    pub testcase: Option<Vec<TestCase>>,
    pub name: Option<String>,
    pub tests: Option<u32>,
    pub failures: Option<u32>,
    pub errors: Option<u32>,
    pub group: Option<String>,
    pub time: Option<f32>,
    pub disabled: Option<u32>,
    pub skipped: Option<u32>,
    pub timestamp: Option<String>,
    #[serde(rename = "system-out", skip_serializing_if = "Option::is_none")]
    pub system_out: Option<Vec<String>>,
    #[serde(rename = "system-err", skip_serializing_if = "Option::is_none")]
    pub system_err: Option<Vec<String>>,
    pub properties: Option<Vec<Property>>,
    pub hostname: Option<String>,
    pub id: Option<String>,
    pub package: Option<String>,
    pub file: Option<String>,
    pub log: Option<String>,
    pub url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct TestCase {
    pub name: Option<String>,
    pub classname: Option<String>,
    pub assertions: Option<u32>,
    pub time: Option<f32>,
    pub status: Option<String>,
    pub skipped: Option<Skipped>,
    pub error: Option<Details>,
    pub failure: Option<Details>,
    #[serde(rename = "system-out", skip_serializing_if = "Option::is_none")]
    pub system_out: Option<Vec<String>>,
    #[serde(rename = "system-err", skip_serializing_if = "Option::is_none")]
    pub system_err: Option<Vec<String>>,
    pub file: Option<String>,
    pub line: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Property {
    pub name: Option<String>,
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Skipped {
    pub message: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Details {
    pub message: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub _type: Option<String>,
    pub inner: Option<String>,
}
