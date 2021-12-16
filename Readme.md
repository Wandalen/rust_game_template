# Rust Game Template
[![stability-experimental](https://img.shields.io/badge/stability-experimental-orange.svg)](https://github.com/emersion/stability-badges#experimental) [![ios](https://github.com/Wandalen/rust_game_template/actions/workflows/IosPush.yml/badge.svg)](https://github.com/Wandalen/rust_game_template/actions/workflows/IosPush.yml) [![android](https://github.com/Wandalen/rust_game_template/actions/workflows/AndroidPush.yml/badge.svg)](https://github.com/Wandalen/rust_game_template/actions/workflows/AndroidPush.yml) [![desktop](https://github.com/Wandalen/rust_game_template/actions/workflows/DesktopPush.yml/badge.svg)](https://github.com/Wandalen/rust_game_template/actions/workflows/DesktopPush.yml) [![web](https://github.com/Wandalen/rust_game_template/actions/workflows/WebPush.yml/badge.svg)](https://github.com/Wandalen/rust_game_template/actions/workflows/WebPush.yml)

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

## Experiment2