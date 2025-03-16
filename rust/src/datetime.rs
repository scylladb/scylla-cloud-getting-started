use std::error::Error;
use std::fmt::Formatter;

use chrono::{TimeZone, Utc};
use scylla::deserialize::value::DeserializeValue;
use scylla::serialize::value::SerializeValue;
use serde::Serialize;

#[derive(Debug, Clone, Default)]
pub struct DateTime(chrono::DateTime<Utc>);

// Derive macros in the driver don't support tuple structs for now,
// so `DeserializeValue` trait needs to be implemented manually.
// Fortunately it is easy to do - we just delegate to existing implementation on
// `chrono::DateTime<Utc>`.
impl<'frame, 'metadata> DeserializeValue<'frame, 'metadata> for DateTime {
    fn type_check(
        typ: &scylla::cluster::metadata::ColumnType,
    ) -> Result<(), scylla::errors::TypeCheckError> {
        <chrono::DateTime<Utc> as DeserializeValue>::type_check(typ)
    }

    fn deserialize(
        typ: &'metadata scylla::cluster::metadata::ColumnType<'metadata>,
        v: Option<scylla::deserialize::FrameSlice<'frame>>,
    ) -> Result<Self, scylla::errors::DeserializationError> {
        let inner = <chrono::DateTime<Utc> as DeserializeValue>::deserialize(typ, v)?;
        Ok(Self(inner))
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
        let millis = deserializer.deserialize_i64(I64Visitor)?;
        match chrono::Utc.timestamp_millis_opt(millis) {
            chrono::LocalResult::Single(ts) => Ok(Self(ts)),
            _ => Err(<D::Error as serde::de::Error>::custom(
                "Timestamp out of range",
            )),
        }
    }
}

// Derive macros in the driver don't support tuple structs for now,
// so `SerializeValue` trait needs to be implemented manually.
// Fortunately it is easy to do - we just delegate to existing implementation on
// `chrono::DateTime<Utc>`.
impl SerializeValue for DateTime {
    fn serialize<'b>(
        &self,
        typ: &scylla::cluster::metadata::ColumnType,
        writer: scylla::serialize::writers::CellWriter<'b>,
    ) -> Result<scylla::serialize::writers::WrittenCellProof<'b>, scylla::errors::SerializationError>
    {
        <chrono::DateTime<Utc> as SerializeValue>::serialize(&self.0, typ, writer)
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
