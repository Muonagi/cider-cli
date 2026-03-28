# cider refresh

Manage saved refresh registrations. When you sign an app with `--refresh --install`, Cider saves a copy of the signed bundle so it can be re-signed and reinstalled later before the provisioning profile expires.

## How it works

1. During `cider sign --apple-id --install --refresh`, the signed `.app` is copied to `~/.config/Cider/refresh_store/`.
2. The provisioning profile's expiration date is parsed and a scheduled refresh time is set (3 days before expiry).
3. Running `cider refresh run` checks each saved app:
   - If the certificate has changed or the app was uninstalled, the app is fully re-signed and reinstalled.
   - If only the provisioning profile changed, the embedded profile is updated on-device without a full reinstall.
4. The scheduled refresh timestamp is updated after each run.

## Subcommands

### list

Show all saved refresh registrations grouped by device.

```sh
cider refresh list
```

### run

Run pending refreshes.

```sh
cider refresh run [--udid <UDID>] [--bundle-id <BUNDLE_ID>]
```

Without filters, all due registrations are refreshed. Use `--udid` to limit to a single device or `--bundle-id` to refresh a specific app.

### remove

Remove a saved refresh registration.

```sh
cider refresh remove [--udid <UDID>] [--bundle-id <BUNDLE_ID>]
```

## Examples

```sh
# Save an app for refresh during signing
cider sign MyApp.ipa --apple-id --install --refresh

# List all refresh registrations
cider refresh list

# Run all due refreshes
cider refresh run

# Refresh a specific device
cider refresh run --udid ABC123DEF456

# Refresh a specific app
cider refresh run --bundle-id com.example.myapp

# Remove a registration
cider refresh remove --udid ABC123DEF456 --bundle-id com.example.myapp
```
