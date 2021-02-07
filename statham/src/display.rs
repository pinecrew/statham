use std::fmt;

use crate::error::JsonError;
use crate::json::{Json, Primitive, Value};

impl fmt::Display for Json {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO: rewrite
        write!(f, "{{{}}}", self.iter().map(|kv| format!(r#""{}":{}"#, kv.0, kv.1)).collect::<Vec<String>>().join(","))
    }
}

impl fmt::Display for Primitive {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Primitive::Object(value) => write!(f, "{{{}}}", value),
            Primitive::Array(value) => {
                // TODO: rewrite
                write!(f, "[{}]", value.iter().map(|x| format!("{}", x)).collect::<Vec<String>>().join(","))
            }
            Primitive::Text(value) => write!(f, r#""{}""#, value),
            Primitive::Number(value) => write!(f, "{}", value),
            Primitive::Boolean(value) => write!(f, "{}", value),
            Primitive::None => f.write_str("null"),
            Primitive::Null => f.write_str("null"),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Integer(value) => write!(f, "{}", value),
            Value::Float(value) => write!(f, "{}", value),
        }
    }
}

impl fmt::Display for JsonError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JsonError::Io(ref err) => err.fmt(f),
            JsonError::Line(ref row, ref col) => write!(f, "Error at line {} column {}", row, col),
        }
    }
}
