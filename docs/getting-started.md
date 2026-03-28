# Getting Started

## Prerequisites

- [Rust](https://rustup.rs/) (via rustup)
- [CMake](https://cmake.org/download/) and a C++ compiler

### Platform-specific requirements

| Platform | Requirement |
|----------|-------------|
| Linux    | `usbmuxd` must be installed and running for device communication |
| Windows  | Apple device drivers from iTunes or Apple Devices |
| macOS    | Xcode or Command Line Tools recommended for building from source |

## Building

```sh
# Debug build
cargo build -p cider

# Release build (optimized for size)
PROFILE=release make dist
```

The release binary lands in `dist/cider`.

## First steps

### 1. Log in to your Apple account

```sh
cider account login
```

You'll be prompted for your Apple ID and password. 2FA is handled interactively.

### 2. Select a team (if you have multiple)

```sh
cider account team
```

### 3. Connect and trust a device

```sh
cider device trust
```

### 4. Sign and install an app

```sh
cider sign MyApp.ipa --apple-id --install
```

## Common workflows

### Apple ID signing with installation

```sh
cider account login -u user@example.com
cider account team -t ABCD1234
cider sign MyApp.ipa --apple-id --install
```

### Ad-hoc signing with export

```sh
cider sign MyApp.ipa --adhoc --output MyApp-signed.ipa
```

### Sign with customizations

```sh
cider sign MyApp.ipa --apple-id --install \
  --custom-name "My App" \
  --custom-icon icon.png \
  --custom-version "2.0" \
  --file-sharing \
  --ipad-fullscreen
```

### Inject tweaks

```sh
cider sign MyApp.ipa --apple-id --install \
  --tweak tweak1.deb \
  --tweak tweak2.dylib \
  --bundle Framework.framework
```

### Save for refresh and run later

```sh
cider sign MyApp.ipa --apple-id --install --refresh
cider refresh list
cider refresh run
```

### Multi-account workflow

```sh
cider account login -u dev1@example.com
cider account login -u dev2@example.com
cider account list
cider account use dev2@example.com
cider sign app.ipa --apple-id --install
```

## Next steps

- [Command Reference](commands/) for full details on every command and flag.
- [How Signing Works](signing.md) for an explanation of the signing pipeline.
- [Configuration & Storage](configuration.md) for where data is stored on disk.
