# async_runtime

[![standard-readme compliant](https://img.shields.io/badge/readme%20style-standard-brightgreen.svg?style=flat-square)](https://github.com/RichardLitt/standard-readme)
[![Build Status](https://api.travis-ci.org/najamelan/async_runtime.svg?branch=master)](https://travis-ci.org/najamelan/async_runtime)
[![Docs](https://docs.rs/async_runtime/badge.svg)](https://docs.rs/async_runtime)
![crates.io](https://img.shields.io/crates/v/async_runtime.svg)


> A lightweight runtime for global spawning of futures.

The purpose of `async_runtime` is to make it convenient to spawn and run futures.
It allows library authors to call [`rt::spawn( future );`](rt::spawn) rather than having to take a `T: Executor`,
yet let client code decide what kind of executor is used. Currently the choice is between
futures 0.3 `LocalPool` and the [juliex](https://github.com/withoutboats/juliex) threadpool.
Other implementations might be added later.

Differences with the [runtime](https://github.com/rustasync/runtime) crate:

  - no need to box futures to spawn them, but you can spawn boxed futures just the same
  - client code can decide that the executor for the thread is a LocalPool (can be a serious performance benefit sometimes)
  - the executor is not a trait object, so you can't just implement a different one without
  patching this crate. I have not yet found the use for this, and tokio futures and streams
  run just fine with the compatibility layer from futures 0.3. If the provided executors are
  not sufficient, please file an issue or a pull request.

Both crates work on WASM.

When not on WASM, the default executor is the juliex threadpool. This is because the executor is set
per thread and when tasks run on a threadpool thread and they spawn, they will automatically spawn
on the threadpool. This alleviates the need for initialization code on the threadpool threads.
This means that you have to call [`rt::init`] if you want the `LocalPool`.

On WASM, the default executor is also a threadpool, even though that's impossible. It means you always
have to call `rt::init`. This might seem like an odd API design, but WASM will have threads in the future,
so I prefered keeping the API future proof and consistent with other targets.


## Table of Contents

- [Install](#install)
  - [Dependencies](#dependencies)
- [Usage](#usage)
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

  async_runtime: ^0.1
```

With raw Cargo.toml
```toml
[dependencies]

   async_runtime = "^0.1"
```

### Dependencies

This crate only has one dependiency. Cargo will automatically handle it's dependencies for you.

```yaml
dependencies:

  futures-preview: { version: ^0.3.0-alpha.15 }
```

## Usage

Please have a look in the [examples directory of the repository](https://github.com/najamelan/async_runtime/tree/master/examples).

```rust
#![ feature( async_await ) ]

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

	// Please look at the documentatino for rt::spawn for the possible errors here.
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

			rt::spawn( fut ).expect( "spawn task" );
			tasks.push( handle );
		}

		join_all( tasks ).await;
	};

	futures::executor::block_on( program );
}
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

