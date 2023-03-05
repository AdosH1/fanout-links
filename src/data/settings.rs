use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    pub api: Option<String>,
    pub custom_links: Option<HashMap<String, String>>,
}

impl ::std::default::Default for Settings {
    fn default() -> Self { Self { api: None, custom_links: None } }
}