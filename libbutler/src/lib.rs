#[link(name = "butler", kind = "static")]
extern "C" {
    pub fn PrintCountry();
    pub fn StartServer();
}
