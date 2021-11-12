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


