## Android

Prerequisites:

- [Rust](https://www.rust-lang.org/) v1.57 or newer.
- [cargo-make](https://github.com/sagiegurari/cargo-make)
  Install with command: ```cargo install cargo-make```
- [Android setup](https://github.com/dodorare/crossbow/wiki)
  * NDK: 22.1.7171670
  * API Level : 30.0.3

Connect your Android device or start an emulator and then execute:

```
cargo make run_android
```

To get list of all command related this target run:

```
cargo make --list-all-steps
```
