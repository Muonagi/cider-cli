# Contributing

Cider is a CLI sideloading tool meant to work on stock systems. To keep compatibility and maintainability high, there are a few project-specific rules in place.

## Rules

- **No usage of any exploits of any kind**.
- **No contributions related to retrieving any signing certificates owned by companies**.
- **Modifying any hardcoded links should be discussed before changing**.
- **If you're planning on making a large contribution, please open an issue in the repository you're contributing to beforehand**.
- **Your contributions should be licensed appropriately**.
- **Typo contributions are okay**, just make sure they are appropriate.
- **Code cleaning contributions are okay**.

## Contributing to Cider

The project is split into a small CLI app plus shared crates.

| Module                 | Description                                                                                                                   |
| ---------------------- | ----------------------------------------------------------------------------------------------------------------------------- |
| `apps`                 | The `cider` CLI application, built with `clap` and interactive terminal prompts.                                             |
| `crates/plume_core`    | Apple developer authentication, provisioning, certificates, Mach-O helpers, and signing integration.                         |
| `crates/plume_gestalt` | Wrapper for `libMobileGestalt.dylib`, used for obtaining your Mac's UDID for Apple Silicon sideloading.                       |
| `crates/plume_utils`   | Shared bundle, package, device, signing, tweak, and install helpers.                                                         |
| `crates/plume_store`   | Local account and refresh persistence.                                                                                        |

### Building

Most development work only needs Rust and CMake.

You need:
- [Rust](https://rustup.rs/).
- [CMake](https://cmake.org/download/) (and a c++ compiler).

```sh
# Show CLI help
cargo run -p cider -- --help

# Run the test suite for the CLI
cargo test -p cider

# Build the full workspace
cargo build --workspace
```

Runtime notes:

- Linux:
  `usbmuxd` must be installed for device communication.
- macOS:
  Xcode or Command Line Tools are recommended.
- Windows:
  Apple device drivers from iTunes or Apple Devices are usually required.

## Workflow

- Make sure your contributions stay isolated in their own branch, and not `main`.
- If you're planning a large change, open an issue first so the scope can be discussed.
- Keep changes focused. Avoid unrelated refactors while touching signing or developer-service code.
- Add or update tests when changing behavior.
