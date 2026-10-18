# CLAW OS 🦀

> A tiny operating system written in Rust, built from scratch following os.phil-opp.com. Made for Terra by Hack Club.

[![ci](https://github.com/Tempoest/Claw-OS/actions/workflows/ci.yml/badge.svg)](https://github.com/Tempoest/Claw-OS/actions/workflows/ci.yml)

## Run it (QEMU is the only dependency)

- Linux: `sudo apt install qemu-system-x86`
- MacOS: `brew install qemu`
- Windows: `winget install qemu`

Linux / macOS:
`wget https://github.com/Tempoest/Claw-OS/releases/latest/download/claw_os.imgqemu-system-x86_64 -drive format=raw,file=claw_os.img`

Windows PowerShell:
`Invoke-WebRequest https://github.com/Tempoest/Claw-OS/releases/latest/download/claw_os.img -OutFile claw_os.imgqemu-system-x86_64 -drive format=raw,file=claw_os.img`

## What works so far

- Boots from a BIOS disk image
- VGA text output, println!, scrolling, on-screen panics
- Serial output (COM1)
- Paging implementation: virtual-to-physical address translation
- Physical memory management: frame allocator for allocating and managing frames
- Page table manipulation: support for `map_to` to create custom virtual-to-physical mappings
- Test framework with no std: cargo test boots QEMU and runs tests inside the OS
- CI: every push — GitHub Actions builds the kernel and runs the test suite in a VM

## Build from source

Toolchain installs automatically from rust-toolchain.toml

You also need `cargo install bootimage` (once) and QEMU.
