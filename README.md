# valet

valet exists primarily to provide [butler](https://github.com/itchio/butler) as
a native N-API addon for [itch](https://github.com/itchio/itch).

## Distribution

valet is published on npm as [@itchio/valet](https://www.npmjs.com/package/@itchio/valet).
This includes the JavaScript and TypeScript bindings parts.

The bindings include not only "the valet interface" (`initialize`, `newConn`,
`send`, `recv`), but also JSON-RPC support code and typings for the butlerd
protocol, see [the docs](http://docs.itch.ovh/butlerd/master/#/).

As part of its `postinstall` script, it downloads the relevant .zip archive
from [GitHub releases](https://github.com/itchio/valet/releases) and extracts
it to `artifacts/x86_64-linux`, for example.

The idea is that, for whichever project `valet` is used in (which, again,
should only be `itch` ideally), it's "just a regular npm dependency", just like
the `electron` package.

On first install, it downloads the binary bits it needs. If the version
changes, it redownloads the binary bits. But apart from that, it should stay
out of everyone's way as much as possible.

That means you don't need Rust *or* Go installed to develop `itch`.

Even though valet includes a fair bit of Rust, is *not* published on
<https://crates.io/> at all.

## How does this even work

`valet` uses N-API to "be a native addon". Which means it doesn't need to be
recompiled to be compatible with various versions of Node.JS and Electron
going forward.

The N-API part is all Rust - it uses [nj-sys](https://lib.rs/crates/nj-sys)
(itself generated with `bindgen`), but solely to get the N-API function
signatures. 

Then there's the `napi` crate which is a thin Rusty layer on top of N-API,
providing traits like `FromNapi` and `ToNapi`, and convenient methods to
manipulate object properties, or create class-like objects with a `this`
state. Among other things, it does an ungodly amount of work dealing with
closures so you don't have to.

The `butler` part is, of course, Go. There's a small Go module named
`libbutler` which imports parts of `butler` and exposes them as a set of C
functions and types, using cgo.

So:

  * User code requires npm package `@itchio/valet`
  * `index.js` detects the platform and requires the relevant `index.node` file
    * This can be overriden to an absolute path - and will be in release builds
      of itch, because packaging / code-signing etc.
  * The Rust code registers itself with the Node.JS runtime using N-API
  * Node.JS ends up calling into the Rust code to get the exports
  * The Rust code exports an object (see the typings for documentation) which
  has methods which end up calling Go code over cgo.

## The heck is an `index.node` file

`index.node` is effectively just a dynamic library (or shared library, or
dylib, or DLL, whatever you want to call it) which Node.JS (or Electron)
loads.

But our dynamic library must use some symbols defined only in `node.exe` (or
`electron.exe`, or `itch.exe`, etc.). On Linux & macOS, this isn't a problem
because their linkers allows "undefined symbols" and "dynamic lookup".

## Any Windows-specific hacks?

On Windows, if we used MSVC, this wouldn't be a problem either, because this
is what `/DELAYLOAD` is used for. And that's usually how native addons are
compiled.

But, because there's Go code involved, we *have* to use a mingw (GCC) toolchain,
which does not support `/DELAYLOAD`. That's why, on windows, we use `napi_stub`,
which exports the functions we need, as stubs, and patches jumps to the *real*
functions when our module gets loaded, using `GetProcAddress` and x86 instruction
templates (one for i686, and one for x86_64).

## Debugging the thing

On Linux & macOS, just use GDB or LLDB.

You can even use Valgrind on Linux, although having Go code in there will
pollute the output *a lot*. I wasn't able to compile it with ASan or MSan
due to obscure linker errors.

On windows, it's kind of a headache. Node.JS (or Electron) is compiled using
an MSVC toolchain. In fact, I'm not aware of any other N-API modules that are
compiled with GCC (probably because people think patching code at runtime is
a silly thing to do - but that's pretty much what MSVC's `/DELAYLOAD` does under
the hook anyway, so..).

The bad news is, a mingw64 (GCC) toolchain will only produce DWARF debug
information, so tools like WinDbg won't be able to see into `valet`'s code.

And Node.JS/Electron only come with PDB (MSVC) debug info (if you can hunt
them down... and extract the sources in the right place, or mess with the
search paths), so MSYS2-GDB for example won't see into the Node code.

The solution is, of course, to use an obscure tool developed for the D language
(I think?), [cv2pdb](https://github.com/rainers/cv2pdb). Use it directly
on the `index.node` file, so you have an `index.pdb` next to it.

Yes, it actually works. To grab `cv2pdb`, go to its `Releases` tab to find
the AppVeyor page, instead of attempting to build it yourself (it's not worth
the trouble).

