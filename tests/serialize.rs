use junit2json;
use pretty_assertions::assert_eq;
use serde_json::json;

#[test]
  // Test when testsuites.testsuite has some fields
fn testsuite_has_some_fields () {
  let xml = r#"
      <?xml version="1.0" encoding="UTF-8"?>
      <testsuites>
          <testsuite failures="1" tests="2">
          </testsuite>
      </testsuites>
  "#;
  let expect = json!({
      "testsuites": {
          "testsuite": [
              {
                  "failures": 1,
                  "tests": 2
              }
          ]
      }
  });

  let actual = junit2json::from_str(xml).unwrap();
  let actual_json_value = serde_json::to_value(actual).unwrap();
  assert_eq!(actual_json_value, expect);
}

  #[test]
  // Test when testcase.failure has inner text
  fn testcase_failure_has_inner() {
    let xml = r#"
        <?xml version="1.0" encoding="UTF-8"?>
        <testsuites>
            <testsuite tests="1">
                <testcase>
                    <failure>inner text</failure>
                </testcase>
            </testsuite>
        </testsuites>
    "#;
    let expect = json!({
        "testsuites": {
            "testsuite": [
                {
                    "tests": 1,
                    "testcase": [
                        {
                          "failure": {
                            "inner": "inner text"
                            }
                        }
                    ]
                }
            ]
        }
    });

    let actual = junit2json::from_str(xml).unwrap();
    let actual_json_value = serde_json::to_value(actual).unwrap();
    assert_eq!(actual_json_value, expect);
  }

  #[test]
  // Test when testcase has skiped test
  fn skipped_testcase() {
    let xml = r#"
        <?xml version="1.0" encoding="UTF-8"?>
        <testsuites>
            <testsuite tests="1" skipped="1">
                <testcase>
                    <skipped message="skip reason" />
                </testcase>
            </testsuite>
        </testsuites>
    "#;
    let expect = json!({
        "testsuites": {
            "testsuite": [
                {
                    "tests": 1,
                    "skipped": 1,
                    "testcase": [
                        {
                          "skipped": {
                            "message": "skip reason"
                            }
                        }
                    ]
                }
            ]
        }
    });

    let actual = junit2json::from_str(xml).unwrap();
    let actual_json_value = serde_json::to_value(actual).unwrap();
    assert_eq!(actual_json_value, expect);
  }

  #[test]
  // Test when testcase.system-out has inner text
  fn testcase_system_out_has_inner() {
    let xml = r#"
        <?xml version="1.0" encoding="UTF-8"?>
        <testsuites>
            <testsuite>
                <testcase>
                <system-out>system out text</system-out>
                </testcase>
            </testsuite>
        </testsuites>
    "#;
    let expect = json!({
      "testsuites": {
          "testsuite": [
              {
                  "testcase": [
                      {
                          "system-out": ["system out text"]
                      }
                  ]
              }
          ]
      }});

    let actual = junit2json::from_str(xml).unwrap();
    let actual_json_value = serde_json::to_value(actual).unwrap();
    assert_eq!(actual_json_value, expect);
  }

  #[test]
  // Test when testcase.system-err has inner text
  fn testcase_system_err_has_inner() {
    let xml = r#"
        <?xml version="1.0" encoding="UTF-8"?>
        <testsuites>
            <testsuite>
                <testcase>
                <system-err>system error text</system-err>
                </testcase>
            </testsuite>
        </testsuites>
    "#;
    let expect = json!({
      "testsuites": {
          "testsuite": [
              {
                  "testcase": [
                      {
                          "system-err": ["system error text"]
                      }
                  ]
              }
          ]
      }});
    
    let actual = junit2json::from_str(xml).unwrap();
    let actual_json_value = serde_json::to_value(actual).unwrap();
    assert_eq!(actual_json_value, expect);
  }

  #[test]
  // Test when testsuite.property is empty
  fn testsuite_property_is_empty() {
    let xml = r#"
        <?xml version="1.0" encoding="UTF-8"?>
        <testsuites>
            <testsuite>
                <properties />
            </testsuite>
        </testsuites>
    "#;
    let expect = json!({
      "testsuites": {
          "testsuite": [
              {}
          ]
      }});

    let actual = junit2json::from_str(xml).unwrap();
    let actual_json_value = serde_json::to_value(actual).unwrap();
    assert_eq!(actual_json_value, expect);
  }

  #[test]
  // Test when testsuite.property has some fields
  fn testsuite_property_has_some_fields() {
    let xml = r#"
        <?xml version="1.0" encoding="UTF-8"?>
        <testsuites>
            <testsuite>
                <properties>
                    <property name="hello" value="bonjour"/>
                    <property name="world" value="monde"/>
                </properties>
            </testsuite>
        </testsuites>
    "#;
    let expect = json!({
      "testsuites": {
          "testsuite": [
              {
                "properties": {
                  "property": [
                    {
                        "name": "hello",
                        "value": "bonjour"
                    },
                    {
                        "name": "world",
                        "value": "monde"
                    }
                  ]
                }
              }
          ]
      }});
    
    let actual = junit2json::from_str(xml).unwrap();
    let actual_json_value = serde_json::to_value(actual).unwrap();
    assert_eq!(actual_json_value, expect);
  }

  #[test]
  // Test when some testsuite.property are empty
  fn some_testsuite_property_are_empty() {
      let xml = r#"
          <?xml version="1.0" encoding="UTF-8"?>
          <testsuites>
              <testsuite>
                  <properties>
                      <property name="hello" value="bonjour"/>
                      <property/>
                  </properties>
              </testsuite>
          </testsuites>
      "#;

    let expect = json!({
      "testsuites": {
          "testsuite": [
              {
                "properties": {
                  "property": [
                    {
                        "name": "hello",
                        "value": "bonjour"
                    },
                  ]
                }
              }
          ]
      }});

    let actual = junit2json::from_str(xml).unwrap();
    let actual_json_value = serde_json::to_value(actual).unwrap();
    assert_eq!(actual_json_value, expect);
  }
