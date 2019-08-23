# async_runtime

[![standard-readme compliant](https://img.shields.io/badge/readme%20style-standard-brightgreen.svg?style=flat-square)](https://github.com/RichardLitt/standard-readme)
[![Build Status](https://api.travis-ci.org/najamelan/async_runtime.svg?branch=master)](https://travis-ci.org/najamelan/async_runtime)
[![Docs](https://docs.rs/naja_async_runtime/badge.svg)](https://docs.rs/naja_async_runtime)
[![crates.io](https://img.shields.io/crates/v/naja_async_runtime.svg)](https://crates.io/crates/naja_async_runtime)


> A lightweight runtime for global spawning of futures.

The purpose of `async_runtime` is to make it convenient to spawn and run futures.
It allows library authors to call [`rt::spawn( future );`](spawn) rather than having to take a `T: Executor`,
yet let client code decide what kind of executor is used. It avoids pulling in the entire network stack/reactor of
several runtime crates like runtime, tokio, async-std just because you want to use a few libraries that provide
an async interface and they all depend on their favorite runtime. `async_runtime` tries to keep it fast and light.

Some key features:

   - macro attributes to turn async fn in sync fn (can be used on main, tests or arbitrary async fn)
   - tries to do one thing and do it good (does not pull network/timer dependencies)
   - support a variety of executors, including single threaded ones that allow spawning `!Send` futures.
   - lightweight with very few dependencies
   - library authors can spawn, application authors can decide which executor is used on each thread.
   - wasm support (this means that a library which compiles on wasm does not need any specific code, just use `rt::spawn` just the same)


## Table of Contents

- [Install](#install)
  - [Upgrade](#upgrade)
  - [Features](#features)
  - [Dependencies](#dependencies)
- [Usage](#usage)
  - [Available executors](#available-executors)
     - [LocalPool](#localpool)
     - [Bindgen](#bindgen)
     - [Juliex](#juliex)
     - [AsyncStd](#asyncstd)
     - [block_on](#block_on)
  - [Examples](#examples)
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

Minimum required rustc version: 1.39.

### Upgrade

Please check out the [changelog](https://github.com/najamelan/async_runtime/blob/master/CHANGELOG.md) when upgrading.


### Features

These features enable extra functionality:

   - `macros`: proc macro attributes to turn an async fn into a sync one.
   - `juliex`: the juliex executor.
   - `async_std`: the async-std executor.
   - `localpool`: the localpool.
   - `bindgen`: the wasm-bindgen backed executor.

**Note** for library authors. You should not enable any features on `async_runtime`. The per thread executor is chosen by the application developer (exception: your library is creating the threads).


### Dependencies

This crate has few dependiencies. Cargo will automatically handle it's dependencies for you, except that
when you are an application developer, you must enable at least one executor and you might want to enable
the `macros` feature.

## Usage

The basic concept is this. When you create a thread, you call [`init`] to decide which executor will be used
for calls to `spawn` and friends in the thread. `async_runtime` makes sure that worker threads in threadpools
are set up to continue spawning on the same threadpool. All top level functions in this library will work correctly
regardless of the chosen executor. One exception are functions ending in `_local`. Those are not available on threadpools
and will return an error.

If [`spawn`]* gets called on a thread for which no executor has been chosen, an error is returned.

### Available executors

__Warning:__ Some executors have specific modules (like `rt::localpool`) which make available functionality
specific to this particular executor. These exist for 2 reasons:
1. The API of the different supported executors varies. It is not always possible to provide a unified API
   on top of them. To avoid losing functionality, we make it available in these modules.
2. Sometimes providing a unified API imposes overhead like boxing a return type (`rt::spawn_handle`).
   Since async-std provides a `JoinHandle`, there is [`async_std::spawn_handle`] to recover that instead
   of a boxed variant. On other executors you can use `remote_handle` from the futures library to avoid boxing.

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

- feature: `Bindgen`, enabled by default on WASM targets
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

A threadpool. Worker threads created will automatically have juliex set as the thread executor. This
cannot be changed through the API `async_runtime` exposes right now. Futures will be polled immediately.

If you have a top level future that you block on, or that is being waited on by the macro attribute,
as soon as that future is done, the progam will end, even if there are still tasks in the thread pool
that haven't finished yet.

`async_runtime` provides the [`spawn_handle`] method to wait on your futures, but
that requires boxing the returned handle. Otherwise you can add your own synchronization like channels or
[`join_all`](https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.18/futures/future/fn.join_all.html)
from the futures library to wait on your tasks. The futures library also provides `remote_handle`.


#### AsyncStd

- feature: `async_std`
- attribute: `#[ rt::async_std ]`
- config: `rt::Config::AsyncStd`
- targets: not on WASM
- type: thread pool
- provider: [async-std](https://crates.io/crates/async-std)

A threadpool. Worker threads created cannot have AsyncStd set automatically as the default executor. This means
that `rt::spawn` will have some overhead to make sure the worker thread is initialized properly.

__Warning__: async-std does not have optional dependencies, so you will be pulling in all their dependencies,
including network libraries, mio, ... which will cause bloat if you don't use them.

Futures will be polled immediately. If you have a top level future that you block on, or that is being waited on
by the macro attribute, as soon as that future is done, the progam will end, even if there are still tasks
in the thread pool that haven't finished yet.

`async_runtime` provides the [`spawn_handle`] method to wait on your futures, but
that requires boxing the returned handle. Otherwise you can add your own synchronization like channels or
[`join_all`](https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.18/futures/future/fn.join_all.html)
from the futures library to wait on your tasks. The futures library also provides `remote_handle`.


#### block_on

- feature: no feature, always available
- attribute: N/A
- config: N/A
- targets: not on WASM
- type: blocks current thread
- provider: [`futures::executor::block_on`](https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.18/futures/executor/fn.block_on.html)

Please read the documentation of [block_on].


### Examples

Please have a look in the [examples directory of the repository](https://github.com/najamelan/async_runtime/tree/master/examples).

```rust
use async_runtime as rt;

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
[ rt::localpool ]
//
fn main()
{
   // Please look at the documentation for spawn for the possible errors here.
   //
   do_something_in_parallel().expect( "Spawn futures" );
}

```

```rust
// In this example we run a bunch of tasks in parallel. To verify that
// they run on different threads we make them all sleep for a second and
// measure the time passed when they finish. If they run on a threadpool
// time passed for all of them should be around 1 second.

#![ feature( duration_constants ) ]

use
{
   async_runtime as rt,

   std           :: { time::{ Duration, Instant }, thread::sleep } ,
   futures       :: { future::{ FutureExt, join_all }            } ,
};


#[ rt::juliex ]
//
async fn main()
{
   let     start = Instant::now();
   let mut tasks = Vec::new();

   for i in 0..4
   {
      // There isn't currently a convenient way to run tasks on a threadpool until all tasks have
      // finished, or until some shutdown signal is given.
      //
      // This is one of the ways tasks can synchronize and wait on eachother. Another way is to wait
      // on channels.
      //
      let (fut, handle) = async move
      {
         sleep( Duration::SECOND );

         println!( "Time elapsed at task {} end: {} second(s).", i, start.elapsed().as_secs() );

      }.remote_handle();

      rt::spawn( fut ).expect( "spawn task" );
      tasks.push( handle );
   }

   join_all( tasks ).await;
}
```

### Wasm

To use the crate in wasm, please have a look at the example in the examples directory of the [repository](https://github.com/najamelan/async_runtime/tree/master/examples).

The only executor available on WASM is currently _bindgen_.


## API

Api documentation can be found on [docs.rs](https://docs.rs/async_runtime).


## Contributing

This repository accepts contributions. Ideas, questions, feature requests and bug reports can be filed through github issues.

Pull Requests are welcome on github. By commiting pull requests, you accept that your code might be modified and reformatted to fit the project coding style or to improve the implementation. Please discuss what you want to see modified before filing a pull request if you don't want to be doing work that might be rejected.

Please file pull requests against the dev branch.


### Code of conduct

Any of the behaviors described in [point 4 "Unacceptable Behavior" of the Citizens Code of Conduct](http://citizencodeofconduct.org/#unacceptable-behavior) are not welcome here and might get you banned. If anyone including maintainers and moderators of the project fail to respect these/your limits, you are entitled to call them out.

## License

[Unlicence](https://unlicense.org/)

