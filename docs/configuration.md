# Configuration & Storage

Cider stores account credentials and refresh data on disk. No configuration file is needed to use the tool; all settings are passed as command-line flags.

## Storage location

| Platform | Path |
|----------|------|
| macOS / Linux | `~/.config/Cider/` |
| Windows | `%APPDATA%\Cider\` |

A legacy path (`~/.config/PlumeImpactor/`) is also checked. If it exists and the new path does not, the legacy path is used.

## Directory layout

```
~/.config/Cider/
  accounts.json           # Saved accounts, team selections, and refresh registrations
  refresh_store/          # Signed app copies saved for refresh
    MyApp-<uuid>.app/
    OtherApp-<uuid>.app/
```

## accounts.json

The account store holds all persisted state in a single JSON file:

```json
{
  "selected_account": "user@example.com",
  "accounts": {
    "user@example.com": {
      "email": "user@example.com",
      "first_name": "John",
      "adsid": "...",
      "xcode_gs_token": "...",
      "team_id": "ABCD1234"
    }
  },
  "refreshes": {
    "ABC123DEF456": {
      "udid": "ABC123DEF456",
      "name": "iPhone",
      "account": "user@example.com",
      "is_mac": false,
      "apps": [
        {
          "path": "~/.config/Cider/refresh_store/MyApp-<uuid>.app",
          "name": "MyApp",
          "bundle_id": "com.example.app",
          "scheduled_refresh": "2025-04-15T10:30:00Z"
        }
      ]
    }
  }
}
```

- **selected_account**: the email of the currently active account (used by `cider sign --apple-id` and other commands that need authentication).
- **accounts**: map of email to saved credentials. Tokens are obtained during `cider account login`.
- **team_id**: cached after running `cider account team`. Avoids re-prompting on each sign.
- **refreshes**: map of device UDID to its refresh registrations. Each app entry tracks its signed bundle path, bundle identifier, and when it should next be refreshed.

## Refresh store

Apps saved with `cider sign --refresh --install` are copied to `refresh_store/` as `.app` directories named `<AppName>-<uuid>.app`. These bundles include the embedded provisioning profile used during signing. When `cider refresh run` triggers a re-sign, the stored bundle is updated in place.

## Credentials

Account tokens (ADSID and Xcode GS token) are stored in `accounts.json`. These tokens are used to authenticate with Apple Developer services without re-entering your password. Logging out (`cider account logout`) removes the selected account and its tokens from the file.
