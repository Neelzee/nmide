#[cfg(test)]
mod state_modification_test {
    use core_std_lib::state::{State, Value};
    use rstest::rstest;

    #[rstest]
    #[case("field", Value::Null)]
    #[case("field.field", Value::Int(2))]
    #[case("field.field.field", Value::List(Vec::new()))]
    fn adding_adds(#[case] field: &str, #[case] value: Value) {
        assert_eq!(
            State::default()
                .add(field, value.clone())
                .get(field)
                .unwrap(),
            value
        );
    }

    #[test]
    fn removing_from_obj_does_not_remove_entire_obj() {
        let state = State::default().add(
            "nested.obj",
            Value::new_obj()
                .obj_add("field", Value::Null)
                .obj_add("field2", Value::Null),
        );
        let state = state.rem("nested.obj.field2");
        assert!(state.get("nested.obj.field").is_some());
        assert!(state.get("nested.obj.field2").is_none());
    }

    #[rstest]
    #[case(Value::Int(2))]
    #[case(Value::new_float(2f32))]
    #[case(Value::new_str("foobar"))]
    #[case(Value::List(Vec::new()))]
    #[case(Value::List(vec![Value::Null, Value::Int(0)]))]
    #[case(Value::new_obj().obj_add("field", Value::Null).obj_add("Foo", Value::List(Vec::new())))]
    fn adding_to_existing_values_replaces(#[case] value: Value) {
        let field = "field";
        let state = State::default().add(field, Value::Null);
        assert_ne!(state.get(field), state.add(field, value).get(field))
    }

    #[rstest]
    #[case(Value::Null)]
    #[case(Value::Int(2))]
    #[case(Value::new_float(2f32))]
    #[case(Value::new_str("foobar"))]
    #[case(Value::List(Vec::new()))]
    #[case(Value::List(vec![Value::Null, Value::Int(0)]))]
    #[case(Value::new_obj().obj_add("field", Value::Null).obj_add("Foo", Value::List(Vec::new())))]
    fn adding_to_list_modifies_list(#[case] value: Value) {
        assert_eq!(
            State::default()
                .add("list", Value::List(Vec::new()))
                .add("list", value.clone())
                .get("list")
                .unwrap()
                .list()
                .unwrap()
                .pop()
                .unwrap(),
            value,
        );
    }
}
