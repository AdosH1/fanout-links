use crate::data::settings::Settings;
use anyhow::Result;

const CFG_NAME: &str = "fl.config";

pub fn set_config(s: &Settings) -> Result<(), anyhow::Error> {
    confy::store(CFG_NAME, &s)?;
    Ok(())
}

pub fn get_config() -> Result<Settings, anyhow::Error> {
    let config : Settings = confy::load(CFG_NAME)?;
    Ok(config)
}