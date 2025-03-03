use clap::Parser;
use clean::clean;
mod build;
mod clean;
mod init;
mod macros;

#[derive(Parser)]
#[command(version = "1.0.0")]
/// A tool for creating Noa package lists
enum Cli {
    /// Create a template in the current directory to create the noa package list
    Init,
    /// Create packagelist.tar.xz in the out directory and calculate the hash value.
    Build,
    /// Delete all the templates that were created
    Clean,
}

fn main() {
    let args = Cli::parse();

    match args {
        Cli::Init => {
            init::init().unwrap_or_else(|e| {
                err!("Failed to initialize: {e}");
            });
        }
        Cli::Build => build::build().unwrap_or_else(|e| {
            err!("Failed to build package list: {e}");
        }),
        Cli::Clean => clean(),
    }
}
