## iOS

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