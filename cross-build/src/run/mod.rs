use std::path::{Path, PathBuf};
use std::process::Command;

use clap::Parser;

const PROJECT_NAME: &str = "binder-monitor";
const ADB_PUSH_PATH: &str = "/data/local/tmp";


#[derive(Debug, Copy, Clone)]
pub enum Platform {
    Phone,
    Avd
}

impl std::str::FromStr for Platform {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "phone" => Platform::Phone,
            "avd" => Platform::Avd,
            _ => return Err("invalid platform".to_owned()),
        })
    }
}


#[derive(Debug, Parser)]
pub struct Args {
    #[clap(default_value = "aarch64-linux-android", long)]
    pub target: String,

    #[clap(long)]
    pub release: bool,

    #[clap(default_value = "phone", short, long)]
    pub platform: Platform
}


fn adb(args: &[&str]) {
    let mut command = Command::new("adb");
    command.args(args)
        .spawn()
        .and_then(|mut proc| proc.wait())
        .unwrap_or_else(|_| panic!("failed to run adb command with args: {:?}", command.get_args().collect::<Vec<_>>()));
}


fn adb_push<P : AsRef<Path>, Q : AsRef<Path>>(src: P, dst: Q) {
    let src: &Path = src.as_ref();
    let dst: &Path = dst.as_ref();

    adb(&["push", src.to_str().unwrap(), dst.to_str().unwrap()]);
}


fn adb_shell(command: &str) {
    adb(&["shell", command]);
}


pub fn run_monitor(args: &Args) {
    let profile = if args.release { "release" } else { "debug" };
    let build_path = PathBuf::from("target").join(&args.target).join(profile).join(PROJECT_NAME);
    let deploy_path = PathBuf::from(ADB_PUSH_PATH).join(PROJECT_NAME);

    adb_push(build_path, ADB_PUSH_PATH);

    let runner = match args.platform {
        Platform::Phone => "su -c",
        Platform::Avd => "su root"
    };
    let command = format!("RUST_LOG=debug {runner} {}", deploy_path.to_str().unwrap());

    println!("running: {command}");

    adb_shell(&command);
}
