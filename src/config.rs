use std::fs::{File, self, OpenOptions};
use std::path::{PathBuf, Path};
use std::io::{Error, ErrorKind};

use home;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum ConfigSetting {
    Name,
    Email,
}

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    name: String,
    email: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Remote {
    remote: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct LocalConfig {
    head: String,
    remotes: Vec<Remote>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Branch {
    commits: Vec<String>,
}

fn get_global_path() -> Result<PathBuf, Error> {
    match home::home_dir() {
        Some(path_buff) => {
            return Ok(path_buff.join(PathBuf::from(".config/")));
        }
        None => Err(Error::new(ErrorKind::Unsupported, "Could not find home path. (Are you using windows? Only linux is supported)")),
    }
}

fn get_local_path() -> Result<PathBuf, Error> {
    let path = Path::new("./.get/");
    match Path::try_exists(path) {
        Ok(is_existent) => {
            if is_existent {
                Ok(PathBuf::from("./.get/"))
            } else {
                Err(Error::new(ErrorKind::NotFound, "Counld not find local repository path"))
            } 
        },
        Err(e) => Err(e),
    }
}

fn get_configuration (path: &PathBuf) -> Result<Config, Error> {
    let file = match OpenOptions::new()
        .read(true)
        .open(path) {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

    let config: Config = match serde_yaml::from_reader(file) {
        Ok(config) => config,
        Err(_) => return Err(Error::new(ErrorKind::Other, "Error Deserializing the contents of the file")),
    };
    Ok(config)
}

pub fn try_install() -> Result<Option<String>, Error> {
    let path = match get_global_path() {
        Ok(path) => path,
        Err(e) => return Err(e),
    };

    let config_path = path.join(PathBuf::from("get.yaml"));

    if config_path.is_file() {
        ();
    } else {
        let config_file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(config_path);

        match config_file {
            Ok(_) => {
                let config_file = config_file.unwrap();
                let default_config = Config{ name: String::from("Default Name"), email: String::from("defaultEmail@example.com"), };

                serde_yaml::to_writer(config_file, &default_config).unwrap();
            },
            Err(e) => return Err(e),
        }
    }

    match get_local_path() {
        Ok(_) => return Ok(Some("Get repository already initialized".to_owned())),
        Err(e) => match e.kind() {
            ErrorKind::NotFound => {
                match fs::create_dir("./.get/") {
                    Ok(_) => (),
                    Err(e) => return Err(e),
                }
            },
            _ => return Err(e)
        }
    };

    let local_path = PathBuf::from("./.get/");
    fs::create_dir(local_path.join("stage"))?;
    fs::create_dir(local_path.join("objects"))?;
    let local_config_file = OpenOptions::new().write(true).create(true).open(local_path.join("config.yaml"))?;
    let local_config = LocalConfig{ head: "master".to_owned(), remotes: Vec::new() };
    serde_yaml::to_writer(&local_config_file, &local_config).unwrap();

    match fs::create_dir(local_path.join("branches/")) {
        Ok(_) => (),
        Err(e) => return Err(e),
    };

    let main_file = OpenOptions::new().write(true).create(true).open(local_path.join("branches/main.yaml"))?;
    let main_branch = Branch { commits: Vec::new(), };
    serde_yaml::to_writer(&main_file, &main_branch).unwrap();

    Ok(None)
}


pub fn set_config_setting(config_setting: ConfigSetting, new_value: &str) -> Result<Option<String>, Error> {
    let path: PathBuf = match get_global_path() {
        Ok(path) => path,
        Err(_) => return Err(Error::new(ErrorKind::Other, "Get is not installed, run \"get init\" to install it and initialize a repository")),
    };

    let path = path.join("get.yaml");

    let mut configuration = match get_configuration(&path) {
        Ok(configuration) => configuration,
        Err(e) => return Err(e),
    };

    match config_setting {
        ConfigSetting::Name => configuration.name = new_value.to_owned(),
        ConfigSetting::Email => configuration.email = new_value.to_owned(),
    }

    let config_file = File::options().write(true).truncate(true).open(&path)?;

    match serde_yaml::to_writer(config_file, &configuration) {
        Ok(_) => return Ok(None),
        Err(e) => return Err(Error::new(ErrorKind::Other, format!("Error updating the config file: {e}")))
    };
}
