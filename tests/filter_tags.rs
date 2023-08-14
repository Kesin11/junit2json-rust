use junit2json::*;
use pretty_assertions::assert_eq;

fn create_fixture() -> &'static str {
    r#"
      <?xml version="1.0" encoding="UTF-8"?>
      <testsuites>
          <testsuite name="suite1">
              <system-out>system out text</system-out>
              <system-err>system error text</system-err>
              <testcase name="case1">
                <system-out>system out text</system-out>
                <system-err>system error text</system-err>
              </testcase>
          </testsuite>
      </testsuites>
  "#
}

#[test]
/// Test when --filter-tags=system-out
fn filter_system_out() {
    let xml = create_fixture();
    let mut actual = from_str(xml).unwrap();
    actual.filter_tags(&vec![junit2json::cli::PossibleFilterTags::SystemOut]);

    assert_eq!(
        actual,
        TestSuitesOrTestSuite::TestSuites(TestSuites {
            testsuite: Some(vec![TestSuite {
                name: Some("suite1".to_string()),
                system_err: Some(vec!["system error text".to_string()]),
                testcase: Some(vec![TestCase {
                    name: Some("case1".to_string()),
                    system_err: Some(vec!["system error text".to_string()]),
                    ..Default::default()
                }]),
                ..Default::default()
            }]),
            ..Default::default()
        })
    );
}

#[test]
/// Test when --filter-tags=system-err
fn filter_system_err() {
    let xml = create_fixture();
    let mut actual = from_str(xml).unwrap();
    actual.filter_tags(&vec![junit2json::cli::PossibleFilterTags::SystemErr]);

    assert_eq!(
        actual,
        TestSuitesOrTestSuite::TestSuites(TestSuites {
            testsuite: Some(vec![TestSuite {
                name: Some("suite1".to_string()),
                system_out: Some(vec!["system out text".to_string()]),
                testcase: Some(vec![TestCase {
                    name: Some("case1".to_string()),
                    system_out: Some(vec!["system out text".to_string()]),
                    ..Default::default()
                }]),
                ..Default::default()
            }]),
            ..Default::default()
        })
    );
}

#[test]
/// Test when --filter-tags=system-out --filter-tags=system-err
fn filter_system_out_and_err() {
    let xml = create_fixture();
    let mut actual = from_str(xml).unwrap();
    actual.filter_tags(&vec![
        junit2json::cli::PossibleFilterTags::SystemOut,
        junit2json::cli::PossibleFilterTags::SystemErr,
    ]);

    assert_eq!(
        actual,
        TestSuitesOrTestSuite::TestSuites(TestSuites {
            testsuite: Some(vec![TestSuite {
                name: Some("suite1".to_string()),
                testcase: Some(vec![TestCase {
                    name: Some("case1".to_string()),
                    ..Default::default()
                }]),
                ..Default::default()
            }]),
            ..Default::default()
        })
    );
}
