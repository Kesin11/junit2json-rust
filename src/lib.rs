use serde::{Deserialize, Serialize};

// Return true if all struct in vec are equal to Default::default()
fn is_all_default<T: PartialEq + Default>(vec: &Vec<T>) -> bool {
    vec.iter().all(|s: &T| s == &T::default())
}


#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]

pub enum TestSuitesOrTestSuite {
    TestSuites(TestSuites),
    TestSuite(TestSuite),
}

// Reference JUnit XML Schema:
// - https://llg.cubic.org/docs/junit/
// - https://github.com/testmoapp/junitxml/tree/main
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct TestSuites {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub testsuite: Option<Vec<TestSuite>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tests: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failures: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<u32>,
}
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct TestSuite {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub testcase: Option<Vec<TestCase>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tests: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failures: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skipped: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,

    #[serde(rename = "system-out", skip_serializing_if = "Option::is_none")]
    pub system_out: Option<Vec<String>>,
    #[serde(rename = "system-err", skip_serializing_if = "Option::is_none")]
    pub system_err: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<Vec<Property>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub log: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct TestCase {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assertions: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skipped: Option<Skipped>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<Details>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure: Option<Details>,

    #[serde(rename = "system-out", skip_serializing_if = "Option::is_none")]
    pub system_out: Option<Vec<String>>,
    #[serde(rename = "system-err", skip_serializing_if = "Option::is_none")]
    pub system_err: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
pub struct Property {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Skipped {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Details {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename(deserialize = "$value"), skip_serializing_if = "Option::is_none")]
    pub inner: Option<String>,
}

#[cfg(test)]
mod tests {
    use crate::{Property, is_all_default};

    #[test]
    fn all_struct_in_vec_are_none() {
        let none_vec: Vec<Property> = vec![Property::default()];
        assert_eq!(is_all_default(&none_vec), true);
    }

    #[test]
    fn vec_is_empty() {
        let none_vec: Vec<Property> = vec![];
        assert_eq!(is_all_default(&none_vec), true);
    }

    #[test]
    fn some_struct_in_vec_are_not_none() {
        let none_vec: Vec<Property> = vec![Property::default(), Property { name: Some("name".to_string()), value: Some("value".to_string()) }];
        assert_eq!(is_all_default(&none_vec), false);
    }
}
