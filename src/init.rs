use std::fs;
use std::io::Error;

use crate::config;

pub fn init() -> Result<(), Error> {
    match config::try_global_install() {
        Ok(()) => (),
        Err(e) => panic!("Cannot install get: {}", e),
    };

    fs::create_dir("./.get")?;
    Ok(())
}

