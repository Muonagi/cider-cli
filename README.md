# Impactor

[![CI](https://github.com/Muonagi/impactor-cli/actions/workflows/ci.yml/badge.svg)](https://github.com/Muonagi/impactor-cli/actions/workflows/ci.yml)
[![Release](https://github.com/Muonagi/impactor-cli/actions/workflows/release.yml/badge.svg)](https://github.com/Muonagi/impactor-cli/actions/workflows/release.yml)

Impactor is a CLI-first iOS signing and sideloading tool for macOS, Linux, and Windows. This repository now ships a single terminal application, `impactor`, instead of a desktop GUI.

## What It Does

- Sign and install `.ipa` and `.app` bundles.
- Use Apple ID signing, ad-hoc signing, or a no-modify install/export flow.
- Customize app metadata, entitlements, icons, and compatibility flags.
- Inject tweaks and extra bundles.
- Manage saved Apple Developer accounts and team selection.
- Export `.p12` certificates for tools like SideStore and LiveContainer.
- Pair devices, inspect installed supported apps, and install pairing files.
- Save signed apps for explicit refresh workflows and run refreshes from the CLI.
- Inspect and patch Mach-O binaries.

## Quick Start

Build the CLI:

```sh
cargo build -p impactor
```

See the command surface:

```sh
impactor --help
```

Common flows:

```sh
# Login and persist your Apple account
impactor account login

# Select the active team for the selected account
impactor account team

# Sign and install an IPA with Apple ID signing
impactor sign MyApp.ipa --apple-id --install

# Sign ad-hoc and export a new IPA
impactor sign MyApp.ipa --adhoc --output MyApp-signed.ipa

# Pair a device and inspect supported installed apps
impactor device trust
impactor device apps

# Manage saved refresh registrations
impactor refresh list
impactor refresh run

# Inspect Mach-O metadata
impactor inspect MyBinary --entitlements
```

## Command Overview

- `impactor sign`
  Sign an app, install it, export a new IPA, or save it for later refresh.
- `impactor account`
  Login, switch accounts, pick teams, inspect developer data, and export certificates.
- `impactor device`
  List devices, trust a device, install an app, list supported apps, and install pairing files.
- `impactor refresh`
  List saved refresh entries, run refresh manually, or remove saved refresh registrations.
- `impactor inspect`
  Inspect entitlements or patch Mach-O load commands and SDK versions.

## Platform Notes

- Linux:
  `usbmuxd` must be installed and running for device communication.
- Windows:
  Apple device drivers from iTunes or Apple Devices are typically required.
- macOS:
  Xcode or Command Line Tools are recommended for building from source.

## Building

Requirements:

- [Rust](https://rustup.rs/)
- [CMake](https://cmake.org/download/)

Useful commands:

```sh
# Build the CLI
cargo build -p impactor

# Run the CLI
impactor --help

# Build a distributable binary into ./dist
make dist
```

## How It Works

Impactor follows the same broad path as Xcode-based sideloading:

1. Authenticate with Apple Developer services.
2. Register the target device when needed.
3. Create or reuse a signing identity.
4. Register the app ID and request provisioning profiles.
5. Apply requested bundle modifications.
6. Sign the app and install or export it.

Without a paid developer account, Apple still imposes the normal free-account limits.

## Credits

This project is a modified version of [Impactor by CLARATION](https://github.com/CLARATION/Impactor).

## License

This project is licensed under MIT. See `LICENSE` for details. Some bundled components may use different licenses in their own directories.
