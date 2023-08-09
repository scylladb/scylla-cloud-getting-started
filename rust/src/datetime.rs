use std::error::Error;
use std::fmt::Formatter;

use chrono::{NaiveDateTime, Utc};
use scylla::frame::response::cql_to_rust::{FromCqlVal, FromCqlValError};
use scylla::frame::response::result::CqlValue;
use scylla::frame::value::{Value, ValueTooBig};
use serde::Serialize;

#[derive(Debug, Clone, Default)]
pub struct DateTime(chrono::DateTime<Utc>);

impl FromCqlVal<CqlValue> for DateTime {
    fn from_cql(cql_val: CqlValue) -> Result<Self, FromCqlValError> {
        Ok(Self(chrono::DateTime::<Utc>::from_cql(cql_val)?))
    }
}

impl DateTime {
    pub fn now() -> Self {
        Self(Utc::now())
    }
}

impl AsRef<chrono::DateTime<Utc>> for DateTime {
    fn as_ref(&self) -> &chrono::DateTime<Utc> {
        &self.0
    }
}

impl Serialize for DateTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_i64(self.0.timestamp_millis())
    }
}

impl<'de> serde::de::Deserialize<'de> for DateTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_i64(I64Visitor).map(|v| {
            Self(chrono::DateTime::<Utc>::from_utc(
                NaiveDateTime::from_timestamp_opt(v, 0).unwrap(),
                Utc,
            ))
        })
    }
}

impl Value for DateTime {
    fn serialize(&self, buf: &mut Vec<u8>) -> Result<(), ValueTooBig> {
        self.0.serialize(buf)
    }
}

struct I64Visitor;

impl<'de> serde::de::Visitor<'de> for I64Visitor {
    type Value = i64;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("an i64 value")
    }

    fn visit_i32<E>(self, v: i32) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(v as i64)
    }

    fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(v)
    }

    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(v as i64)
    }

    fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E>
    where
        E: Error,
    {
        Ok(v as i64)
    }
}
