# Rust Game Template
[![stability-experimental](https://img.shields.io/badge/stability-experimental-orange.svg)](https://github.com/emersion/stability-badges#experimental) [![ios](https://github.com/Wandalen/rust_game_template/actions/workflows/iOS.yml/badge.svg)](https://github.com/Wandalen/rust_game_template/actions/workflows/iOS.yml) [![android](https://github.com/Wandalen/rust_game_template/actions/workflows/Android.yml/badge.svg)](https://github.com/Wandalen/rust_game_template/actions/workflows/Android.yml) [![desktop](https://github.com/Wandalen/rust_game_template/actions/workflows/Desktop.yml/badge.svg)](https://github.com/Wandalen/rust_game_template/actions/workflows/Desktop.yml) [![web](https://github.com/Wandalen/rust_game_template/actions/workflows/Web.yml/badge.svg)](https://github.com/Wandalen/rust_game_template/actions/workflows/Web.yml)

Neutral cross-platform Rust game template.

## Build tool

This project uses [cargo-make](https://github.com/sagiegurari/cargo-make) task runner. It's required to build the project. To install it run:

```
cargo install cargo-make
```

## Platforms

Supported platforms:

- [Desktop](./doc/platform/Desktop.md) ( _default_ )
- [Web](./doc/platform/Web.md)
- [Android](./doc/platform/Android.md)
- [iOS](./doc/platform/iOS.md)

To run the project on default platform execute:

```
cargo make
```
