use structopt::StructOpt;

mod init;
mod config;

#[derive(StructOpt, Debug)]
#[structopt(name = "get")]
enum Opt {

    /// Initialize a get repository
    Init,

    /// Configurations
    Config,

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
        Init => match init::init() {
            Ok(()) => println!("Get repository initialized!"),
            Err(e) => println!("Error initializing the repository: {e}"),
        },
        _ => (),
    }
}
