use std::fs;

fn main() {
    #[cfg(target_os = "macos")]
    {
        println!("cargo:rustc-cdylib-link-arg=-undefined");
        println!("cargo:rustc-cdylib-link-arg=dynamic_lookup");
    }

    gen_version();
}

fn gen_version() {
    use tinyjson::JsonValue;
    let payload = fs::read_to_string("package.json").unwrap();

    let payload: JsonValue = payload.parse().unwrap();
    let version_string: &String = payload["version"].get().unwrap();
    let tokens: Vec<&str> = version_string.split(".").collect();
    assert_eq!(tokens.len(), 3);
    let (major, minor, patch) = (tokens[0], tokens[1], tokens[2]);

    let source = format!(
        r#"
pub struct Version {{
    major: usize,
    minor: usize,
    patch: usize,
}}

pub const VERSION: Version = Version {{
    major: {major},
    minor: {minor},
    patch: {patch},
}};
    "#,
        major = major,
        minor = minor,
        patch = patch
    );
    fs::create_dir_all("generated").unwrap();
    fs::write("generated/version.rs", source).unwrap();
}
