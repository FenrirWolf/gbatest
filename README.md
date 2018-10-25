
# First Time Setup

[ketsuban](https://github.com/ketsuban) is the wizard who explained to me how to
do this stuff.

1) Install `devkitpro`. They have a graphical installer for Windows, or you can
   use pacman or whatever for linux things I guess. The goal here, among other
   things, is to have a `binutils` setup that's targeting `arm-none-eabi`. We'll
   also use some of their tools that are specific to GBA development so if you
   for some reason already have the appropriate `binutils` then you probably
   still want devkitpro.
   * On Windows you'll want something like `C:\devkitpro\devkitARM\bin` and
     `C:\devkitpro\tools\bin` to be added to your PATH. I'm not sure on the
     directories for other systems. If you know then file a PR with the info.

2) Next we use `cargo install cargo-xbuild` to get that all setup.

3) Create a binary project. We're going to want nightly rust for this, so if you
   don't already have it set to default to nightly you should [set that
   up](https://github.com/rust-lang-nursery/rustup.rs#the-toolchain-file) for
   this project.

4) Clone this repo. It has an appropriate `main.rs` that will draw three test
   dots as well as other support files:
  * crt0.s
  * linker.ld
  * thumbv4-none-eabi.json

5) Be aware that you must run `arm-none-eabi-as crt0.s -o crt0.o` any time the
   `crt0.s` file changes. The provided `crt0.o` file in this repo is correct,
   but if you make any changes to `crt0.s` you'll need to rebuild `crt0.o`.

6) Build with `cargo xbuild --target thumbv4-none-eabi.json`
  * The file extension is significant, and `cargo xbuild` takes it as a flag to
    compile dependencies with the same sysroot, so you can include crates
    normally. Well, `no_std` crates that can run inside a GBA at least.
  * This generates an ELF binary that some emulators can run directly (which is
    helpful because it has debug symbols).

7) Also you can patch up the output to be a "real" ROM file:
  * `arm-none-eabi-objcopy -O binary target/thumbv4-none-eabi/debug/gbatest output.gba`
  * `gbafix output.gba`

8) Time to read the [Tonc](https://www.coranac.com/tonc/text/toc.htm) tutorial
   and convert all the C code you see into rust code.
