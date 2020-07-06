#![allow(dead_code)]

use super::Settings;
use once_cell::sync::Lazy;
use std::process::Command;

trait ToGo {
    fn to_go(&self) -> &str;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OS {
    Windows,
    Linux,
    MacOS,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Arch {
    X86,
    X86_64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Runtime {
    pub os: OS,
    pub arch: Arch,
}

#[derive(thiserror::Error, Debug, Clone)]
pub enum Error {
    #[error("unsupported OS")]
    UnsupportedOS,
    #[error("unsupported arch")]
    UnsupportedArch,
}

pub const fn build_os() -> Result<OS, Error> {
    #[cfg(target_os = "windows")]
    return Ok(OS::Windows);

    #[cfg(target_os = "linux")]
    return Ok(OS::Linux);

    #[cfg(target_os = "macos")]
    return Ok(OS::MacOS);

    #[allow(unreachable_code)]
    Err(Error::UnsupportedOS)
}

pub const fn build_arch() -> Result<Arch, Error> {
    #[cfg(target_arch = "x86")]
    return Ok(Arch::X86);

    #[cfg(target_arch = "x86_64")]
    return Ok(Arch::X86_64);

    #[allow(unreachable_code)]
    Err(Error::UnsupportedArch)
}

fn tool_output(tool: &str, args: &[&str]) -> Option<String> {
    let output = Command::new(tool).args(args).output().ok()?.stdout;
    let output = std::str::from_utf8(&output[..]).ok()?;
    Some(output.trim().into())
}

#[allow(dead_code)]
static RUNTIME_ARCH: Lazy<Result<Arch, Error>> = Lazy::new(|| {
    match build_os()? {
        OS::MacOS => {
            // we don't build for 32-bit macOS
            Ok(Arch::X86_64)
        }
        OS::Windows => {
            // N.B: the order of these tests matters
            let build_arch = build_arch()?;
            if matches!(build_arch, Arch::X86_64)
                || matches!(
                    std::env::var("PROCESSOR_ARCHITECTURE")
                        .ok()
                        .unwrap_or_default()
                        .as_str(),
                    "AMD64" | "IA64"
                )
                || matches!(
                    std::env::var("PROCESSOR_ARCHITEW6432")
                        .ok()
                        .unwrap_or_default()
                        .as_str(),
                    "AMD64" | "IA64"
                )
            {
                Ok(Arch::X86_64)
            } else {
                Ok(build_arch)
            }
        }
        OS::Linux => {
            if matches!(tool_output("uname", &["-s"]).as_deref(), Some("x86_64"))
                || matches!(tool_output("arch", &[]).as_deref(), Some("x86_64"))
            {
                Ok(Arch::X86_64)
            } else {
                build_arch()
            }
        }
    }
});

pub fn current_runtime() -> Result<Runtime, Error> {
    let arch = (*RUNTIME_ARCH).clone();
    Ok(Runtime {
        os: build_os()?,
        arch: arch?,
    })
}

pub fn get_channel() -> String {
    todo!()
}
