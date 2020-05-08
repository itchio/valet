use std::{
    env,
    ffi::OsStr,
    path::{Path, PathBuf},
    process::Command,
};

/// For use in Cargo build scripts: sets the proper environment variables
/// to use `i686-w64-mingw32-XXX` or `x86_64-w64-mingw32-XXX` toolchains.
pub fn install<F>(mut env_set: F)
where
    F: FnMut(&OsStr, &OsStr),
{
    macro_rules! env {
        ($k: expr, $v: expr) => {
            println!("[mingw_setup] env {:?} = {:?}", $k, $v);
            env_set($k.as_ref(), $v.as_ref())
        };
    }

    let env_target = env::var("TARGET").unwrap();
    let env_path = env::var("PATH").expect("$PATH environment variable must be present");

    match env_target.as_ref() {
        "i686-pc-windows-gnu" => {
            env!("CGO_ENABLED", "1");
            env!("GOOS", "windows");
            env!("GOARCH", "386");
            env!("PATH", env_path.prepend_msys_path("/mingw32/bin"));
        }
        "x86_64-pc-windows-gnu" => {
            env!("CGO_ENABLED", "1");
            env!("GOOS", "windows");
            env!("GOARCH", "amd64");
            env!("PATH", env_path.prepend_msys_path("/mingw64/bin"));
        }
        _ => {}
    }
}

trait PrependMsysPath {
    fn prepend_msys_path<P: AsRef<Path>>(&self, msys_path: P) -> Self;
}

impl PrependMsysPath for String {
    fn prepend_msys_path<P: AsRef<Path>>(&self, msys_path: P) -> Self {
        format!(
            "{addition};{original}",
            addition = msys_path.to_windows_path().display(),
            original = self
        )
    }
}

trait ToWindowsPath {
    fn to_windows_path(&self) -> PathBuf;
}

impl<T> ToWindowsPath for T
where
    T: AsRef<Path>,
{
    fn to_windows_path(&self) -> PathBuf {
        let output = Command::new("cygpath")
            .arg("-w")
            .arg(self.as_ref())
            .output()
            .expect("should be able to convert to windows path with cygpath");
        String::from_utf8_lossy(&output.stdout).trim().into()
    }
}
