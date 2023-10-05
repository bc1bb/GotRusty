# GotRusty

HTTP server made in Rust ~~with no dependency~~.

**Please do not use in prod.**

## Config
Environment variables can be used to set any settings including config file path: `GR_Config`: `/etc/gotrusty/Config.toml`

(to set a setting from env var prefix with `GR_`)

- `addr` (defaulted to `127.0.0.1`),
- `port` (defaulted to `1337`),
- `file_root` (defaulted to `.`),
- `errors_root` (defaulted to `./error`),
- `mime_default` (defaulted to `text/plain`).