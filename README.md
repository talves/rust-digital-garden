Workspace for [Chris Biscardi's course on egghead.io][course]

# Rust Digital Garden

A CLI tool for the creation of our Digital Garden.

## Commands

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

## Changing the name of the binary

See the cargo.toml for the `bin` option.

[course]: https://egghead.io/courses/creating-a-digital-garden-cli-with-rust-34b8
