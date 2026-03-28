# cider inspect

Inspect and patch Mach-O binaries.

```
cider inspect <BINARY> <ACTION>
```

## Actions

Exactly one action must be specified:

| Flag | Description |
|------|-------------|
| `--entitlements` | Print the binary's embedded entitlements as XML |
| `--list-dylibs` | List all dynamic library load commands |
| `--add-dylib <PATH>` | Add a new dylib load command (e.g., `@rpath/MyLib.dylib`) |
| `--replace-dylib <OLD> <NEW>` | Replace an existing dylib path with a new one (takes two values) |
| `--sdk-version <VERSION>` | Set the SDK version in the Mach-O header (e.g., `26.0.0`) |

## Examples

```sh
# View entitlements
cider inspect MyBinary --entitlements

# List linked dylibs
cider inspect MyBinary --list-dylibs

# Add a dylib
cider inspect MyBinary --add-dylib @rpath/Custom.dylib

# Replace a dylib path
cider inspect MyBinary --replace-dylib /usr/lib/old.dylib @rpath/new.dylib

# Patch SDK version
cider inspect MyBinary --sdk-version 26.0.0
```
