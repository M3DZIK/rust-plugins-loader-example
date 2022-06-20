# Example of dynamic plugins loading in Rust

An example of using and loading plugins from .so files.

![](https://i.imgur.com/NJrIkYS.png)

## 👨‍💻 Building

### Requirements

- [Rust](https://rust-lang.org)

To build, run the command: `cargo build --release --all`

The built plugins can be found in `target/release/lib{PLUGIN_NAME}.so`

## Run

To run this example, run the command: `cargo run -- ./target/debug/libplugin.so`
