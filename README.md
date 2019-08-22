# async_runtime

[![standard-readme compliant](https://img.shields.io/badge/readme%20style-standard-brightgreen.svg?style=flat-square)](https://github.com/RichardLitt/standard-readme)
[![Build Status](https://api.travis-ci.org/najamelan/async_runtime.svg?branch=master)](https://travis-ci.org/najamelan/async_runtime)
[![Docs](https://docs.rs/naja_async_runtime/badge.svg)](https://docs.rs/naja_async_runtime)
[![crates.io](https://img.shields.io/crates/v/naja_async_runtime.svg)](https://crates.io/crates/naja_async_runtime)


> A lightweight runtime for global spawning of futures.

The purpose of `async_runtime` is to make it convenient to spawn and run futures.
It allows library authors to call [`rt::spawn( future );`](rt::spawn) rather than having to take a `T: Executor`,
yet let client code decide what kind of executor is used. Currently the choice is between
futures 0.3 `LocalPool` and the [juliex](https://github.com/withoutboats/juliex) threadpool.
Other implementations might be added later.

Some key features:

   - macro attributes to turn async fn in sync fn (can be used on main, tests or arbitrary async fn)
   - tries to do one thing and do it good (does not pull network/timer dependencies)
   - support a variety of executors, including single threaded ones that allow spawning `!Send` futures.
   - lightweight with very few dependencies
   - library authors can spawn, application authors can decide which executor is used on each thread
   - doesn't load network dependencies

When not on WASM, the default executor (when you don't choose one explicitly) is the LocalPool from the futures library. On WASM, the default executor is also a Bindgen, based on wasm-bindgen-futures.


## Table of Contents

- [Install](#install)
  - [Upgrade](#upgrade)
  - [Features](#features)
  - [Dependencies](#dependencies)
- [Usage](#usage)
  - [WASM](#wasm)
- [API](#api)
- [Contributing](#contributing)
  - [Code of Conduct](#code-of-conduct)
- [License](#license)


## Install
With [cargo add](https://github.com/killercup/cargo-edit):
`cargo add async_runtime`

With [cargo yaml](https://gitlab.com/storedbox/cargo-yaml):
```yaml
dependencies:

  async_runtime: { version: ^0.4, package: naja_async_runtime }
```

With raw Cargo.toml
```toml
[dependencies]

   async_runtime = { version = "^0.4", package = "naja_async_runtime" }
```


### Upgrade

Please check out the [changelog](https://github.com/najamelan/async_runtime/blob/master/CHANGELOG.md) when upgrading.


### Features

These features enable extra functionality:

   - `macros`: proc macro attributes to turn an async fn into a sync one. _On by default_.
   - `juliex`: the juliex executor.
   - `async_std`: the async-std executor.
   - `localpool`: the localpool. _Turned on by default on non WASM targets_.
   - `bindgen`: the wasm-bindgen backed executor. _Turned on by default on WASM targets_.

Various aspects of the library are only available if certain features are enabled. This will be noted in the documentation.

**Note** for library authors. You should not enable any features on `async_runtime`. The global executor is chosen by the application developer.


### Dependencies

This crate has few dependiencies. Cargo will automatically handle it's dependencies for you, except:

- `juliex` is optional. The feature is not additive. The default executor for each thread is the threadpool if `juliex` is turned on, but it is the localpool if it's not.

Other dependencies:

```yaml
failure         : ^0.1
futures-preview : { version: ^0.3.0-alpha, features: [ std, compat ], default-features: false }
log             : ^0.4
once_cell       : { version: ^0.2, default-features: false }
juliex          : { version: ^0.3.0-alpha, optional: true }
```

## Usage

### Available executors

__Warning:__ Some executors have specific modules (like `rt::async_std`) that make available functionality
specific to this particular executor. These exist for 2 reasons:
- The API of the different supported executors varies. It is not always possible to provide a unified API
  on top of them. To avoid losing functionality, we make it available in these modules.
- Sometimes providing a unified API imposes overhead like boxing a return type (`rt::spawn_handle`)
  or running initialization code for worker threads on threadpools that don't support setting up the
  threadpool with initialization code when it is created. (`rt::spawn` on async-std).

You should be careful using functionality from these modules. They work if you know what executor you
are using. You shouldn't use these in code that has to abstract out over executors, or that will call into
such code. Eg. if you are a library author or your futures will call into library code that uses async_runtime,
you generally shouldn't use these.

#### LocalPool

- feature: `localpool`, enabled by default on non WASM targets
- attribute: `#[ rt::localpool ]`
- config: `rt::Config::LocalPool`
- targets: not on WASM
- type: single threaded
- provider: [futures::executor::LocalPool](https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.18/futures/executor/struct.LocalPool.html)

The localpool executor, being a single threaded executor has a specific design you should be aware of. If it would
poll futures immediately after spawning, the thread would be occupied by this and your code that called spawn
would not return immediately. Therefor, there is a process of spawning first and then calling blocking methods
in order to run the executor.

Four methods are available on the executor to run it:

   - [`run`](https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.18/futures/executor/struct.LocalPool.html#method.run)
   - [`run_until`](https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.18/futures/executor/struct.LocalPool.html#method.run_until).
   - [`try_run_one`](https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.18/futures/executor/struct.LocalPool.html#method.try_run_one)
   - [`run_until_stalled`](https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.18/futures/executor/struct.LocalPool.html#method.run_until_stalled)

I'm not going to go into detail here about what these functions do, go check their documentation. Currently
async_runtime only exposes the `run` function from these. If you run into specific issues that require to
access one of the other three functions, please file an issue.


#### Bindgen

- feature: `bindgen`, enabled by default on WASM targets
- attribute: `#[ rt::bindgen ]`
- config: `rt::Config::Bindgen`
- targets: only on WASM
- type: single threaded
- provider: [wasm-bingen-futures](https://docs.rs/wasm-bindgen-futures)

Currently the only executor available on WASM. It functions like a multithreaded executor in that spawned
futures will start to be polled immediately and no `run` method must be called to start the executor.


#### Juliex

- feature: `juliex`
- attribute: `#[ rt::juliex ]`
- config: `rt::Config::Juliex`
- targets: not on WASM
- type: thread pool
- provider: juliex

A threadpool. Worker threads created will automatically have juliex set as the default executor. This
cannot be changed. Futures will be polled immediately. If you have a top level future that you block one,
or that is being waited on by the macro attribute, as soon as that future is done, the progam will end,
even if there are still tasks in the thread pool that haven't finished yet. You must add your own synchronization
like channels or [`join_all`](https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.18/futures/future/fn.join_all.html) from the futures library to wait on your tasks.


#### AsyncStd

- feature: `async_std`
- attribute: `#[ rt::async_std ]`
- config: `rt::Config::AsyncStd`
- targets: not on WASM
- type: thread pool
- provider: [async-std](https://crates.io/crates/async-std)

A threadpool. Worker threads created cannot have AsyncStd set automatically as the default executor. This means
that `rt::spawn` will have some overhead to make sure the worker thread is initialized properly.

Futures will be polled immediately. If you have a top level future that you block one, or that is being waited on
by the macro attribute, as soon as that future is done, the progam will end, even if there are still tasks
in the thread pool that haven't finished yet.

You can add your own synchronization like channels or [`join_all`](https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.18/futures/future/fn.join_all.html) from the futures library to wait on your tasks.

There is an `rt::async_std` module with specific functionality from this executor:

- [`spawn`](rt::async_std::spawn): Spawn directly on this executor (avoids the overhead from rt::spawn).
- [`spawn_handle`](rt::async_std::spawn_handle): Get a `async-std::task::TaskHandle` to await this future.
  This avoids the boxing that `rt::spawn_handle` has to do.

#### block_on

- feature: N/A
- attribute: N/A
- config: N/A
- targets: not on WASM
- type: blocks current thread
- provider: [`futures::executor::block_on`](https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.18/futures/executor/fn.block_on.html)

Please read the documentation of [rt::block_on].


### Examples

Please have a look in the [examples directory of the repository](https://github.com/najamelan/async_runtime/tree/master/examples).

```rust
use
{
   async_runtime :: { * } ,
};

// in library code:
//
fn do_something_in_parallel() -> Result<(), RtErr>
{
   rt::spawn( async
   {
      println!( "I'm running in async context" );
   })
}

// In client code we might decide that this runs in a LocalPool, instead of a threadpool:
//
fn main()
{
   // This only fails if you initialize twice. Therefor library code should not do this
   // unless the library is creating the threads.
   //
   rt::init( RtConfig::Local ).expect( "executor init" );

   // Please look at the documentation for rt::spawn for the possible errors here.
   //
   do_something_in_parallel().expect( "Spawn futures" );

   // On a threadpool, futures are polled immediately, but since here we only have one thread,
   // first we spawn our topmost tasks and then we have to tell the runtime that it's time to
   // start polling them. This will block the thread until all futures are finished.
   //
   rt::run();
}

```

```rust
// In this example we run a bunch of tasks in parallel. To verify that
// they run on different threads we make them all sleep for a second and
// measure the time passed when they finish.

#![ feature( async_await, duration_constants ) ]

use
{
   async_runtime :: { *                                          } ,
   std           :: { time::{ Duration, Instant }, thread::sleep } ,
   futures       :: { future::{ FutureExt, join_all }            } ,
};

fn main()
{
   let program = async
   {
      let start = Instant::now();
      let mut tasks = Vec::new();

      for i in 0..8
      {
         // There isn't currently a convenient way to run tasks on a threadpool
         // until all tasks have finished, or until some shutdown signal is given.
         //
         // This is one of the ways tasks can synchronize and wait on each other.
         // Another way is to wait on channels.
         //
         let (fut, handle) = async move
         {
            sleep( Duration::SECOND );

            println!
            (
               "Time elapsed at task {} end: {} second(s).",
               i, start.elapsed().as_secs()
            );

         }.remote_handle();


         // If the juliex feature is enabled, RtConfig::Pool becomes the default executor, so we don't
         // have to call rt::init.
         //
         rt::spawn( fut ).expect( "spawn task" );
         tasks.push( handle );
      }

      join_all( tasks ).await;
   };

   rt::block_on( program );
}
```

### Wasm

Note that it's best to turn of default-features in your Cargo.toml to avoid loading `juliex` which isn't used on wasm.
```toml
[dependencies]

   async_runtime = { version = "^0.1", default-features = false, package = "naja_async_runtime" }
```

To use the crate in wasm, please have a look at the example in the examples directory of the [repository](https://github.com/najamelan/async_runtime).

For the documentation, docs.rs does not make the wasm specific parts available, but their use is identical to the `rt` module for other targets. The only difference is that even though it's on a local pool (wasm does not have threads), you don't need to call run because the browser automatically runs the promises. This might change in the future.

**Note:** Wasm will panic on `thread_park`, which is used by `futures::executor::block_on`, so `rt::block_on` is not available on wasm.

For running the integration tests:
```bash
cargo install wasm-pack wasm-bindgen-cli
```
Now you can do either:
```bash
wasm-pack test --firefox --headless
```
or:
```bash
cargo test --target wasm32-unknown-unknown
```

## API

Api documentation can be found on [docs.rs](https://docs.rs/async_runtime).


## Contributing

This repository accepts contributions. Ideas, questions, feature requests and bug reports can be filed through github issues.

Pull Requests are welcome on github. By commiting pull requests, you accept that your code might be modified and reformatted to fit the project coding style or to improve the implementation. Please discuss what you want to see modified before filing a pull request if you don't want to be doing work that might be rejected.


### Code of conduct

Any of the behaviors described in [point 4 "Unacceptable Behavior" of the Citizens Code of Conduct](http://citizencodeofconduct.org/#unacceptable-behavior) are not welcome here and might get you banned. If anyone including maintainers and moderators of the project fail to respect these/your limits, you are entitled to call them out.

## License

[Unlicence](https://unlicense.org/)

