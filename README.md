# sea-of-stars

An auto splitter for Sea Of Stars


## Outstanding

- Sigscan for Assembly-CSharp.dll
- [ ] Splits on Character Select
- [ ] Load Removal
- [ ] ~ Split on last boss final hit
- [ ] ~ Story Flags


## Compilation

This auto splitter is written in Rust. In order to compile it, you need to
install the Rust compiler: [Install Rust](https://www.rust-lang.org/tools/install).

Afterwards install the WebAssembly target:
```sh
rustup target add wasm32-unknown-unknown --toolchain stable
```

The auto splitter can now be compiled:
```sh
cargo b
```

The auto splitter is then available at:
```
target/wasm32-unknown-unknown/release/sea_of_stars.wasm
```

Make sure too look into the [API documentation](https://livesplit.org/asr/asr/) for the `asr` crate.

You can use the [debugger](https://github.com/CryZe/asr-debugger) while
developing the auto splitter to more easily see the log messages, statistics,
dump memory and more.
