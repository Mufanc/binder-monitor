use std::path::PathBuf;
use std::process::Command;

use crate::utils::Also;

const BPF_TARGET: &str = "bpfel-unknown-none";

pub fn build(release: bool) {
    let project_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).parent().unwrap().to_owned();
    let project_dir = project_root.join("crates/ebpf");

    let code = Command::new("cargo")
        .current_dir(project_dir)
        .env_remove("RUSTUP_TOOLCHAIN")
        .arg("build")
        .args(["--target", BPF_TARGET])
        .arg("-Z")
        .arg("build-std=core")
        .also(|cmd| if release { cmd.arg("--release"); })
        .status().expect("failed to get exit status")
        .code().expect("failed to get status code");

    if code != 0 {
        panic!("build_ebpf: cargo command failed with code {code}");
    }
}
