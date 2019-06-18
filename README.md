# async_runtime

A lightweight runtime for global spawning of futures.

The purpose of async_runtime is to make it convenient to spawn and run futures. It allows library authors to call `rt::spawn( future );` rather than having to take a `T: Executor`, yet let client code decide what kind of executor is used. Currently the choice is between futures 0.3 `LocalPool` and the [juliex](https://github.com/withoutboats/juliex) threadpool. Other implementations might be added later.

Differences with the [runtime](https://github.com/rustasync/runtime) crate:

  - no need to box futures to spawn them, but you can spawn boxed futures just the same
  - client code can decide that the executor for the thread is a LocalPool (can be a serious performance benefit sometimes)
  - the executor is not a trait object, so you can't just implement a different one without patching this crate. I have not yet found the use for this, and tokio futures and streams run just fine with the compatibility layer from futures 0.3.

Both crates work on WASM.

When not on WASM, the default executor is the juliex threadpool. This is because the executor is set per thread and when tasks run on a threadpool thread and they spawn, they will automatically spawn on the threadpool. This alleviates the need for initialization code on the threadpool threads. This means that you have to call `rt::init` if you want the `LocalPool`.

On WASM, the default executor is also a threadpool, even though that's impossible. It means you always have to call `rt::init`. This might seem like an odd API design, but WASM will have threads in the future, so I prefered keeping the API consistent with other targets, and future proof.

