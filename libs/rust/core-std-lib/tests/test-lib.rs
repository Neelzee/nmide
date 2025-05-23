#[cfg(test)]
mod get_test_data {
    use serde::{Deserialize, Serialize};
    use serde_json::Value as Val;

    pub struct TestSuite {
        pub suite_name: String,
        pub tests: Vec<TestStruct>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct TestStruct {
        pub name: String,
        pub input: Val,
    }

    pub fn get_test_data(content: &str) -> Vec<TestSuite> {
        let dt = serde_json::from_str::<Val>(content)
            .expect("Serialization should succeed")
            .as_object()
            .cloned()
            .expect("Data should always be object");
        dt.keys()
            .map(|k| {
                let v = dt.get(k).unwrap();
                TestSuite {
                    suite_name: k.clone(),
                    tests: serde_json::from_value::<Vec<TestStruct>>(v.clone()).unwrap(),
                }
            })
            .collect()
    }

    pub struct TestInstrSuite {
        pub suite_name: String,
        pub tests: Vec<TestInstrStruct>,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct TestInstrStruct {
        pub name: String,
        pub kind: Option<String>,
        pub input: Val,
    }

    pub fn get_instr_test_data() -> Vec<TestInstrSuite> {
        let content = include_str!("../../../test-libs/test-data/instr-test-data.json");
        let dt = serde_json::from_str::<Val>(content)
            .expect("Serialization should succeed")
            .as_object()
            .cloned()
            .expect("Data should always be object");
        dt.keys()
            .map(|k| {
                let v = dt.get(k).unwrap();

                TestInstrSuite {
                    suite_name: k.clone(),
                    tests: serde_json::from_value::<Vec<TestInstrStruct>>(v.clone())
                        .inspect_err(|e| panic!("Failed to parse data: {e:?}, input: {v:?}"))
                        .unwrap(),
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod serialization_tests {
    use crate::get_test_data::{get_instr_test_data, get_test_data};
    use core_std_lib::{
        attrs::Attr, event::Event, html::Html, instruction::inst::Instruction, state::Value,
    };

    #[test]
    fn attr_test() {
        let content = include_str!("../../../test-libs/test-data/attr-test-data.json");
        for suite in get_test_data(content) {
            suite
                .tests
                .into_iter()
                .for_each(|t| match serde_json::from_value::<Attr>(t.input) {
                    Ok(e) => assert!(e == e),
                    Err(err) => panic!(
                        "Failed serialization: {:?} {} {}",
                        err, suite.suite_name, t.name
                    ),
                });
        }
    }

    #[test]
    fn event_test() {
        let content = include_str!("../../../test-libs/test-data/event-test-data.json");
        for suite in get_test_data(content) {
            suite
                .tests
                .into_iter()
                .for_each(|t| match serde_json::from_value::<Event>(t.input) {
                    Ok(e) => assert!(e == e),
                    Err(err) => panic!(
                        "Failed serialization: {:?} {} {}",
                        err, suite.suite_name, t.name
                    ),
                });
        }
    }

    #[test]
    fn html_test() {
        let content = include_str!("../../../test-libs/test-data/html-test-data.json");
        for suite in get_test_data(content) {
            suite
                .tests
                .into_iter()
                .for_each(|t| match serde_json::from_value::<Html>(t.input) {
                    Ok(e) => assert!(e == e),
                    Err(err) => panic!(
                        "Failed serialization: {:?} {} {}",
                        err, suite.suite_name, t.name
                    ),
                });
        }
    }

    #[test]
    fn value_test() {
        let content = include_str!("../../../test-libs/test-data/value-test-data.json");
        for suite in get_test_data(content) {
            suite
                .tests
                .into_iter()
                .for_each(|t| match serde_json::from_value::<Value>(t.input) {
                    Ok(e) => assert!(e == e),
                    Err(err) => panic!(
                        "Failed serialization: {:?} {} {}",
                        err, suite.suite_name, t.name
                    ),
                });
        }
    }

    #[test]
    fn instr_test() {
        for suite in get_instr_test_data() {
            suite.tests.into_iter().for_each(|t| {
                match t.kind.clone().unwrap_or_default().to_lowercase().as_str() {
                    "i32" => match serde_json::from_value::<Instruction<i32>>(t.input) {
                        Ok(e) => assert!(e == e),
                        Err(err) => panic!(
                            "Failed serialization: {:?} {} {}",
                            err, suite.suite_name, t.name
                        ),
                    },
                    "string" => match serde_json::from_value::<Instruction<String>>(t.input) {
                        Ok(e) => assert!(e == e),
                        Err(err) => panic!(
                            "Failed serialization: {:?} {} {}",
                            err, suite.suite_name, t.name
                        ),
                    },
                    "attr" => match serde_json::from_value::<Instruction<Attr>>(t.input) {
                        Ok(e) => assert!(e == e),
                        Err(err) => panic!(
                            "Failed serialization: {:?} {} {}",
                            err, suite.suite_name, t.name
                        ),
                    },
                    "html" => match serde_json::from_value::<Instruction<Html>>(t.input) {
                        Ok(e) => assert!(e == e),
                        Err(err) => panic!(
                            "Failed serialization: {:?} {} {}",
                            err, suite.suite_name, t.name
                        ),
                    },
                    "value" => match serde_json::from_value::<Instruction<Value>>(t.input) {
                        Ok(e) => assert!(e == e),
                        Err(err) => panic!(
                            "Failed serialization: {:?} {} {}",
                            err, suite.suite_name, t.name
                        ),
                    },
                    _ => unreachable!(
                        "Invalid test data: {:?} {:?} {} {}",
                        t.input, t.kind, suite.suite_name, t.name
                    ),
                }
            });
        }
    }
}
