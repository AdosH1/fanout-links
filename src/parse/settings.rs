use crate::data::settings::Settings;
use anyhow::Result;
use std::collections::HashMap;

pub fn from_config(s: String) -> Result<Settings> {
    let deserialized = serde_json::from_str(&s)?;
    Ok(deserialized)
}

pub fn to_config(s: &Settings) -> Result<String> {
    let serialized = serde_json::to_string(&s)?;
    Ok(serialized)
}

pub fn add_custom_link(
    l: Option<HashMap<String, String>>,
    kv: (String, String),
) -> Option<HashMap<String, String>> {
    match l {
        Some(mut hm) => {
            hm.insert(kv.0, kv.1);
            Some(hm)
        }
        None => {
            let mut hm = HashMap::new();
            hm.insert(kv.0, kv.1);
            Some(hm)
        }
    }
}
