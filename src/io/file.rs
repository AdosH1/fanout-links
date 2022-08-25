use crate::data::settings::Settings;
use crate::parse::settings::{from_config, to_config};
use anyhow::Result;
use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::path::PathBuf;

const CFG_NAME: &str = "fl.cfg";

fn get_cfg_data(d: PathBuf) -> Result<String> {
    let file_path = d.join(CFG_NAME);
    let mut file = File::open(file_path)?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;
    Ok(buf)
}

pub fn set_config(s: &Settings) -> Result<()> {
    let dir = env::temp_dir();
    let file_path = dir.join(CFG_NAME);
    let file_res = File::create(file_path);

    let serialized = to_config(s)?;

    Ok(writeln!(file_res?, "{serialized}")?)
}

pub fn get_config() -> Result<Settings> {
    let dir = env::temp_dir();
    let s = get_cfg_data(dir)?;
    from_config(s)
}
