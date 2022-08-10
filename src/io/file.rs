use core::result::Result;
use std::env;
use std::fs::File;
use std::io::Error;
use std::io::{Read, Write};
use std::path::PathBuf;

const CFG_NAME: &str = "fl.cfg";

fn get_cfg_data(d: PathBuf) -> Result<String, Error> {
    let file_path = d.join(CFG_NAME);
    let mut file = File::open(file_path)?;
    let mut buf = String::new();
    match file.read_to_string(&mut buf) {
        Ok(_) => Ok(buf),
        Err(e) => Err(e),
    }
}

pub fn set_config(url: String) -> Result<(), Error> {
    let dir = env::temp_dir();
    let file_path = dir.join(CFG_NAME);
    let file_res = File::create(file_path);

    writeln!(file_res?, "{url}")
}

pub fn get_config() -> Result<String, Error> {
    let dir = env::temp_dir();
    get_cfg_data(dir)
}
