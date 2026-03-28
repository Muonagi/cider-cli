# Platform Notes

Cider runs on macOS, Linux, and Windows. Most features work the same across platforms with a few exceptions.

## macOS

- **Apple Silicon sideloading**: the `--mac` flag lets you sign and install apps directly to your local Mac. This is only available on `aarch64` Macs. Cider uses `MobileGestalt` to retrieve the Mac's UDID.
- **Xcode or Command Line Tools** are recommended for building from source (provides the C++ toolchain).
- USB device communication works out of the box via the system's `usbmuxd`.

## Linux

- **`usbmuxd`** must be installed and running for device communication over USB. Install it from your distribution's package manager (e.g., `apt install usbmuxd`).
- The `--mac` flag is not available.
- All other features (signing, export, refresh, inspect) work identically to macOS.

## Windows

- **Apple device drivers** from iTunes or Apple Devices are typically required for USB device communication.
- The `--mac` flag is not available.
- Data is stored in `%APPDATA%\Cider\` instead of `~/.config/Cider/`.
- All signing and export features work identically to other platforms.

## Building on any platform

Requirements are the same everywhere:

- [Rust](https://rustup.rs/) (via rustup)
- [CMake](https://cmake.org/download/) and a C++ compiler

```sh
cargo build -p cider
```
