# cider sign

Sign an iOS app, install it to a device, or export a re-signed IPA.

```
cider sign <PACKAGE> [OPTIONS]
```

`<PACKAGE>` is a path to an `.ipa` or `.app` bundle.

## Signing modes

Exactly one mode must be chosen:

| Flag | Description |
|------|-------------|
| `--apple-id` | Sign with your Apple Developer account (certificate + provisioning profile) |
| `--adhoc` | Ad-hoc signing (no Apple account needed) |
| `--no-modify` | Skip signing entirely; extract, install, or export only |
| `--pem <FILES>` | Sign with explicit PEM certificate and key files (one or more paths) |

## Output

| Flag | Description |
|------|-------------|
| `--install` | Install the signed app to a connected device after signing |
| `-o, --output <PATH>` | Export the signed bundle as a new `.ipa` file (conflicts with `--install`) |

## Device targeting

| Flag | Description |
|------|-------------|
| `--udid <UDID>` | Target a specific device by its UDID |
| `--mac` | Target the local Mac for sideloading (macOS Apple Silicon only) |

If neither is specified and `--install` is used, you'll be prompted to select a device interactively.

## Bundle modifications

| Flag | Description |
|------|-------------|
| `--custom-identifier <ID>` | Override the bundle identifier |
| `--custom-name <NAME>` | Override the display name |
| `--custom-version <VERSION>` | Override version strings (both `CFBundleShortVersionString` and `CFBundleVersion`) |
| `--custom-icon <IMAGE>` | Replace app icons (auto-resized to 120x120 and 152x152 PNG) |
| `--custom-entitlements <PLIST>` | Inject custom entitlements (requires `--single-profile`) |

## Compatibility flags

| Flag | Effect |
|------|--------|
| `--support-minimum-os-version` | Set `MinimumOSVersion` to 7.0 |
| `--file-sharing` | Enable `UIFileSharingEnabled` and `UISupportsDocumentBrowser` |
| `--ipad-fullscreen` | Set `UIRequiresFullScreen` to true |
| `--game-mode` | Set `GCSupportsGameMode` to true |
| `--pro-motion` | Set `CADisableMinimumFrameDurationOnPhone` to true |
| `--liquid-glass` | Apply Liquid Glass compatibility tweaks |
| `--ellekit` | Replace CydiaSubstrate with ElleKit |

## Injection

| Flag | Description |
|------|-------------|
| `--tweak <PATH>` | Inject a `.deb` or `.dylib` file (can be specified multiple times) |
| `--bundle <PATH>` | Inject a `.framework`, `.bundle`, or `.appex` (can be specified multiple times) |

## Provisioning

| Flag | Description |
|------|-------------|
| `--provision <FILE>` | Use a manually-supplied provisioning profile |
| `--single-profile` | Only register the main bundle (skip extensions) |

## Refresh

| Flag | Description |
|------|-------------|
| `--refresh` | Save the signed app for later refresh (requires `--install` and a device) |

See [cider refresh](refresh.md) for managing saved refresh registrations.

## Examples

```sh
# Sign and install with Apple ID
cider sign MyApp.ipa --apple-id --install

# Ad-hoc sign and export
cider sign MyApp.ipa --adhoc --output MyApp-signed.ipa

# Install to a specific device with tweaks
cider sign MyApp.ipa --apple-id --install --udid ABC123 \
  --tweak hack.deb --tweak lib.dylib

# Custom metadata and compatibility flags
cider sign MyApp.ipa --apple-id --install \
  --custom-name "Patched" --custom-icon icon.png \
  --file-sharing --ipad-fullscreen

# Save for refresh
cider sign MyApp.ipa --apple-id --install --refresh

# Sign with explicit PEM files
cider sign MyApp.ipa --pem cert.pem --pem key.pem --install

# Install to local Mac (Apple Silicon)
cider sign MyApp.app --apple-id --install --mac
```
