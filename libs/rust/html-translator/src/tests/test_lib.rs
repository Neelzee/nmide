#[cfg(test)]
mod serialization_tests {
    use crate::{translate, translator::from_html::parse_html};
    use pretty_assertions::assert_eq;
    use core_std_lib::html::Html;
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

    #[test]
    fn html_test() {
        let content = include_str!("../../../../test-libs/test-data/html-test-data.json");
        for suite in get_test_data(content) {
            suite
                .tests
                .into_iter()
                .for_each(|t| {
                   let ui = serde_json::from_value::<Html>(t.input).unwrap().remove_events().sort_attrs(); 
                   let html_str = translate(ui.clone());
                    match parse_html(&html_str) {
                    Ok(e) => {
                        assert_eq!(ui, e.sort_attrs(), "Failed assertion: {}-{}", suite.suite_name, t.name);
                    },
                    Err(err) => panic!(
                        "Failed serialization: {:?}, {}, {}",
                        err, suite.suite_name, t.name
                    ),
                }
            });
        }
    }
}
