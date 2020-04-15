extern crate neon_build;
use std::{env, error::Error, path::PathBuf, process::Command};

fn main() {
    neon_build::setup();
    build_libbutler().unwrap();
}

fn build_libbutler() -> Result<(), Box<dyn Error>> {
    let out_dir = PathBuf::from(env::var("OUT_DIR")?);
    let lib_dir = out_dir.join("libbutler-prefix");
    println!("cargo:rustc-link-lib={}", "butler");
    println!("cargo:rustc-link-search={}", lib_dir.display());

    let lib_path = lib_dir.join("libbutler.a");

    let dasho_arg = format!("-o={}", lib_path.display());
    let mut cmd = Command::new("go");
    cmd.args(&[
        "build",
        "-v",
        "-buildmode=c-archive",
        &dasho_arg,
        "../libbutler",
    ]);
    let output = cmd.output()?;
    if !output.status.success() {
        panic!(
            "Could not build libbutler (status {:?}):\n {}",
            output.status,
            std::str::from_utf8(&output.stderr)?
        );
    }

    let mut cmd = Command::new("ar");
    cmd.args(&["-s", lib_path.to_str().unwrap()]);
    let output = cmd.output()?;
    if !output.status.success() {
        panic!(
            "Could not build libbutler (status {:?}):\n {}",
            output.status,
            std::str::from_utf8(&output.stderr).unwrap()
        );
    }

    Ok(())
}
