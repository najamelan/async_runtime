//! # async_runtime
//!
//! [![standard-readme compliant](https://img.shields.io/badge/readme%20style-standard-brightgreen.svg?style=flat-square)](https://github.com/RichardLitt/standard-readme)
//! [![Build Status](https://api.travis-ci.org/najamelan/async_runtime.svg?branch=master)](https://travis-ci.org/najamelan/async_runtime)
//! [![Docs](https://docs.rs/naja_async_runtime/badge.svg)](https://docs.rs/async_runtime)
//! ![crates.io](https://img.shields.io/crates/v/naja_async_runtime.svg)
//!
//!
//! > A lightweight runtime for global spawning of futures.
//!
//! The purpose of `async_runtime` is to make it convenient to spawn and run futures.
//! It allows library authors to call [`rt::spawn( future );`](rt::spawn) rather than having to take a `T: Executor`,
//! yet let client code decide what kind of executor is used. Currently the choice is between
//! futures 0.3 `LocalPool` and the [juliex](https://github.com/withoutboats/juliex) threadpool.
//! Other implementations might be added later.
//!
//! Differences with the [runtime](https://github.com/rustasync/runtime) crate:
//!
//!   - no need to box futures to spawn them, but you can spawn boxed futures just the same
//!   - client code can decide that the executor for the thread is a LocalPool (can be a serious performance benefit sometimes)
//!   - the executor is not a trait object, so you can't just implement a different one without
//!   patching this crate. I have not yet found the use for this, and tokio futures and streams
//!   run just fine with the compatibility layer from futures 0.3. If the provided executors are
//!   not sufficient, please file an issue or a pull request.
//!
//! Both crates work on WASM.
//!
//! When not on WASM, the default executor is the juliex threadpool (unless you use `default-features = false`).
//! This is because the executor is set per thread and when tasks run on a threadpool thread and they spawn,
//! they will automatically spawn on the threadpool. This alleviates the need for initialization code on the threadpool
//! threads. This means that you have to call [`rt::init`] if you want the `LocalPool` (or disable the default features).
//!
//! On WASM, the default executor is also a threadpool, even though that's impossible (wasm does not have threads right now).
//! It's recommended to use `default-features = false` on wasm to disable the dependency on juliex.
//! This will change the default executor to be the local pool. This might seem like an odd API design,
//! but WASM will have threads in the future, so I prefered keeping the API future proof and consistent with other targets.
//! Another consistency choice is that `spawn` and `spawn_local` return Result, even though currently on wasm
//! they cannot fail.
//!
//! There isn't currently a separate api documentation for WASM and docs.rs will not show modules included only
//! when the target is WASM. However, the use of the library is identical, so I have chosen not to set up a separate
//! documentation. You can check the wasm example in the
//! [examples directory of the repository](https://github.com/najamelan/async_runtime/tree/master/examples), as
//! well as the integration tests. You can also clone the repository and run:
//! `cargo doc --open --target wasm32-unknown-unknown` or read the source code.
//!
//!
//! ## Table of Contents
//!
//! - [Install](#install)
//!   - [Features](#features)
//!   - [Dependencies](#dependencies)
//! - [Usage](#usage)
//!   - [WASM](#wasm)
//! - [API](#api)
//! - [Contributing](#contributing)
//!   - [Code of Conduct](#code-of-conduct)
//! - [License](#license)
//!
//!
//! ## Install
//! With [cargo add](https://github.com/killercup/cargo-edit):
//! `cargo add async_runtime`
//!
//! With [cargo yaml](https://gitlab.com/storedbox/cargo-yaml):
//! ```yaml
//! dependencies:
//!
//!     async_runtime: { version: ^0.1, package: naja_async_runtime }
//! ```
//!
//! With raw Cargo.toml
//! ```toml
//! [dependencies]
//!
//!    async_runtime = { version = "^0.1", package = "naja_async_runtime" }
//! ```
//!
//! ### Features
//!
//! There is one feature: `juliex`. It's on by default and you can turn it off if you only want the localpool. On wasm, turn it off as it's not being used. See the [Dependencies section](#dependencies).
//!
//! ### Dependencies
//!
//! This crate has few dependiencies. Cargo will automatically handle it's dependencies for you, except:
//!
//! - `juliex` is optional. Add the dependency with `default-features = false` to disable. On wasm you should also
//!   do this as it won't be used:
//!
//!   ```toml
//!   [dependencies]
//!
//!      async_runtime = { version = "^0.1", default-features = false, package = "naja_async_runtime" }
//!   ```
//!
//! Other dependencies:
//!
//! ```yaml
//! failure         : ^0.1
//! futures-preview : { version: ^0.3.0-alpha.16, features: [ std, compat, nightly ], default-features: false }
//! log             : ^0.4
//! once_cell       : ^0.1
//! juliex          : { version: 0.3.0-alpha.6, optional: true }
//! ```
//!
//! ## Usage
//!
//! Please have a look in the [examples directory of the repository](https://github.com/najamelan/async_runtime/tree/master/examples).
//!
//! ```rust
//! #![ feature( async_await ) ]
//!
//! use
//! {
//! 	async_runtime :: { * } ,
//! };
//!
//! // in library code:
//! //
//! fn do_something_in_parallel() -> Result<(), RtErr>
//! {
//! 	rt::spawn( async
//! 	{
//! 		println!( "I'm running in async context" );
//! 	})
//! }
//!
//! // In client code we might decide that this runs in a LocalPool, instead of a threadpool:
//! //
//! fn main()
//! {
//! 	// This only fails if you initialize twice. Therefor library code should not do this
//! 	// unless the library is creating the threads.
//! 	//
//! 	rt::init( RtConfig::Local ).expect( "executor init" );
//!
//! 	// Please look at the documentation for rt::spawn for the possible errors here.
//! 	//
//! 	do_something_in_parallel().expect( "Spawn futures" );
//!
//! 	// On a threadpool, futures are polled immediately, but since here we only have one thread,
//! 	// first we spawn our topmost tasks and then we have to tell the runtime that it's time to
//! 	// start polling them. This will block the thread until all futures are finished.
//! 	//
//! 	rt::run();
//! }
//!
//! ```
//!
//! ```rust
//! // In this example we run a bunch of tasks in parallel. To verify that
//! // they run on different threads we make them all sleep for a second and
//! // measure the time passed when they finish.
//!
//! #![ feature( async_await, duration_constants ) ]
//!
//! use
//! {
//! 	async_runtime :: { *                                          } ,
//! 	std           :: { time::{ Duration, Instant }, thread::sleep } ,
//! 	futures       :: { future::{ FutureExt, join_all }            } ,
//! };
//!
//! fn main()
//! {
//! 	let program = async
//! 	{
//! 		let start = Instant::now();
//! 		let mut tasks = Vec::new();
//!
//! 		for i in 0..8
//! 		{
//! 			// There isn't currently a convenient way to run tasks on a threadpool
//! 			// until all tasks have finished, or until some shutdown signal is given.
//! 			//
//! 			// This is one of the ways tasks can synchronize and wait on each other.
//! 			// Another way is to wait on channels.
//! 			//
//! 			let (fut, handle) = async move
//! 			{
//! 				sleep( Duration::SECOND );
//!
//! 				println!
//! 				(
//! 					"Time elapsed at task {} end: {} second(s).",
//! 					i, start.elapsed().as_secs()
//! 				);
//!
//! 			}.remote_handle();
//!
//!
//! 			// If the juliex feature is enabled, RtConfig::Pool becomes the default executor, so we don't
//! 			// have to call rt::init.
//! 			//
//! 			rt::spawn( fut ).expect( "spawn task" );
//! 			tasks.push( handle );
//! 		}
//!
//! 		join_all( tasks ).await;
//! 	};
//!
//! 	futures::executor::block_on( program );
//! }
//! ```
//!
//! ### Wasm
//!
//! Note that it's best to turn of default-features in your Cargo.toml to avoid loading `juliex`
//!  which isn't used on wasm.
//! ```toml
//! [dependencies]
//!
//!    async_runtime = { version = "^0.1", default-features = false }
//! ```
//!
//! To use the crate in wasm, please have a look at the example in the examples directory of the
//! [repository](https://github.com/najamelan/async_runtime).
//!
//! For the documentation, docs.rs does not make the wasm specific parts available, but their use
//! is identical to the `rt` module for other targets. The only difference is that even though it's
//! on a local pool (wasm does not have threads), you don't need to call run because the browser
//! automatically runs the promises. This might change in the future.
//!
//! For running the integration tests:
//! ```bash
//! cargo install wasm-pack wasm-bindgen-cli
//! ```
//! Now you can do either:
//! ```bash
//! wasm-pack test --firefox --headless
//! ```
//! or:
//! ```bash
//! cargo test --target wasm32-unknown-unknown
//! ```
//!
//! ## API
//!
//! Api documentation can be found on [docs.rs](https://docs.rs/async_runtime).
//!
//!
//! ## Contributing
//!
//! This repository accepts contributions. Ideas, questions, feature requests and bug reports can be filed through github issues.
//!
//! Pull Requests are welcome on github. By commiting pull requests, you accept that your code might be modified and reformatted to fit the project coding style or to improve the implementation. Please discuss what you want to see modified before filing a pull request if you don't want to be doing work that might be rejected.
//!
//!
//! ### Code of conduct
//!
//! Any of the behaviors described in [point 4 "Unacceptable Behavior" of the Citizens Code of Conduct](http://citizencodeofconduct.org/#unacceptable-behavior) are not welcome here and might get you banned. If anyone including maintainers and moderators of the project fail to respect these/your limits, you are entitled to call them out.
//!
//! ## License
//!
//! [Unlicence](https://unlicense.org/)
//
#![ feature( async_await ) ]

#![forbid( unsafe_code ) ]

#![ warn
(
	missing_debug_implementations ,
	missing_docs                  ,
	nonstandard_style             ,
	rust_2018_idioms              ,
)]

#![allow( clippy::suspicious_else_formatting ) ]


#[ cfg(not( target_arch = "wasm32" )) ] pub mod rt                                       ;
#[ cfg(not( target_arch = "wasm32" )) ] pub use { rt::exec03::* }                        ;

#[ cfg(     target_arch = "wasm32" )  ] pub mod wasm_rt                                  ;
#[ cfg(     target_arch = "wasm32" )  ] pub use { wasm_rt::wasm_exec::*, wasm_rt as rt } ;


mod error;
mod rt_config;

pub use
{
	error     :: * ,
	rt_config :: * ,
};


mod import
{
	pub use
	{
		once_cell :: { unsync::OnceCell, unsync::Lazy, unsync_lazy          } ,
		failure   :: { Backtrace, Fail, Context as FailContext              } ,
		std       :: { fmt, future::Future, rc::Rc, cell::RefCell, pin::Pin } ,

		futures ::
		{
			prelude :: { Stream, StreamExt, Sink, SinkExt                                         } ,
			channel :: { oneshot, mpsc                                                            } ,
			future  :: { FutureExt, TryFutureExt                                                  } ,
			task    :: { Spawn, SpawnExt, LocalSpawn, LocalSpawnExt, Context as TaskContext, Poll } ,

			executor::
			{
				LocalPool    as LocalPool03    ,
				LocalSpawner as LocalSpawner03 ,
				ThreadPool   as ThreadPool03   ,
			},
		},
	};
}
