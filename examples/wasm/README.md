# Async Runtime Wasm Example

Demonstration of `async_runtime` working in WASM.

## Dependencies

```shell
rustup target add wasm32-unknown-unknown
cargo install wasm-pack
```

## Usage

```shell
git clone --recursive https://github.com/najamelan/async_runtime
cd async_runtime/examples/wasm
wasm-pack build --target web
```
If all goes well you should see the last line of the output as:
```
| :-) Your wasm pkg is ready to publish at ./pkg.
```

Now open the index.html file in your browser. If it says:
```
You succesfully spawned a future!
```

than it works!
