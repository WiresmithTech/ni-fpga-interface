# Cross Compiling

Cross-compiling for cRIO targets is expected to be a common case with this library to target compactRIOs.

Most of this is automatic when setting up rust for cross compiling in general.

The main special consideration is that we must use the cc crate internally to compile the C code generated from the NI FPGA C Interface.

## Sysroot

When cross-compiling you must first download the sysroot for the LinuxRT version you are targetting.

Then you must point cargo to the linker in a .cargo/config file. You must also specify the sysroot here.

```
[target.x86_64-unknown-linux-gnu]
linker = "C:\\build\\2023\\x64\\sysroots\\x86_64-w64-mingw32\\usr\\bin\\x86_64-nilrt-linux\\x86_64-nilrt-linux-gcc"
ar = "C:\\build\\2023\\x64\\sysroots\\x86_64-w64-mingw32\\usr\\bin\\x86_64-nilrt-linux\\x86_64-nilrt-linux-ar"
rustflags = [
"-C", "link-arg=--sysroot=C:\\build\\2023\\x64\\sysroots\\core2-64-nilrt-linux"
]
```

`cc` still needs the sysroot for the compile stage which must be provided seperately.

There are two solutions to this.

1. You can specify the `CFLAG` env variable which cc reads. e.g. `CFLAGS="--sysroot=C:\\build\\2023\\x64\\sysroots\\core2-64-nilrt-linux" cargo build --target x86_64-unknown-linux-gnu`
2. The interface build API has a `sysroot` command. If set, it will set the sysroot flag when invoking cc.

## C Compiler

`cc` appears to be able to detect the compiler at this stage with no further setup.