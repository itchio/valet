use std::{
    env,
    error::Error,
    path::{Path, PathBuf},
};

fn main() {
    let target = env::var("TARGET").unwrap();
    if !target.contains("gnu") {
        eprintln!(
            "mingw_compat requires a GNU toolchain, but target is {:?}",
            target
        );
        std::process::exit(1);
    }

    let out_path = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    gen_mingw_compat(&out_path).unwrap();
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
