# LessbadOS

An OS that is, well... less bad.

## Getting started

Clone this repository, and change directory:

```shell
$ git clone https://github.com/Jonxslays/lessbad-os.git
$ cd lessbad-os
```

Have Rust installed - if you are unsure you can verify with:

```shell
$ rustup --version
```

Your output should look something like:

```shell
$ rustup -q --version
rustup 1.25.1 (bb60b1e89 2022-07-12)
info: This is the version for the rustup toolchain manager, not the rustc compiler.
info: The currently active `rustc` version is `rustc 1.65.0-nightly (addacb587 2022-08-24)`
```

If your output doesn't look like that, you can find install instructions
[here](https://www.rust-lang.org/tools/install).

The rust nightly toolchain will be installed automatically (if necessary) when building the project.

Install [`bootimage`](https://github.com/rust-osdev/bootimage), then add the `llvm-tools-preview`
and `rust-src` components:

```shell
$ cargo install bootimage
$ rustup component add llvm-tools-preview rust-src
```

## Running in a VM

If you'd like to run the OS through a virtual machine, [`qemu`](https://www.qemu.org/)
is recommended. It is a fully open source virtual machine emulator.

Installation instructions for `qemu` can be found at:

- [Linux](https://www.qemu.org/download/#linux)
- [MacOS](https://www.qemu.org/download/#macos)
- [Windows](https://www.qemu.org/download/#windows)

Once you have `qemu`, go ahead and run the project with cargo:

```shell
$ cargo run
```

## Attribution

This project would not have been possible without the work of
[Philipp Oppermann](https://github.com/phil-opp) and the
[Writing an OS in Rust](https://os.phil-opp.com/) blog.

Consider giving Phil a [sponsor](https://github.com/sponsors/phil-opp)
for this awesome contribution to open source.

## License

LessbadOS is licensed under the
[MIT License](https://github.com/Jonxslays/lessbad-os/blob/master/LICENSE).
