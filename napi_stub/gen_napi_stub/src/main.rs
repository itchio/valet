use fs_err::File;
use ra_syntax::ast::{ExternItem, ModuleItem, ModuleItemOwner, NameOwner, SourceFile};
use std::{
    error::Error,
    io::Write,
    path::{Path, PathBuf},
};

fn main() {
    let out_path = PathBuf::from("../generated");
    gen_napi_stub(&out_path).unwrap();
}

// Generates stubs for N-API
fn gen_napi_stub(out_path: &Path) -> Result<(), Box<dyn Error>> {
    let code_path = get_code_path()?;
    println!("Parsing {:?}", code_path);

    let endpoints = parse_code(&code_path)?;

    // Rust side (for DELAYLOAD)
    {
        let out_path = out_path.join("setup.rs");
        let mut f = File::create(&out_path)?;
        writeln!(
            f,
            "/// Patches N-API functions, replacing the stubs with jumps"
        )?;
        writeln!(f, "/// to the actual implementations provided by node.exe")?;
        writeln!(f, "pub unsafe fn setup() {{")?;
        writeln!(f, "   type F = *const ::std::ffi::c_void;")?;
        for e in &endpoints {
            writeln!(
                f,
                "   winhook::hook({:?}, ::nj_sys::{} as F);",
                e.name, e.name
            )?;
        }
        writeln!(f, "}}")?;
        println!("Wrote {:?}", out_path);
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
        println!("Wrote {:?}", out_path);
    }

    Ok(())
}

#[derive(Debug)]
struct Endpoint {
    name: String,
}

fn get_code_path() -> Result<PathBuf, Box<dyn Error>> {
    let mut cmd = cargo_metadata::MetadataCommand::new();
    cmd.manifest_path("../Cargo.toml");
    let metadata = cmd.exec().unwrap();
    assert_eq!(metadata.workspace_root.file_name().unwrap(), "napi_stub");

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
