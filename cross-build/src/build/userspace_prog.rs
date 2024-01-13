use std::env;
use std::path::PathBuf;
use std::process::Command;

use glob::{glob, Paths};

use crate::utils::Also;

const NDK_ROOT: &str = "ANDROID_NDK_ROOT";


fn last_entry(entries: Paths) -> Option<String> {
    let mut entries: Vec<PathBuf> = entries.map(|it| it.unwrap()).collect();
    entries.sort();
    Some(entries.last()?.to_str()?.to_string())
}

fn find_ar(ndk_root: &str) -> String {
    let pattern = format!("{ndk_root}/toolchains/llvm/prebuilt/*/bin/llvm-ar");

    glob(&pattern).ok()
        .and_then(last_entry)
        .unwrap_or_else(|| panic!("could not find llvm-ar for build"))
}

fn find_linker(ndk_root: &str, target: &str) -> String {
    let pattern = format!("{ndk_root}/toolchains/llvm/prebuilt/*/bin/{target}*-clang");

    glob(&pattern).ok()
        .and_then(last_entry)
        .unwrap_or_else(|| panic!("could not find {target}-clang for build"))
}


pub fn build(target: &str, release: bool) {
    let project_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).parent().unwrap().to_owned();
    let triple_upper = &target.to_uppercase().replace('-', "_");
    let ndk_root = &env::var(NDK_ROOT).expect("Please set ANDROID_NDK_ROOT to specify your Android NDK.");

    let ar = &find_ar(ndk_root);
    let linker = &find_linker(ndk_root, target);

    println!("found ar: {}", ar);
    println!("found linker: {}", linker);

    let code = Command::new(env!("CARGO"))
        .current_dir(project_dir)
        .arg("build")
        .args(["--target", target])
        .also(|cmd| if release { cmd.arg("--release"); })
        .env(format!("CARGO_TARGET_{}_AR", triple_upper), ar)
        .env(format!("CARGO_TARGET_{}_LINKER", triple_upper), linker)
        .status().expect("failed to get exit status")
        .code().expect("failed to get status code");

    if code != 0 {
        panic!("build_userspace: cargo command failed with code {code}");
    }
}
