use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserializer, Serializer};

#[derive(Debug)]
/// This timestamp is embedded in the built HTML and is used by the hot reload
/// implementation to decide if the page needs to be rebuilt.
pub struct EmbeddedBuildTimestamp(pub SystemTime);

impl std::fmt::Display for EmbeddedBuildTimestamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .duration_since(UNIX_EPOCH)
                .expect("SystemTime instances should always > UNIX_EPOCH")
                .as_nanos()
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
        let formatted: String = serde::Deserialize::deserialize(deserializer)?;
        Ok(EmbeddedBuildTimestamp(
            UNIX_EPOCH
                + std::time::Duration::from_nanos(
                    u64::from_str(&formatted).map_err(serde::de::Error::custom)?,
                ),
        ))
    }
}
