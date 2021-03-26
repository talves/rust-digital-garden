Workspace for [Chris Biscardi's course on egghead.io][course]

# Rust Digital Garden

A CLI tool for the creation of our Digital Garden.

## Commands

### Setting the garden path

```shell
GARDEN_PATH=~/git/Rust/rust-digital-garden garden write
garden -p ~/git/Rust/rust-digital-garden write
garden --garden_path ~/git/Rust/rust-digital-garden write
```

### write

Open a new file to write in our digital garden. Since we don't necessarily know what we want to title what we're writing, we'll leave the title as aptional and ask the usre for it later if they don't provide it first.

```shell
garden write
garden write -t "Some Title"
```

## Setting up the Cargo binary crate

```shell
cargo init
cargo run
cargo install cargo-edit # installs the cargo extended commands to allow for cargo sub-commands (add, rm, upgrade, etc.)
cargo add cargo-eyre # adds a crate for better error messages
```

We can run our output and see the result after the structop addition.

```shell
cargo run -- --help
# or after
./target/debug/garden --help
```

## Testing (integration)

```shell
cargo add assert_cmd --dev
cargo test
```

An integration test setup not added to our crate, but used during development (--dev)

## Adding cargo watch

```shell
cargo install cargo-watch
cargo watch -x test
cargo watch -x check -x test
```

## Changing the name of the binary

See the cargo.toml for the `bin` option.

[course]: https://egghead.io/courses/creating-a-digital-garden-cli-with-rust-34b8
