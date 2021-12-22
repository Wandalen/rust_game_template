## How to generate a repository from a template

1. Install cargo-generate tool and generate a repository

```
cargo install cargo-generate
cargo generate --git https://github.com/Wandalen/rust_game_template.git --branch alpha
```

The utility will ask to enter the project name. Enter the desired name and hit enter. The name of the generated folder will be the same as the project name you provided.

2. Change your working directory to the generated project

3. Setup git branches

```
git add -A . && git commit -am init && git branch beta && git branch gamma && git branch master
```

4. Install build utility

```
cargo install cargo-make
```

5. Build the project

```
cargo make
```

6. See the [readme](./Readme.md) for futher instructions
