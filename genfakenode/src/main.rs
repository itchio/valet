use ra_syntax::ast::{ExternItem, ModuleItem, ModuleItemOwner, NameOwner, SourceFile};
use std::{
    error::Error,
    path::{Path, PathBuf},
};

fn main() -> Result<(), Box<dyn Error>> {
    let code_path = get_code_path()?;
    println!("Parsing {:?}", code_path);

    let endpoints = parse_code(&code_path);
    println!("endpoints = {:#?}", endpoints);

    Ok(())
}

#[derive(Debug)]
struct Endpoint {
    name: String,
    num_params: usize,
}

fn get_code_path() -> Result<PathBuf, Box<dyn Error>> {
    let mut cmd = cargo_metadata::MetadataCommand::new();
    cmd.manifest_path("..");
    let metadata = cmd.exec().unwrap();
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
        match item {
            ModuleItem::ExternBlock(block) => {
                if let Some(list) = block.extern_item_list() {
                    for item in list.extern_items() {
                        if let ExternItem::FnDef(fd) = item {
                            if let Some(name) = fd.name() {
                                endpoints.push(Endpoint {
                                    name: name.text().to_string(),
                                    num_params: fd
                                        .param_list()
                                        .map(|x| x.params().count())
                                        .unwrap_or_default(),
                                });
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }

    Ok(endpoints)
}
