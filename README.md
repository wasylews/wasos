# wasos

A small RISC-V kernel written in Rust.

It boots under SBI (OpenSBI on QEMU's `virt` machine), zeroes BSS, and prints to the console through the SBI legacy putchar call. The whole thing is `no_std`.

## Status

Early. It boots and prints. Not much else yet.

## Build

```sh
cargo build --release
```

Needs the nightly toolchain and the `riscv64gc-unknown-none-elf` target. Both are pinned in `rust-toolchain.toml`, so `rustup` installs them on the first build.

## Run

```sh
qemu-system-riscv64 -machine virt -bios default -kernel target/riscv64gc-unknown-none-elf/release/wasos
```

Expected output:

```
Hello world from WasOS!
```

## Layout

- `src/main.rs`: entry point, BSS setup, panic handler
- `src/io.rs`: `print!` / `println!` over the SBI console
- `src/sbi.rs`: SBI `ecall` wrapper
- `src/kernel.ld`: linker script
