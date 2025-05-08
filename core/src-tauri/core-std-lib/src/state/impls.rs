use std::cmp::Ordering;

use ordered_float::NotNan;

use crate::state::Value;

impl Value {
    pub fn map<F, T, R, G>(self, f: F) -> Result<Self, ValueMapError>
    where
        G: Fn(T) -> R,
        F: ValueMapper<G, T, R>,
    {
        f.map_value(self)
    }
}

#[derive(Debug, Clone)]
pub enum ValueMapError {
    CannotCast(Value, Value),
    FloatIsNan(f32),
    UnimplementedMapping(Value),
}

pub trait ValueMapper<F, T, R>
where
    F: Fn(T) -> R,
{
    fn map_value(&self, value: Value) -> Result<Value, ValueMapError>;
}

impl<F> ValueMapper<F, i32, i32> for F
where
    F: Fn(i32) -> i32,
{
    fn map_value(&self, value: Value) -> Result<Value, ValueMapError> {
        match value {
            Value::Int(i) => Ok(Value::Int(self(i))),
            Value::Float(not_nan) => {
                let cmp = not_nan.total_cmp(&(i32::MAX as f32));
                if not_nan.is_infinite() || cmp == Ordering::Greater || cmp == Ordering::Equal {
                    return Err(ValueMapError::CannotCast(value, Value::Int(0)));
                }
                let new_value: i32 = unsafe { not_nan.to_int_unchecked() };
                Ok(Value::Int(self(new_value)))
            }
            Value::List(xs) => xs
                .into_iter()
                .map(|v| self.map_value(v))
                .collect::<Result<Vec<Value>, _>>()
                .map(|zs| Value::List(zs)),
            _ => Err(ValueMapError::CannotCast(value, Value::Int(0))),
        }
    }
}

impl<F> ValueMapper<F, f32, i32> for F
where
    F: Fn(f32) -> i32,
{
    fn map_value(&self, value: Value) -> Result<Value, ValueMapError> {
        match value {
            Value::Int(i) => Ok(Value::Int(self(i as f32))),
            Value::Float(not_nan) => Ok(Value::Int(self(*not_nan))),
            Value::List(xs) => xs
                .into_iter()
                .map(|v| self.map_value(v))
                .collect::<Result<Vec<Value>, _>>()
                .map(|zs| Value::List(zs)),
            _ => Err(ValueMapError::CannotCast(value, Value::Int(0))),
        }
    }
}

impl<F> ValueMapper<F, f32, f32> for F
where
    F: Fn(f32) -> f32,
{
    fn map_value(&self, value: Value) -> Result<Value, ValueMapError> {
        match value {
            Value::Int(i) => {
                let new_value = self(i as f32);
                match NotNan::new(new_value) {
                    Ok(f) => Ok(Value::Float(f)),
                    Err(_) => Err(ValueMapError::FloatIsNan(new_value)),
                }
            }
            Value::Float(not_nan) => {
                let new_value = self(*not_nan);
                match NotNan::new(new_value) {
                    Ok(f) => Ok(Value::Float(f)),
                    Err(_) => Err(ValueMapError::FloatIsNan(new_value)),
                }
            }
            Value::List(xs) => xs
                .into_iter()
                .map(|v| self.map_value(v))
                .collect::<Result<Vec<Value>, _>>()
                .map(|zs| Value::List(zs)),
            _ => Err(ValueMapError::CannotCast(value, Value::Int(0))),
        }
    }
}
