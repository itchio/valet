use std::{env, error::Error, path::PathBuf, process::Command, process::Stdio};

fn main() {
    golang().unwrap();
}

fn golang() -> Result<(), Box<dyn Error>> {
    let gopkg_path = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap());
    let lib_path = PathBuf::from(env::var_os("OUT_DIR").unwrap()).join("libbutler.a");

    let mut cmd = Command::new("go");
    cmd.current_dir(gopkg_path);
    mingw_setup::install(|k, v| {
        cmd.env(k, v);
    });
    cmd.arg("build");
    cmd.arg("-v");
    cmd.arg("-ldflags=-s -w");
    cmd.arg("-buildmode=c-archive");
    cmd.arg("-o");
    cmd.arg(&lib_path);
    cmd.stdout(Stdio::inherit());
    cmd.stderr(Stdio::inherit());

    let status = cmd.spawn()?.wait()?;
    if !status.success() {
        return Err("go compilation failed".into());
    }

    println!(
        "cargo:rustc-link-search={}",
        lib_path.parent().unwrap().display()
    );

    Ok(())
}
