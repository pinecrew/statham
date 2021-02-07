use std::io;

use crate::error::*;
use crate::json::*;

// macros
#[macro_export]
macro_rules! from {
    ($x:expr) => {
        Primitive::from($x)
    };
}

#[macro_export]
macro_rules! array {
    ( $($t:expr), * ) => {
        vec![$( Primitive::from($t), )*]
    }
}

macro_rules! integers {
    ( $($t:tt),* ) => {
        $(
            impl From<$t> for Primitive {
                fn from(value: $t) -> Self {
                    Primitive::Number(Value::Integer(value as i64))
                }
            }

            impl From<Vec<$t>> for Primitive {
                fn from(value: Vec<$t>) -> Self {
                    Primitive::Array(value.iter().map(|&x| Primitive::from(x as i64)).collect())
                }
            }
        )*
    };
}

macro_rules! floats {
    ( $($t:tt),* ) => {
        $(
            impl From<$t> for Primitive {
                fn from(value: $t) -> Self {
                    Primitive::Number(Value::Float(value as f64))
                }
            }

            impl From<Vec<$t>> for Primitive {
                fn from(value: Vec<$t>) -> Self {
                    Primitive::Array(value.iter().map(|&x| Primitive::from(x as f64)).collect())
                }
            }
        )*
    };
}

// casts

integers!(i8, u8, i16, u16, i32, u32, i64, u64);
floats!(f32, f64);

impl From<bool> for Primitive {
    fn from(value: bool) -> Self {
        Primitive::Boolean(value)
    }
}

impl From<&str> for Primitive {
    fn from(value: &str) -> Self {
        Primitive::Text(value.to_string())
    }
}

impl From<Vec<Primitive>> for Primitive {
    fn from(value: Vec<Primitive>) -> Self {
        Primitive::Array(value)
    }
}

impl From<String> for Primitive {
    fn from(value: String) -> Self {
        Primitive::Text(value)
    }
}

impl From<Json> for Primitive {
    fn from(value: Json) -> Self {
        Primitive::Object(value)
    }
}

impl From<io::Error> for JsonError {
    fn from(value: io::Error) -> Self {
        JsonError::Io(value)
    }
}

impl<T: Into<Primitive>> From<Option<T>> for Primitive {
    fn from(value: Option<T>) -> Self {
        match value {
            Some(v) => v.into(),
            None => Primitive::None,
        }
    }
}

#[cfg(feature = "unstable")]
impl<T, const N: usize> From<&[T; N]> for Primitive
where
    T: Into<Primitive> + Copy,
{
    fn from(value: &[T; N]) -> Self {
        Primitive::Array(value.iter().map(|&x| x.into()).collect())
    }
}