use std::time::{SystemTime, UNIX_EPOCH};

use chrono::{DateTime, Local, TimeZone};
use serde::{Deserializer, Serializer};

#[derive(Debug)]
/// This timestamp is embedded in the built HTML and is used by the hot reload
/// implementation to decide if the page needs to be rebuilt.
pub struct EmbeddedBuildTimestamp(pub SystemTime);

impl std::fmt::Display for EmbeddedBuildTimestamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let duration = self
            .0
            .duration_since(UNIX_EPOCH)
            .expect("SystemTime instances should always > UNIX_EPOCH");
        let secs = duration.as_secs() as i64;
        let nanos = duration.subsec_nanos();
        let datetime = Local
            .timestamp_opt(secs, nanos)
            .single()
            .expect("valid timestamp");
        write!(
            f,
            "{}",
            datetime.to_rfc3339_opts(chrono::SecondsFormat::Nanos, true)
        )
    }
}

impl serde::Serialize for EmbeddedBuildTimestamp {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> serde::Deserialize<'de> for EmbeddedBuildTimestamp {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let datetime_str: String = serde::Deserialize::deserialize(deserializer)?;
        let datetime =
            DateTime::parse_from_rfc3339(&datetime_str).map_err(serde::de::Error::custom)?;
        let timestamp = datetime.timestamp();
        let nanos = datetime.timestamp_subsec_nanos();
        Ok(EmbeddedBuildTimestamp(
            UNIX_EPOCH + std::time::Duration::new(timestamp as u64, nanos),
        ))
    }
}
