# Architecture

## Introduction

async_runtime aims to be a lightweight, low - preferably no - overhead wrapper around the available executors in the async ecosystem. For this, dependencies are kept as few as possible.

All executors are enabled with features.

There is a support crate naja_async_runtime_macros that enables proc macros on functions.

## Design

The idea is to provide a convenient API on the `rt` module. Methods that can be called on all executors live here.
I use an executor per thread, which is kept in a once_cell. On each thread where `rt::spawn` or similar methods are to be called, a call to `rt::init` must first be made to define which executor the user wants to use on this thread.

The actual executor object is an enum in `rt::executor`. It implements all the methods from rt for each executor.

Executor functionality that is to specific to be provided for all executors is provided in modules specific to the executor, like `rt::async_std`.

By using this design, we can avoid boxing futures and executors alike.

## Tests

There is a script `ci.bash` showing all of the tests I run!



## Problem domain

### Initial problem

Allow library code to spawn futures without:

1. bloating the dependency graph of client code with runtimes (reactors/network dependencies, ...)
2. deciding what executor implementation to use, and whether to spawn on a threadpool or on the current thread.
3. limiting the use of the library to multithreaded systems (notably, WASM is currently single threaded)
4. limiting the use of the library to `std` enabled environments (currently I don't think async await works at all on `no_std`)

It's worth quickly touching on why should you spawn futures? Futures can be awaited or returned to the client code. While that is true, it doesn't always work out:
- you might have to bridge not async API's to async. It might be synchronous API's, callback based API's (Web API in WASM), you might be on a single threaded environment, ... and sometimes you just need something to run concurrently and it might be part of an implementation detail and not your API.
- you might not be in async context because you are implementing poll functions of Sinks, Streams, AsyncRead, ... In this context you cannot await, and you cannot return a future because the signature of these traits is not defined as such.

### global spawn or `T: Executor`

Two possible designs come to mind when trying to solve the initial problem.
1. library code that needs to spawn must always take in `T: Executor`, but what is this Executor trait? It turns out the trait already exists, in a library almost all async code imports: `T: futures::task::Spawn`. Oh, awesome. But, none of the three main executor implementations we currently have: tokio, juliex and async-std implements this trait. Oops. Async-std doesn't even expose an executor object. We could wrap those executors and provide the trait, or executor implementers could become better behaved and implement the trait themselves.
2. We somehow make a global spawn function available everywhere. As an extra convenience, this does not clutter up the interfaces of your API. The downside is that it is not obvious that some piece of code spawns.

As long as executor implementers do not agree on all providing this trait, both ways require that both libraries and client code include an extra library to take care of this. I personally would not consider writing a library to implement a trait that would be better implemented by the executor implementers. I feel I already do enough of other peoples chores.

Both approaches do not need to be exclusive either. Even if all executors would implement `Spawn` and `SpawnLocal` where appropriate, we could still chose to provide API's that don't require passing around executors all over the place. So for now, I have chosen to go the path of globally available spawn.


### Storing the global executor

The idea is that any given code can call `rt::spawn` and just not worry about executor implementation. So we need to store our executor configuration somewhere. What possibilities do we have?

1. process global static variable
2. thread local variable

3. Scopes

   If we took code in a closure, we could do something like:

   ```rust
   rt::exec( rt::Config::SomeExecutor, |exec|
   {
   	// no need to use `exec` here, but if code being called would require you to pass in some T: Executor,
   	// this could work as long as we implement the correct trait.
   	//
   	rt::spawn( future );
   });
   ```

	This can work because we could keep a stack of executors, so it would even deal with nested calls to `exec`.

The three models have an increasing granular control. The third level really has an attractive property that people can specifically isolate external code to a given executor (some users have demanded that). It also has the property that executors have an explicit lifetime.

Note that threadpools create threads, which means that if we do any thread local bookkeeping, like in cases 2 and 3 above, we need to set it up for the worker threads. I assume that generally you want futures spawned from worker threads to go to the same executor. In practice, futures Threadpool, Juliex, tokio::Runtime all allow to run initializing code on the worker threads, but async-std does not. Current workaround for async-std is to check at each spawn whether the thread is initialized, and if not, initialize it. This creates overhead on each spawn.

### 2 user scenarios

1. App developers should be able to decide where futures get spawned (on what executor)
2. Lib developers should be able to spawn without making any such decision and without imposing heavy dependencies on client code.

Some of the API are user api (`spawn`, `spawn_local`, `spawn_handle`).
Some of the API are controller (`rt::init( rt::Config::Juliex );` would set up juliex as the executor for subsequent calls to `spawn`).


### Abstracting out over implementations

Not all executors we want to support have the same feature-set. There are some big categories:

1. Being able to spawn `!Send` futures, and generally not spawning any threads (not threadpools).
2. Allowing controlling the lifetime of the executor. Juliex and futures LocalPool
   allow us to take ownership of the executor object, whereas async_std just provides a global spawn function
   allowing strictly no control over any aspect of the executor.
3. Configuration. Some implementations allow you to configure a great deal of things, number of worker threads, panic handling, run code to initialize and on shutdown of the worker threads, ...

We shouldn't really lose executor specific functionality. Where libraries shouldn't have to care, if an application dev wants to leverage all of the extra functionality tokio offers, they should be able to do so. Obviously, we cannot

	We let users choose their executor by means of an enum. Like:
   ```rust
   rt::init( rt::Config::Juliex );
   ```
   Currently, the enum is data-less. This gives for a convenient API. It is `Copy`, easy to match on, serves as a way of letting the users check which executor is active... If we want to allow configuration, we need to take in options, which will be specific to a particular executor. Possibly we could make this enum contain configuration data.


### To fail or not to fail

Should our API be fallible or not.


### To trait or not to trait

It could be argued that the best way available in rust to express objects with varying behaviors are traits. Let's look at some of the relevant aspects of traits:
1. If you reduce an object to it's behavior, eg. you have `Box< dyn Trait >` or an `impl Trait`, you can at that moment no longer use any specific features not defined in the trait. A workaround is downcasting with `std::Any`. So if we store a `Box<Executor>` for each thread, only behaviors available on all implementers are still readily available.
2. Trait objects cannot have generics in their methods. This means that futures have to be boxed to be spawned. I would very much not like to go this path.
3. Static variables (eg. thread_local) cannot have generics.
3. Traits can be implemented by foreign code, so people could create compatibility for executors where we have not included them in the library.
4. The potential advantage here is that we could know who supports what in a general way. Eg. there could be seveal traits. Something that

Currently we rather use an enum. Since the list of executors we support is limited, this works pretty well. Supporting new executors means this library needs to be updated.
