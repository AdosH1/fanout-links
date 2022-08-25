use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    pub api: Option<String>,
    pub custom_links: Option<HashMap<String, String>>,
}
