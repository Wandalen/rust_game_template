# rust_game_template
Cross-platform rust game template 


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

```
cargo make run-web
```

**Android:**

Prerequisites:

* [Android setup](https://github.com/dodorare/crossbow/wiki)
* NDK: 22.1.7171670	
* API Level : 30.0.3

Connect your Android device or start an emulator and then execute:

```
cargo make run-android
```

