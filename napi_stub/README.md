# napi_stub

This is a `DELAYLOAD` hack for N-API + the mingw64 toolchain, used only on
Windows.

## Why

MVSC supports building dynamic libraries that rely on symbols "to be figured
out later" (at runtime). It automatically generates thunks that are filled out
at load time.

But mingw64 doesn't. And we need mingw64 because golang.

So this is a handrolled equivalent.

## How

`generated/napi_stub.c` contains stubs for all the N-API functions, which
makes the mingw linker happy.

The stubs are large enough that we can patch absolute-address JMPs into them
later on.

`generated/setup.rs` contains `winhook` invocations that look up the *actual*
addresses of the N-API functions and patches the thunks to forward the call
to the actual function.

## Regenerating files

The `generated/` files are generated from `nj-sys` by `gen_napi_stub`
(subfolder).

To regen, just run:

```shell
$ (cd gen_napi_stub && cargo run)
```

This used to be a Cargo `build.rs` script, but it took forever, since it
pulls some rust-analyzer stuff, serde-derive etc.