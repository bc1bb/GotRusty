# GotRusty

Simple HTTP server made in Rust.

**Please do not use in prod.**

As per `wrk` (8t,8c) on a MacBook Pro M1, this server handles 1.7k req/s. (terrible)

## Config
Environment variables can be used to set any settings including config file path: `GR_Config`: `/etc/gotrusty/Config.toml`

(to set a setting from env var prefix with `GR_`)

- `addr` (defaulted to `127.0.0.1`),
- `port` (defaulted to `1337`),
- `file_root` (defaulted to `.`),
- `errors_root` (defaulted to `./error`),
- `mime_default` (defaulted to `text/plain`).