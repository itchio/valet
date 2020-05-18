fn main() {
    cc::Build::new()
        .file("generated/napi_stub.c")
        .compile("napi_stub");
}
