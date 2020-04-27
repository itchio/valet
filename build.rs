fn main() {
    #[cfg(windows)]
    {
        cc::Build::new()
            .file("workaround/workaround.c")
            .compile("workaround");

        cc::Build::new()
            .file("fakenode/fakenode.c")
            .compile("fakenode");
    }

    #[cfg(not(windows))]
    {
        println!("cargo:rustc-cdylib-link-arg=-undefined");
        if cfg!(target_os = "macos") {
            println!("cargo:rustc-cdylib-link-arg=dynamic_lookup");
        }
    }
}
