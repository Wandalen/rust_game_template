# Rust Game Template [![stability-experimental](https://img.shields.io/badge/stability-experimental-orange.svg)](https://github.com/emersion/stability-badges#experimental) [![ios](https://github.com/Wandalen/rust_game_template/actions/workflows/iOS.yml/badge.svg)](https://github.com/Wandalen/rust_game_template/actions/workflows/iOS.yml) [![android](https://github.com/Wandalen/rust_game_template/actions/workflows/Android.yml/badge.svg)](https://github.com/Wandalen/rust_game_template/actions/workflows/Android.yml)

Neutral cross-platform Rust game template.

## General prerequisites

```
cargo install cargo-make
```

## How to run

**Desktop:**

```
cargo make run
```

**Web:**

Install dev dependencies:
```
cargo make install-web-dependencies
```

Run web target:

```
cargo make run-web
```

To speedup incremental builds use:

```
cargo make run-web-fast
```

This command doesn't perform crate installation checks to reduce total build time.

**Android:**

Prerequisites:

* [Android setup](https://github.com/dodorare/crossbow/wiki)
* NDK: 22.1.7171670
* API Level : 30.0.3

Connect your Android device or start an emulator and then execute:

```
cargo make run-android
```

**iOS:**

Create the file `module/renderer/config/private.toml` and add next fields:

```toml
[ios]
development_team = "PRIVATE_TEAM_ID"
```

Any field can also may contain environment variable as a value:

```toml
[ios]
development_team = "${PRIVATE_TEAM_ID}"
```

In this case value will be taken from your environment variable during the build.

Then run:

```
cargo make run-ios
```

Generated Xcode project can be found at `module/renderer/platform/ios/xcode`.


## Commands

Avaiable commands:

```
Build
----------
build - Build desktop target. Rebuilds on change
build-android - Build android target. Rebuilds on change
build-web - Build web target. Rebuilds on change

Run
----------
run - Run desktop target. Rebuilds app on change
run-android - Run android target. Rebuilds on change
run-web - Run web target. Rebuilds app on change
```

To execute the command use following syntax ```cargo make [command]```.

Execute `cargo make --list-all-steps` to get list of commands.
