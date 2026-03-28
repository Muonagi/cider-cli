# cider account

Manage Apple Developer accounts, teams, certificates, and devices registered with your developer portal.

## Subcommands

### login

Authenticate and save an Apple ID.

```sh
cider account login [-u <EMAIL>] [-p <PASSWORD>]
```

If `-u` or `-p` are omitted, you'll be prompted interactively. 2FA codes are handled via prompt when required.

### logout

Remove the currently selected account from local storage.

```sh
cider account logout
```

### list

Show all saved accounts. The active account is marked.

```sh
cider account list
```

### use

Switch the active account.

```sh
cider account use <EMAIL>
```

### team

Select or persist a development team for the active account.

```sh
cider account team [--email <EMAIL>] [-t <TEAM_ID>]
```

If `-t` is omitted and the account belongs to multiple teams, you'll be prompted to choose.

### export-p12

Export the signing certificate and private key as a `.p12` file. Useful for tools like SideStore or LiveContainer.

```sh
cider account export-p12 [--email <EMAIL>] [-o <OUTPUT>]
```

### certificates

List certificates registered with the active team.

```sh
cider account certificates [-t <TEAM_ID>]
```

### devices

List devices registered with the active team.

```sh
cider account devices [-t <TEAM_ID>]
```

### register-device

Register a new device with the active team.

```sh
cider account register-device [-t <TEAM_ID>] -u <UDID> -n <NAME>
```

### app-ids

List App IDs registered with the active team.

```sh
cider account app-ids [-t <TEAM_ID>]
```

## Examples

```sh
# Login interactively
cider account login

# Login non-interactively
cider account login -u user@example.com -p mypassword

# Switch accounts
cider account list
cider account use dev2@example.com

# Select a team
cider account team -t ABCD1234

# Export certificate
cider account export-p12 -o cert.p12

# Inspect developer portal data
cider account certificates
cider account devices
cider account app-ids

# Register a device
cider account register-device -u ABC123DEF456 -n "My iPhone"
```
