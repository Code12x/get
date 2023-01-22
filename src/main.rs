use structopt::StructOpt;

mod init;
mod config;

#[derive(Debug, StructOpt)]
enum ConfigOptions {
    /// The name of the user
    Name(Name),

    /// The email of the user
    Email(Email),
}

#[derive(StructOpt, Debug)]
struct Config {
    /// Set one of these values to set the respective scope of the config
    #[structopt(subcommand)]
    config_opt: ConfigOptions,
}

#[derive(StructOpt, Debug)]
struct Name {
    #[structopt(name="name")]
    name: String,
}

#[derive(StructOpt, Debug)]
struct Email {
    #[structopt(name="email")]
    email: String,
}

#[derive(StructOpt, Debug)]
#[structopt(name = "get")]
enum Opt {

    /// Initialize a get repository
    Init,

    /// Configurations
    Config(Config),

    /// Add a file to the staging area
    Add,

    /// Commit the staged files
    Commit,

    /// Merge a branch into another one
    Merge,

    /// Fetch the latest commits on a remote repository
    Fetch,

    /// Push a snapshot to a remote repository
    Push,

    /// Pull the latest snapshot from a remote repository
    Pull,
}

fn main() {
    let opt = Opt::from_args();

    match opt {
        Opt::Init => match init::init() {
            Ok(option) => {
                match option {
                    Some(msg) => println!("{}", msg),
                    None => println!("Get repository initialized!"),
                }
            }
            Err(e) => println!("Error initializing the repository: {e}"),
        },
        Opt::Config(config) => {
            match config.config_opt {
                ConfigOptions::Name(name) => {
                    match config::set_config_setting(config::ConfigSetting::Name, &name.name) {
                        Ok(opt) => match opt {
                            Some(msg) => println!("{}", msg),
                            None => println!("Name updated to: {}", name.name),
                        },
                        Err(e) => println!("Error updating name: {}", e),
                    }
                },
                ConfigOptions::Email(email) => {
                    match config::set_config_setting(config::ConfigSetting::Email, &email.email) {
                        Ok(opt) => match opt {
                            Some(msg) => println!("{}", msg),
                            None => println!("Email updated to: {}", email.email),
                        },
                        Err(e) => println!("Error updating email: {}", e),
                    }
                },
            }
        },
        _ => (),
    }
}
