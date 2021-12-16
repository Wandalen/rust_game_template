## iOS

Prerequisites:

- [Rust](https://www.rust-lang.org/) v1.57 or newer.
- [cargo-make](https://github.com/sagiegurari/cargo-make)
  Install with command: ```cargo install cargo-make```
- [ios-deploy](https://github.com/ios-control/ios-deploy)
  Install with command: ```brew install ios-deploy```
- [xcodegen](https://github.com/yonaskolb/XcodeGen)
  Install with command: ```brew install xcodegen```
- [Xdode](https://apps.apple.com/us/app/xcode/id497799835)
- iOS Simulator

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
cargo make ios_run
```

Generated Xcode project can be found at `module/renderer/platform/ios/xcode`.
