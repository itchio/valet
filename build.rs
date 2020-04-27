fn main() {
    #[cfg(windows)]
    windows();

    #[cfg(not(windows))]
    {
        println!("cargo:rustc-cdylib-link-arg=-undefined");
        if cfg!(target_os = "macos") {
            println!("cargo:rustc-cdylib-link-arg=dynamic_lookup");
        }
    }
}

fn windows() {
    cc::Build::new()
        .file("mingwcompat/mingwcompat.c")
        .compile("mingwcompat");

    cc::Build::new()
        .file("nodestub/stubs.c")
        .compile("nodestub");
}
