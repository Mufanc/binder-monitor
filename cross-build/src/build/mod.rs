mod ebpf_prog;
mod userspace_prog;

use clap::Parser;

#[derive(Debug, Parser)]
pub struct Args {
    #[clap(default_value = "aarch64-linux-android", long)]
    pub target: String,

    #[clap(long)]
    pub release: bool,
}

pub fn build_project(target: &str, release: bool) {
    ebpf_prog::build(release);
    userspace_prog::build(target, release);
}
