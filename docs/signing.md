# How Signing Works

Cider follows the same broad path as Xcode-based sideloading. This page describes each phase of the signing pipeline.

## Overview

1. Load and prepare the app bundle.
2. Apply requested modifications.
3. Register with Apple Developer services (Apple ID mode only).
4. Code-sign all binaries.
5. Install to device or export as IPA.

## Phase 1: Bundle preparation

The input `.ipa` or `.app` is loaded into a temporary working directory. For IPAs, the archive is extracted and the embedded `.app` bundle is located. All nested bundles (app extensions, frameworks) are collected and sorted by directory depth so they are processed innermost-first during signing.

## Phase 2: Bundle modifications

Before signing, Cider applies any requested changes:

- **Metadata**: custom name, version, and bundle identifier are written to `Info.plist`. A custom bundle identifier is also propagated to app extensions.
- **Icons**: a custom icon image is resized to 120x120 (`@2x`) and 152x152 (`@2x~ipad`) PNG and placed in the bundle.
- **Compatibility flags**: each flag sets a specific `Info.plist` key (see [sign command flags](commands/sign.md#compatibility-flags)).
- **Tweaks**: `.deb` files are extracted and their dylibs injected. Raw `.dylib` files are copied directly. Extra `.framework`, `.bundle`, or `.appex` directories are embedded.
- **App-specific handling**: certain apps (AltStore, SideStore, LiveContainer, StikStore) have certificates and metadata embedded automatically when detected by bundle identifier.

## Phase 3: Apple Developer registration (Apple ID mode)

When signing with `--apple-id`, Cider authenticates with Apple and performs the following against the developer portal:

1. **Device registration** — the target iOS device UDID is registered with the team (skipped when targeting a Mac).
2. **App ID registration** — the main bundle identifier and each extension's identifier are registered. With `--single-profile`, only the main bundle is registered.
3. **Provisioning profile request** — a development provisioning profile is fetched for each registered App ID. Profiles are embedded in the corresponding bundle during signing.

## Phase 4: Code signing

A signing identity is built from either the Apple-issued certificate (Apple ID mode) or explicit PEM files. The `apple-codesign` crate applies the code signature to every Mach-O binary in the bundle, including frameworks and app extensions. Provisioning profiles are embedded alongside each signed bundle.

In ad-hoc mode, binaries are signed without a certificate or provisioning profile.

## Phase 5: Install or export

- **Install** (`--install`): the signed bundle is pushed to the device via the `idevice` protocol. A progress bar tracks the transfer.
- **Export** (`--output`): the signed bundle is zipped back into an `.ipa` file.
- **Refresh** (`--refresh`): a copy of the signed app is saved to `~/.config/Cider/refresh_store/` for later re-signing (see [cider refresh](commands/refresh.md)).

## Free account limitations

Without a paid Apple Developer account, the normal free-account limits apply: apps are signed with a 7-day provisioning profile and a maximum of three active app IDs at a time. The refresh workflow helps by re-signing before expiry.
