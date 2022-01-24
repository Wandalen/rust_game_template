# Rust Game Template
[![stability-experimental](https://img.shields.io/badge/stability-experimental-orange.svg)](https://github.com/emersion/stability-badges#experimental) [![ios](https://img.shields.io/github/workflow/status/Wandalen/rust_game_template/IosPush?label=ios)](https://github.com/Wandalen/rust_game_template/actions/workflows/IosPush.yml) [![android](https://img.shields.io/github/workflow/status/Wandalen/rust_game_template/AndroidPush?label=android)](https://github.com/Wandalen/rust_game_template/actions/workflows/AndroidPush.yml) [![desktop](https://img.shields.io/github/workflow/status/Wandalen/rust_game_template/DesktopPush?label=desktop)](https://github.com/Wandalen/rust_game_template/actions/workflows/DesktopPush.yml) [![web](https://img.shields.io/github/workflow/status/Wandalen/rust_game_template/WebPush?label=web)](https://github.com/Wandalen/rust_game_template/actions/workflows/WebPush.yml)

<!-- [![stability-experimental](https://img.shields.io/badge/stability-experimental-orange.svg)](https://github.com/emersion/stability-badges#experimental) [![ios](https://github.com/Wandalen/rust_game_template/actions/workflows/IosPush.yml/badge.svg)](https://github.com/Wandalen/rust_game_template/actions/workflows/IosPush.yml) [![android](https://github.com/Wandalen/rust_game_template/actions/workflows/AndroidPush.yml/badge.svg)](https://github.com/Wandalen/rust_game_template/actions/workflows/AndroidPush.yml) [![desktop](https://github.com/Wandalen/rust_game_template/actions/workflows/DesktopPush.yml/badge.svg)](https://github.com/Wandalen/rust_game_template/actions/workflows/DesktopPush.yml) [![web](https://github.com/Wandalen/rust_game_template/actions/workflows/WebPush.yml/badge.svg)](https://github.com/Wandalen/rust_game_template/actions/workflows/WebPush.yml) -->

Neutral cross-platform Rust game template.

## Build tool

This project uses [cargo-make](https://github.com/sagiegurari/cargo-make) task runner. It's required to build the project. To install it run:

```
cargo install cargo-make
```

## How to instantiate repository from the template?

See instructions [here](./doc/InstantiatingTemplate.md)

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
