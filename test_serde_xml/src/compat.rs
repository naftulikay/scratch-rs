use chrono::NaiveDateTime;
use serde::de::Error as DeserializeError;
use serde::{Deserialize, Deserializer, Serializer};

/// Deserialize a Unix epoch timestamp into a `chrono::NaiveDateTime`.
pub fn deserialize_naive_date_timestamp<'de, D>(d: D) -> Result<NaiveDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    NaiveDateTime::from_timestamp_opt(i64::deserialize(d)?, 0)
        .ok_or_else(|| D::Error::custom("unable to construct NaiveDateTime from i64"))
}

/// Serialize a `chrono::NaiveDateTime` into a string representation of the seconds since the Unix
/// epoch.
pub fn serialize_naive_datetime_timestamp_to_str<S>(
    value: &NaiveDateTime,
    s: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_str(value.timestamp().to_string().as_str())
}
