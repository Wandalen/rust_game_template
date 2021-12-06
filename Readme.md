# Rust Game Template [![stability-experimental](https://img.shields.io/badge/stability-experimental-orange.svg)](https://github.com/emersion/stability-badges#experimental) [![ios](https://github.com/Wandalen/rust_game_template/actions/workflows/iOS.yml/badge.svg)](https://github.com/Wandalen/rust_game_template/actions/workflows/iOS.yml) [![android](https://github.com/Wandalen/rust_game_template/actions/workflows/Android.yml/badge.svg)](https://github.com/Wandalen/rust_game_template/actions/workflows/Android.yml)

Neutral cross-platform Rust game template.

## General prerequisites

```
cargo install cargo-make
```

## How to run

**Desktop:**

```
cargo make desktop_run
```

**Web:**

Install dev dependencies:
```
cargo make web_install_dependencies
```

Run web target:

```
cargo make web_run
```

To speedup incremental builds use:

```
cargo make web_rerun
```

This command doesn't perform crate installation checks to reduce total build time.

**Android:**

Prerequisites:

* [Android setup](https://github.com/dodorare/crossbow/wiki)
* NDK: 22.1.7171670
* API Level : 30.0.3

Connect your Android device or start an emulator and then execute:

```
cargo make run_android
```

**iOS:**

Create the file `private.toml` at the root of the repo and add next fields:

```toml
[ios]
development_team = "TEAM_ID"
```

Any field can also may contain environment variable as a value:

```toml
[ios]
development_team = "${TEAM_ID}"
```

In this case value will be taken from your environment variable during the build.

Then run:

```
cargo make run_ios
```

Generated Xcode project can be found at `module/renderer/platform/ios/xcode`.


## Commands

Avaiable commands:

```
Android
----------
build_android - Build android target. Rebuilds on change.
run_android - Run android target. Rebuilds on change.

Desktop
----------
desktop_build - Build desktop target.
desktop_run - Run desktop target
desktop_run_watching - Run desktop target. Rebuilds app on change

Prerequisites
----------
web_install_dependencies - Install web dependencies

Web
----------
web_build - Build web target. Rebuilds on change.
web_rebuild - Build web target. Rebuilds on change.
web_rerun - Run web target. Rebuilds app on change
web_run - Run web target. Rebuilds app on change

iOS
----------
build_ios - Build ios target.
clean_ios - Cleanup generated files of ios target.
run_ios - Run ios target. Rebuilds on change.
```

To execute the command use following syntax ```cargo make [command]```.

Execute `cargo make --list-all-steps` to get list of commands.


