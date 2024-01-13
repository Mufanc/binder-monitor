#![feature(try_blocks)]
#![feature(exit_status_error)]

use clap::Parser;

mod utils;
mod run;
mod build;

#[derive(Debug, Parser)]
struct Args {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Parser)]
enum Command {
    Build(build::Args),
    Run(run::Args),
}


fn main() {
    let args = Args::parse();

    match args.command {
        Command::Build(args) => build::build_project(&args.target, args.release),
        Command::Run(args) => {
            build::build_project(&args.target, args.release);
            run::run_monitor(&args);
        }
    }
}
