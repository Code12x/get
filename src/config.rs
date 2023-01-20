use std::{fs::OpenOptions, path::PathBuf};
use std::io::{Error, ErrorKind};
use home;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    name: String,
    email: String,
}

pub fn try_global_install() -> std::io::Result<()> {
    let path_home = home::home_dir();
    let path: PathBuf = match path_home {
        Some(path_buff) => {
            let config_path = PathBuf::from(".config/get.yml");
            path_buff.join(config_path)
        }
        None => return Err(Error::new(ErrorKind::Unsupported, "Could not find home path. (Are you using windows? Only linux is supported)")),
    };

    if path.is_file() {
        return Ok(());
    } else {
        let config_file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(path);

        match config_file {
            Ok(_) => {
                let config_file = config_file.unwrap();
                let default_config = Config{ name: String::from("Default Name"), email: String::from("defaultEmail@example.com"), };

                serde_yaml::to_writer(config_file, &default_config).unwrap();
                Ok(())
            },
            Err(e) => Err(e),
        }
    }
}
