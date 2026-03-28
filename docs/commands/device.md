# cider device

Manage connected iOS devices: list, pair, install apps, and push pairing files.

## Common flags

| Flag | Description |
|------|-------------|
| `-u, --udid <UDID>` | Target a specific device by UDID |
| `-m, --mac` | Target the local Mac (macOS Apple Silicon only) |

When neither flag is given and multiple devices are connected, you'll be prompted to choose.

## Subcommands

### list

Show all connected devices.

```sh
cider device list
```

### trust

Pair and establish trust with a device. Required before installing apps.

```sh
cider device trust [-u <UDID>] [-m]
```

On macOS with Apple Silicon, `-m` adds the local Mac as a target.

### apps

List apps installed on the device that support pairing-file operations (e.g., SideStore, Feather).

```sh
cider device apps [-u <UDID>] [-m]
```

### install

Install an `.ipa` or `.app` bundle directly to a device.

```sh
cider device install <PATH> [-u <UDID>] [-m]
```

### pairing

Push a pairing file to a supported app on the device.

```sh
cider device pairing [-u <UDID>] [-m] [--app <BUNDLE_ID>] [--path <PATH>]
```

If `--app` is omitted, you'll be prompted to choose from the supported apps installed on the device. The `--path` flag overrides the default destination path inside the app container.

## Supported apps for pairing

The following apps are detected and have default pairing-file paths:

| App | Bundle ID pattern | Default pairing path |
|-----|-------------------|---------------------|
| Antrag | `thewonderofyou.antrag2` | `/Documents/pairingFile.plist` |
| Feather | `thewonderofyou.Feather` | `/Documents/pairingFile.plist` |
| Protokolle | `thewonderofyou.syslog` | `/Documents/pairingFile.plist` |
| SideStore | `com.SideStore.SideStore` | `/Documents/ALTPairingFile.mobiledevicepairing` |
| StikDebug | `com.stik.sj` | `/Documents/pairingFile.plist` |
| SparseBox | `com.kdt.SparseBox` | `/Documents/pairingFile.plist` |
| EnsWilde | `com.yangjiii.EnsWilde` | `/Documents/pairingFile.plist` |
| ByeTunes | `com.EduAlexxis.MusicManager` | `/Documents/pairing file/pairingFile.plist` |
| StikStore | `me.stik.store` | `/Documents/pairingFile.plist` |
| StikStore2 | `app.stik.store` | `/Documents/pairingFile.plist` |

## Examples

```sh
# List connected devices
cider device list

# Trust a device interactively
cider device trust

# Trust a specific device
cider device trust -u ABC123DEF456

# List supported apps on a device
cider device apps -u ABC123DEF456

# Install an IPA directly
cider device install MyApp.ipa -u ABC123DEF456

# Install to local Mac
cider device install MyApp.app -m

# Push pairing file to SideStore
cider device pairing -u ABC123 --app com.SideStore.SideStore
```
