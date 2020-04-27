use std::{env, error::Error, path::PathBuf, process::Command, process::Stdio};

fn main() {
    #[cfg(windows)]
    windows::process().unwrap();

    #[cfg(not(windows))]
    {
        println!("cargo:rustc-cdylib-link-arg=-undefined");
        if cfg!(target_os = "macos") {
            println!("cargo:rustc-cdylib-link-arg=dynamic_lookup");
        }
    }

    golang().unwrap();
}

fn golang() -> Result<(), Box<dyn Error>> {
    let gopkg_path = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").unwrap()).join("libbutler");
    let lib_path = PathBuf::from(env::var_os("OUT_DIR").unwrap()).join("libbutler.a");

    let mut cmd = Command::new("go");
    cmd.current_dir(gopkg_path);
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

    println!("cargo:rustc-cdylib-link-arg={}", lib_path.display());

    Ok(())
}

#[cfg(windows)]
mod windows {
    use ra_syntax::ast::{ExternItem, ModuleItem, ModuleItemOwner, NameOwner, SourceFile};
    use std::{
        env,
        error::Error,
        fs::File,
        io::Write,
        path::{Path, PathBuf},
    };

    pub(crate) fn process() -> Result<(), Box<dyn Error>> {
        let out_path = PathBuf::from(env::var_os("OUT_DIR").unwrap());
        gen_mingw_compat(&out_path)?;
        gen_napi_stub(&out_path)?;

        Ok(())
    }

    // Works around rust shipping a different version of mingw
    // than MSYS2.
    fn gen_mingw_compat(out_path: &Path) -> Result<(), Box<dyn Error>> {
        let source = r#"
#define _CRTBLD
#include <stdio.h>

FILE *__cdecl __acrt_iob_func(unsigned index)
{
    return &(__iob_func()[index]);
}

typedef FILE *__cdecl (*_f__acrt_iob_func)(unsigned index);
_f__acrt_iob_func __MINGW_IMP_SYMBOL(__acrt_iob_func) = __acrt_iob_func;
"#;
        let out_path = out_path.join("mingw_compat.c");
        std::fs::write(&out_path, source)?;

        cc::Build::new().file(&out_path).compile("mingw_compat");
        Ok(())
    }

    // Generates stubs for N-API
    fn gen_napi_stub(out_path: &Path) -> Result<(), Box<dyn Error>> {
        let code_path = get_code_path()?;
        println!("Parsing {:?}", code_path);

        let endpoints = parse_code(&code_path)?;

        // Rust side (for DELAYLOAD)
        {
            let out_path = out_path.join("delayload_process.rs");
            let mut f = File::create(&out_path)?;
            writeln!(f, "pub unsafe fn process() {{")?;
            writeln!(f, "type F = *const ::std::ffi::c_void;")?;
            for e in &endpoints {
                writeln!(f, "winhook::hook({:?}, ::nj_sys::{} as F);", e.name, e.name)?;
            }
            writeln!(f, "}}")?;
        }

        // C side (for linker)
        {
            let out_path = out_path.join("napi_stub.c");
            let mut f = File::create(&out_path)?;
            writeln!(
                f,
                "{}",
                r#"
#include <stdlib.h>
#include <stdio.h>

#define stub(name) void name() { \
    asm("ud2"); asm("nop"); asm("nop"); asm("nop"); \
    asm("nop"); asm("nop"); asm("nop"); asm("nop"); \
    asm("nop"); asm("nop"); asm("nop"); asm("nop"); \
}
            "#
            )?;
            for e in &endpoints {
                writeln!(f, "stub({})", e.name)?;
            }
            drop(f);
            cc::Build::new().file(&out_path).compile("napi_stub");
        }

        Ok(())
    }

    #[derive(Debug)]
    struct Endpoint {
        name: String,
    }

    fn get_code_path() -> Result<PathBuf, Box<dyn Error>> {
        let metadata = cargo_metadata::MetadataCommand::new().exec().unwrap();
        assert_eq!(metadata.workspace_root.file_name().unwrap(), "valet");

        let pkg = metadata
            .packages
            .iter()
            .find(|p| p.name == "nj-sys")
            .unwrap();
        let target = &pkg.targets[0];
        let code_path = target.src_path.parent().unwrap().join("binding.rs");
        Ok(code_path)
    }

    fn parse_code(code_path: &Path) -> Result<Vec<Endpoint>, Box<dyn Error>> {
        let code = std::fs::read_to_string(&code_path)?;

        let parse = SourceFile::parse(&code);
        assert!(parse.errors().is_empty());

        let file: SourceFile = parse.tree();

        let mut endpoints = Vec::new();
        for item in file.items() {
            if let ModuleItem::ExternBlock(block) = item {
                for item in block
                    .extern_item_list()
                    .iter()
                    .flat_map(|il| il.extern_items())
                {
                    if let ExternItem::FnDef(fd) = item {
                        for name in fd.name() {
                            endpoints.push(Endpoint {
                                name: name.text().to_string(),
                            });
                        }
                    }
                }
            }
        }

        Ok(endpoints)
    }
}
