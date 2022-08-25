use crate::data::settings::Settings;
use anyhow::Result;

pub fn from_config(s: String) -> Result<Settings> {
    let deserialized = serde_json::from_str(&s)?;
    Ok(deserialized)
}

pub fn to_config(s: &Settings) -> Result<String> {
    let serialized = serde_json::to_string(&s)?;
    Ok(serialized)
}
