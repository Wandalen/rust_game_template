# Rust Game Template
[![stability-experimental](https://img.shields.io/badge/stability-experimental-orange.svg)](https://github.com/emersion/stability-badges#experimental) [![ios](https://github.com/Wandalen/rust_game_template/actions/workflows/iOS.yml/badge.svg)](https://github.com/Wandalen/rust_game_template/actions/workflows/iOS.yml) [![android](https://github.com/Wandalen/rust_game_template/actions/workflows/Android.yml/badge.svg)](https://github.com/Wandalen/rust_game_template/actions/workflows/Android.yml) [![desktop](https://github.com/Wandalen/rust_game_template/actions/workflows/Desktop.yml/badge.svg)](https://github.com/Wandalen/rust_game_template/actions/workflows/Desktop.yml) [![web](https://github.com/Wandalen/rust_game_template/actions/workflows/Web.yml/badge.svg)](https://github.com/Wandalen/rust_game_template/actions/workflows/Web.yml)

Neutral cross-platform Rust game template.

## General prerequisites

```
cargo install cargo-make
```

## How to run on Desktop

```
cargo make desktop_run
```

## How to run on Web

Run web target:

```
cargo make web_run
```

To speedup incremental builds use:

```
cargo make web_rerun_watching
```

This command doesn't perform crate installation checks to reduce total build time.

## How to run on Android

Prerequisites:

* [Android setup](https://github.com/dodorare/crossbow/wiki)
* NDK: 22.1.7171670
* API Level : 30.0.3

Connect your Android device or start an emulator and then execute:

```
cargo make run_android
```

## How to run on iOS

Create the file `private.toml` at the root of the repo and add next fields:

```toml
[ios]
development_team = "PRIVATE_APPLE_TEAM_ID"
```

Any field can also may contain environment variable as a value:

```toml
[ios]
development_team = "${PRIVATE_APPLE_TEAM_ID}"
```

In this case value will be taken from your environment variable during the build.

Then run:

```
cargo make run_ios
```

Generated Xcode project can be found at `module/renderer/platform/ios/xcode`.

## Commands

To get list of commands use command `cargo make --list-all-steps`

```
Prerequisites
----------
web_install_dependencies - Install web dependencies

Android
----------
build_android - Build android target. Rebuilds on change.
run_android - Run android target. Rebuilds on change.

Desktop
----------
desktop_build - Build desktop target.
desktop_run - Run desktop target
desktop_run_watching - Run desktop target. Rebuilds app on change

Web
----------
web_build - Build web target. Rebuilds on change.
web_rebuild - Build web target. Rebuilds on change.
web_rerun - Run web target. Rebuilds app on change
web_run - Run web target. Rebuilds app on change

iOS
----------
build_ios - Build iOS target.
clean_ios - Cleanup generated files of iOS target.
run_ios - Run iOS target. Rebuilds on change.
```
