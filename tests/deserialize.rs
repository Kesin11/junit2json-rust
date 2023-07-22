  use junit2json::*;
  use pretty_assertions::assert_eq;

  #[test]
  // Test when input is not JUnit XML
  fn test_not_junit_xml() {
      let xml = r#"
          <?xml version="1.0" encoding="UTF-8"?>
          <unrecognized />
      "#;
      let actual = from_str(xml);
      assert!(actual.is_err());
  }

  #[test]
  // Test when all testsuites fields are absent
  fn test_testsuites_properties_are_absent() {
      let xml = r#"
          <?xml version="1.0" encoding="UTF-8"?>
          <testsuites>
          </testsuites>
      "#;
      let actual = from_str(xml);
      assert_eq!(actual.unwrap(), TestSuitesOrTestSuite::TestSuites(
          TestSuites { ..Default::default() }
      ));
  }

  #[test]
  // Test when testsuites.testsuite has some fields
  fn test_testsuite_has_some_fields() {
      let xml = r#"
          <?xml version="1.0" encoding="UTF-8"?>
          <testsuites>
              <testsuite failures="1" tests="2">
              </testsuite>
          </testsuites>
      "#;
      let actual = from_str(xml);
      assert_eq!(actual.unwrap(), TestSuitesOrTestSuite::TestSuites( TestSuites {
          testsuite: Some(vec![TestSuite {
              failures: Some(1),
              tests: Some(2),
              ..Default::default()
          }]),
          ..Default::default()
      }));
  }

  #[test]
  // Test when testcase.failure has inner text
  fn test_testcase_failure_has_inner() {
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
      let actual = from_str(xml);
      assert_eq!(actual.unwrap(), TestSuitesOrTestSuite::TestSuites( TestSuites {
          testsuite: Some(vec![TestSuite {
              tests: Some(1),
              testcase: Some(vec![TestCase {
                  failure: Some(Detail {
                      inner: Some("inner text".to_string()),
                      ..Default::default()
                  }),
              ..Default::default()
              }]),
          ..Default::default()
          }]),
      ..Default::default()
      }));
  }

  #[test]
  // Test when testcase has skiped test
  fn test_skipped_testcase() {
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
      let actual = from_str(xml);
      assert_eq!(actual.unwrap(), TestSuitesOrTestSuite::TestSuites( TestSuites {
          testsuite: Some(vec![TestSuite {
              tests: Some(1),
              skipped: Some(1),
              testcase: Some(vec![TestCase {
                  skipped: Some(Detail {
                      message: Some("skip reason".to_string()),
                      ..Default::default()
                  }),
              ..Default::default()
              }]),
          ..Default::default()
          }]),
      ..Default::default()
      }));
  }

  #[test]
  // Test when testcase.system-out has inner text
  fn test_testcase_system_out_has_inner() {
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
      let actual = from_str(xml);
      assert_eq!(actual.unwrap(), TestSuitesOrTestSuite::TestSuites( TestSuites {
          testsuite: Some(vec![TestSuite {
              testcase: Some(vec![TestCase {
                  system_out: Some(vec!["system out text".to_string()]),
              ..Default::default()
              }]),
          ..Default::default()
          }]),
      ..Default::default()
      }));
  }

  #[test]
  // Test when testcase.system-err has inner text
  fn test_testcase_system_err_has_inner() {
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
      let actual = from_str(xml);
      assert_eq!(actual.unwrap(), TestSuitesOrTestSuite::TestSuites( TestSuites {
          testsuite: Some(vec![TestSuite {
              testcase: Some(vec![TestCase {
                  system_err: Some(vec!["system error text".to_string()]),
              ..Default::default()
              }]),
          ..Default::default()
          }]),
      ..Default::default()
      }));
  }

  #[test]
  // Test when testsuite.property is empty
  fn test_testsuite_property_is_empty() {
      let xml = r#"
          <?xml version="1.0" encoding="UTF-8"?>
          <testsuites>
              <testsuite>
                  <properties />
              </testsuite>
          </testsuites>
      "#;
      let actual = from_str(xml);
      assert_eq!(actual.unwrap(), TestSuitesOrTestSuite::TestSuites( TestSuites {
          testsuite: Some(vec![TestSuite {
              properties: None,
          ..Default::default()
          }]),
      ..Default::default()
      }));
  }

  #[test]
  // Test when testsuite.property has some fields
  fn test_testsuite_property_has_some_fields() {
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
      let actual = from_str(xml);
      assert_eq!(actual.unwrap(), TestSuitesOrTestSuite::TestSuites( TestSuites {
          testsuite: Some(vec![TestSuite {
              properties: Some(Properties {
                  property: Some(vec![
                      Property {
                          name: Some("hello".to_string()),
                          value: Some("bonjour".to_string()),
                          ..Default::default()
                      }, Property {
                          name: Some("world".to_string()),
                          value: Some("monde".to_string()),
                          ..Default::default()
                      }]),
                  }),
          ..Default::default()
          }]),
      ..Default::default()
      }));
  }

  #[test]
  // Test when some testsuite.property are empty
  fn test_some_testsuite_property_are_empty() {
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
      let actual = from_str(xml);
      assert_eq!(actual.unwrap(), TestSuitesOrTestSuite::TestSuites( TestSuites {
          testsuite: Some(vec![TestSuite {
              properties: Some(Properties {
                  property: Some(vec![
                      Property {
                          name: Some("hello".to_string()),
                          value: Some("bonjour".to_string()),
                          ..Default::default()
                      }]),
                  }),
          ..Default::default()
          }]),
      ..Default::default()
      }));
  }
