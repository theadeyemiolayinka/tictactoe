use serde::{Deserialize, Serialize};
use serde_json::Value;
use strum::IntoEnumIterator;
use strum_macros::{AsRefStr, EnumIter, EnumString};

use crate::{services::config::codes::ResultCode, Failure};

#[derive(Serialize, Deserialize, Debug)]
pub struct Record {
    pub key: RecordKey,
    pub tags: Vec<String>,
    pub value: Value,
}

impl Record {
    pub fn new(key: RecordKey, value: Value, tags: Vec<String>) -> Record {
        Record {
            key: key,
            tags,
            value,
        }
    }

    pub fn value_as<T: for<'de> Deserialize<'de>>(&self) -> Result<T, Failure> {
        serde_json::from_value(self.value.clone()).map_err(|e| Failure {
            message: "Error occured parsing values from database".to_string(),
            trace: format!("Reason: Type {:?}: {}", e.classify(), e.to_string()),
            code: ResultCode::SerializationError,
        })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, EnumString, EnumIter, AsRefStr)]
pub enum RecordKey {
    CommandUsage,
    Custom(String),
}

impl RecordKey {
    pub fn as_string(&self) -> String {
            match self {
                RecordKey::Custom(k) => k.clone(),
                _ => self.as_ref().to_owned().to_string().to_lowercase(),
            }
        }
}

// ANALYTICS
#[derive(EnumString, EnumIter, AsRefStr)]
pub enum ToolsAnalytics {
    INIT,
    START
}

impl ToolsAnalytics {
    pub fn value(&self) -> String {
        match self {
            _ => self.as_ref().to_string().to_lowercase()
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CommandUsageRecord {
    pub values: Vec<(String, i32)>,
}

impl Default for CommandUsageRecord {
    fn default() -> Self {
        Self {
            values: ToolsAnalytics::iter()
                .map(|tool| (tool.value(), 0))
                .collect(),
        }
    }
}
