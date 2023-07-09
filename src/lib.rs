use serde::{Deserialize, Serialize, Serializer};
use serde_with::skip_serializing_none;

// Return true if all struct in vec are equal to Default::default()
// fn is_all_default<T>(vec: &T) -> bool
// where
//     T: Iterator,
//     <T as Iterator>::Item: Default + PartialEq,
// {
//     vec.into_iter().all(|s| s == Default::default())
// }

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
    pub testsuite: Option<Vec<TestSuite>>,
    pub name: Option<String>,
    pub time: Option<f32>,
    pub tests: Option<u32>,
    pub failures: Option<u32>,
    pub errors: Option<u32>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct TestSuite {
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

    #[serde(rename = "system-out")]
    pub system_out: Option<Vec<String>>,
    #[serde(rename = "system-err")]
    pub system_err: Option<Vec<String>>,

    // #[serde(skip_serializing_if = "is_all_default")]
    pub properties: Option<Vec<Property>>,
    pub hostname: Option<String>,
    pub id: Option<String>,
    pub package: Option<String>,
    pub file: Option<String>,
    pub log: Option<String>,
    pub url: Option<String>,
}

#[skip_serializing_none]
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

    #[serde(rename = "system-out")]
    pub system_out: Option<Vec<String>>,
    #[serde(rename = "system-err")]
    pub system_err: Option<Vec<String>>,

    pub file: Option<String>,
    pub line: Option<u32>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
pub struct Property {
    pub name: Option<String>,
    pub value: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Skipped {
    pub message: Option<String>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Details {
    pub message: Option<String>,
    pub r#type: Option<String>,
    #[serde(rename(deserialize = "$value"))]
    pub inner: Option<String>,
}

// #[cfg(test)]
// mod tests {
//     use crate::{Property, is_all_default};

//     #[test]
//     fn all_struct_in_vec_are_none() {
//         let none_vec: Vec<Property> = vec![Property::default()];
//         assert_eq!(is_all_default(&none_vec), true);
//     }

//     #[test]
//     fn vec_is_empty() {
//         let none_vec: Vec<Property> = vec![];
//         assert_eq!(is_all_default(&none_vec), true);
//     }

//     #[test]
//     fn some_struct_in_vec_are_not_none() {
//         let none_vec: Vec<Property> = vec![Property::default(), Property { name: Some("name".to_string()), value: Some("value".to_string()) }];
//         assert_eq!(is_all_default(&none_vec), false);
//     }
// }
