# Changelog

## 0.4 - 2019-08-??

This is a major update with new features and breaking changes.

- I ditched a bunch of dependencies. Minimally we just depend on futures-preview and once_cell.
  On wasm also wasm-bindgen-futures.
  Opt into extra functionality with 2 more features, `macros`, to enable attributes and `juliex` to enable
  the juliex threadpool.

- I dropped failure, so we just have a custom error type that implements `std::error::Error`. If you need
  context information for errors, I suggest having a look at [`error-context`](https://docs.rs/error-context/0.1.1/error_context/). I haven't used it yet, but it looks good and has no dependencies.

- Because features weren't really additive before, the default executors when none are set by the user are now
  `LocalPool` if not on WASM and `Bindgen` on wasm. Juliex always needs to be enabled with `rt::init` or by a
  macro attribute.

- Macro attributes! Tag your async functions to turn them into sync functions, and set the default executor
  for the thread in the process. It works for `main`, `#[test]`, `#[wasm_bindgen_test]` or arbitrary `async fn`.
  It doesn't work for `#[bench]`. Please see the readme for more details.

- Overhaul of the architecture, making it better fit to accomodate more executors in the future.

- Revamp tests, examples and documentation.

- `#![ feature( async_await ) ]` is gone! So as soon as 1.39 comes out, this crate should work on stable rust.

## 0.3.4 - 2019-10-11

- another try at fixing docs.rs

## 0.3.3 - 2019-10-10

- another try at fixing docs.rs

## 0.3.2 - 2019-10-08

- fix docs.rs

## 0.3.1 - 2019-10-08

- update dependencies

## 0.3.0 - 2019-08-02

- actually, `block_on` will panic on wasm (uses `thread::park` internally), so it's removed again until wasm get's threads


## 0.2.1 - 2019-07-19

- add forgotten `rt::block_on` to the wasm runtime


## 0.2.0 - 2019-07-15

- BREAKING CHANGE: set the lib name to async_runtime


## 0.1.7 - 2019-07-11

- enable futures-preview to version 0.3.0-alpha.17. Until https://github.com/rustwasm/wasm-bindgen/issues/1640 get's fixed,
  this version won't work on wasm, however often cargo will require that different dependencies use the same version of
  futures-preview. If your other dependencies use alpha.17, you can use this version. If you need wasm to work, pin to 0.1.6:
  `async_runtime = { version = "=0.1.6", package = "naja_async_runtime" }`


## 0.1.6 - 2019-07-11

- lock futures-preview to version 0.3.0-alpha.16 until https://github.com/rustwasm/wasm-bindgen/issues/1640 get's fixed.
