# Cargo

```bash
cargo build [--release]

cargo doc --open
```

```toml
# Cargo.toml
[package]
name = "pack_name" # the name of the package
version = "pack_version" # the current version, obeying semver
authors = ["name <email@example.com>"]
edition = "2018"

[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3

[dependencies]

[workspace]
members = ["..."]
```
