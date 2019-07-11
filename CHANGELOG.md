# Changelog

## 0.1.7

- enable futures-preview to version 0.3.0-alpha.17. Until https://github.com/rustwasm/wasm-bindgen/issues/1640 get's fixed,
  this version won't work on wasm, however often cargo will require that different dependencies use the same version of
  futures-preview. If your other dependencies use alpha.17, you can use this version. If you need wasm to work, pin to 0.1.6:
  `async_runtime = { version = "=0.1.6", package = "naja_async_runtime" }`


## 0.1.6

- lock futures-preview to version 0.3.0-alpha.16 until https://github.com/rustwasm/wasm-bindgen/issues/1640 get's fixed.
