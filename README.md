# junit2json-rs
[![CI](https://github.com/Kesin11/junit2json-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/Kesin11/junit2json-rs/actions/workflows/ci.yml)

junit2json-rs is a tool to convert JUnit XML format to JSON.
From a library perspective, it provides a function to serialize Junit XML to Struct.

junit2json-rs is a reimplementation of [ts-junit2json](https://github.com/Kesin11/ts-junit2json) that is my previous work in TypeScript.

# Purpose
junit2json-rs is designed for uploading test result data to BigQuery or any other DB that supports JSON.

Many languages and test frameworks support to output test result data as JUnit XML format, which is de fact standard in today.
On the other hand, most DBs do not support to import XML but support JSON.

For this purpose, junit2json-rs provides a simple JUnit XML to JSON converter.

# Install
```
cargo install junit2json-rs
```

# Usage
```
junit2json-rs --pretry <junit_xml_file>
```

# Output example
```json
{
  "testsuites": {
    "name": "gcf_junit_xml_to_bq_dummy",
    "time": 8.018,
    "tests": 12,
    "failures": 2,
    "testsuite": [
      {
        "name": "__tests__/gen_dummy_junit/dummy1.test.js",
        "tests": 4,
        "failures": 1,
        "errors": 0,
        "time": 4.772,
        "skipped": 0,
        "timestamp": "2020-01-12T16:33:13",
        "testcase": [
          {
            "name": "dummy1 Always success tests should be wait 0-2sec",
            "classname": "dummy1 Always success tests should be wait 0-2sec",
            "time": 0.414
          },
          {
            "name": "dummy1 Always success tests should be wait 1-3sec",
            "classname": "dummy1 Always success tests should be wait 1-3sec",
            "time": 1.344
          },
          {
            "name": "dummy1 Randomly fail tests should be wait 0-1sec and fail 50%",
            "classname": "dummy1 Randomly fail tests should be wait 0-1sec and fail 50%",
            "time": 0.673,
            "failure": {
              "inner": "Error: expect(received).toBeGreaterThan(expected)\n\nExpected: > 50\nReceived:   4.897277513425746\n    at Object.it (/Users/kesin/github/gcf_junit_xml_to_bq/__tests__/gen_dummy_junit/dummy1.test.js:22:17)"
            }
          },
          {
            "name": "dummy1 Randomly fail tests should be wait 1-2sec and fail 30%",
            "classname": "dummy1 Randomly fail tests should be wait 1-2sec and fail 30%",
            "time": 1.604
          }
        ]
      },
      {
        "name": "__tests__/gen_dummy_junit/dummy3.test.js",
        "tests": 4,
        "failures": 1,
        "errors": 0,
        "time": 6.372,
        "skipped": 0,
        "timestamp": "2020-01-12T16:33:13",
        "testcase": [
          {
            "name": "dummy3 Always success tests should be wait 0-2sec",
            "classname": "dummy3 Always success tests should be wait 0-2sec",
            "time": 1.328
          },
          {
            "name": "dummy3 Always success tests should be wait 1-3sec",
            "classname": "dummy3 Always success tests should be wait 1-3sec",
            "time": 2.598
          },
          {
            "name": "dummy3 Randomly fail tests should be wait 0-1sec and fail 30%",
            "classname": "dummy3 Randomly fail tests should be wait 0-1sec and fail 30%",
            "time": 0.455,
            "failure": {
              "inner": "Error: expect(received).toBeGreaterThan(expected)\n\nExpected: > 30\nReceived:   12.15901879426653\n    at Object.it (/Users/kesin/github/gcf_junit_xml_to_bq/__tests__/gen_dummy_junit/dummy3.test.js:22:17)"
            }
          },
          {
            "name": "dummy3 Randomly fail tests should be wait 1-2sec and fail 20%",
            "classname": "dummy3 Randomly fail tests should be wait 1-2sec and fail 20%",
            "time": 1.228
          }
        ]
      }
    ]
  }
}
```

# With `jq` examples
Show testsuites test count

```
junit2json-rs --pretry <junit_xml_file> | jq .testsuites.tests
```

Show testsuite names

```
junit2json-rs --pretry <junit_xml_file> | jq .testsuites.testsuite[].name
```

Show testcase classnames

```
npx junit2json junit.xml | jq .testsuites.testsuite[].testcase[].classname
```

# Notice
> [!IMPORTANT]
> junit2json-rs has some major changes from ts-junit2json.
> Most of the changes are to compliant with the JUnit XML Schema.

- A `testsuites` or `testsuite` key appears in the root of JSON.
- `properties` has `property` array. ts-junit2json has `property` array of object directly.
- `skipped`, `error`, `failure` are object, not array of object.
- If XML has undefined tag, it will be ignored. ts-junit2json will be converted to JSON if possible.

Referenced JUnit XML Schema:
- <https://llg.cubic.org/docs/junit/>
- <https://github.com/testmoapp/junitxml/tree/main>

# CLI Options
```
A tool convert JUnit XML format to JSON with Rust

Usage: junit2json [OPTIONS] <PATH>

Arguments:
  <PATH>  JUnit XML path

Options:
  -p, --pretty                     Output pretty JSON
  -f, --filter-tags <FILTER_TAGS>  Filter XML tag names [possible values: system-out, system-err]
  -h, --help                       Print help
  -V, --version                    Print version
```

# Development
## Setup
You can use DevContainer or Codespaces. Please see [devcontainer.json](./.devcontainer/devcontainer.json).

## Build
```bash
cargo build
cargo build --release
```

## Test
```bash
# Run test
cargo nextest run

# Update snapshot
## Need to install cargo-insta first.
## `cargo install cargo-insta`
cargo insta review
```

# License
MIT
