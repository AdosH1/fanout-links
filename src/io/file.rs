use core::result::Result;
use std::fs::File;
use std::io::{Error, ErrorKind};
use std::io::{Read, Write};
use tempfile::{tempdir, TempDir};

const CFG_NAME: &str = "fl.cfg";

fn get_file_data(d: TempDir) -> Result<String, Error> {
    let file_path = d.path().join(CFG_NAME);
    println!("{}", &file_path.to_string_lossy());
    let file_res = File::open(file_path);
    match file_res {
        Ok(mut f) => {
            let mut buf = String::new();
            match f.read_to_string(&mut buf) {
                Ok(_) => Ok(buf),
                Err(e) => Err(Error::new(ErrorKind::Other, e)),
            }
        }
        Err(e) => match e.kind() {
            ErrorKind::NotFound => Err(Error::new(ErrorKind::NotFound, e)),
            ErrorKind::PermissionDenied => Err(Error::new(ErrorKind::PermissionDenied, e)),
            _default => Err(Error::new(ErrorKind::Other, e)),
        },
    }
}

pub fn set_config(url: String) -> Result<(), Error> {
    let dir_result = tempdir();
    let dir = match dir_result {
        Ok(d) => d,
        Err(e) => {
            return match e.kind() {
                ErrorKind::NotFound => Err(Error::new(ErrorKind::NotFound, e)),
                ErrorKind::PermissionDenied => Err(Error::new(ErrorKind::PermissionDenied, e)),
                _default => Err(Error::new(ErrorKind::Other, e)),
            }
        }
    };

    let file_path = dir.path().join(CFG_NAME);
    println!("{}", &file_path.to_string_lossy());
    let file_res = File::create(file_path);
    let mut file = match file_res {
        Ok(f) => f,
        Err(e) => {
            return match e.kind() {
                ErrorKind::NotFound => Err(Error::new(ErrorKind::NotFound, e)),
                ErrorKind::PermissionDenied => Err(Error::new(ErrorKind::PermissionDenied, e)),
                _default => Err(Error::new(ErrorKind::Other, e)),
            }
        }
    };

    writeln!(file, "{url}")
}

pub fn get_config() -> Result<String, Error> {
    let dir_result = tempdir();
    match dir_result {
        Ok(d) => get_file_data(d),
        Err(e) => match e.kind() {
            ErrorKind::NotFound => Err(Error::new(ErrorKind::NotFound, e)),
            ErrorKind::PermissionDenied => Err(Error::new(ErrorKind::PermissionDenied, e)),
            _default => Err(Error::new(ErrorKind::Other, e)),
        },
    }
}
