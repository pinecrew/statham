#![allow(dead_code, unused_variables)]
use std::collections::hash_map::{Iter, Keys, Values};
use std::collections::HashMap;
use std::io::{self, Read, Write};

use crate::error::JsonError;

pub enum Value {
    Integer(i64),
    Float(f64),
}

pub enum Primitive {
    Object(Json),
    Array(Vec<Primitive>),
    Text(String),
    Number(Value),
    Boolean(bool),
    Null,
}

#[derive(Default)]
pub struct Json(HashMap<String, Primitive>);

// main code
impl Json {
    pub fn new() -> Json {
        Json::default()
    }

    pub fn from_reader<R: Read>(reader: &mut R) -> Result<Json, JsonError> {
        let mut buffer = String::new();
        reader.read_to_string(&mut buffer)?;
        Json::deserialize(&buffer)
    }

    pub fn to_writer<W: Write>(&self, writer: &mut W) -> Result<(), io::Error> {
        writer.write_all(self.serialize().as_bytes())?;
        Ok(())
    }

    pub fn item<K, V>(mut self, key: K, value: V) -> Self
    where
        K: Into<String>,
        V: Into<Primitive>,
    {
        self.0.insert(key.into(), value.into());
        self
    }

    pub fn items<K, V, I>(mut self, items: I) -> Self
    where
        K: Into<String>,
        V: Into<Primitive>,
        I: IntoIterator<Item = (K, V)>,
    {
        for (k, v) in items {
            self = self.item(k, v);
        }
        self
    }

    pub fn serialize(&self) -> String {
        self.to_string()
    }

    pub fn deserialize(data: &str) -> Result<Json, JsonError> {
        todo!()
    }

    pub fn iter(&self) -> Iter<'_, String, Primitive> {
        self.0.iter()
    }

    pub fn keys(&self) -> Keys<'_, String, Primitive> {
        self.0.keys()
    }

    pub fn values(&self) -> Values<'_, String, Primitive> {
        self.0.values()
    }

    pub fn get<S>(&self, k: S) -> Option<&Primitive>
    where
        S: Into<String>,
    {
        self.0.get(&k.into())
    }

    pub fn get_mut<S>(&mut self, k: S) -> Option<&mut Primitive>
    where
        S: Into<String>,
    {
        self.0.get_mut(&k.into())
    }
}
