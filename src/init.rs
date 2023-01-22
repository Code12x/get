use std::io::Error;

use crate::config;

pub fn init() -> Result<Option<String>, Error> {
    match config::try_install() {
        Ok(option) => Ok(option),
        Err(e) => panic!("Cannot install get: {}", e),
    }
}

