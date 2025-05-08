#[cfg(test)]
mod value_tests {
    use core_std_lib::state::Value;
    use ordered_float::NotNan;

    #[test]
    fn can_map() {
        let val = Value::Int(1);
        let new_val = val.map(|i: i32| i + 1);
        assert!(new_val.is_ok());
        assert_eq!(new_val.unwrap(), Value::Int(1 + 1));

        let val = Value::Float(NotNan::new(1f32).unwrap());
        let new_val = val.map(|i: i32| i + 1);
        assert!(new_val.is_ok());
        assert_eq!(new_val.unwrap(), Value::Int(1 + 1));

        let val = Value::Float(NotNan::new(1f32).unwrap());
        let new_val = val.map(|i: f32| i + 1f32);
        assert!(new_val.is_ok());
        assert_eq!(
            new_val.unwrap(),
            Value::Float(NotNan::new(1f32 + 1f32).unwrap())
        );
    }

    #[test]
    fn invalid_map_is_err() {
        let val = Value::new_obj();
        let new_val = val.map(|i: i32| i + 1);
        assert!(new_val.is_err());
    }
}
