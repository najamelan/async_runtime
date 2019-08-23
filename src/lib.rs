// See: https://github.com/rust-lang/rust/issues/44732#issuecomment-488766871
//!
#![ cfg_attr( feature = "external_doc", feature(external_doc)         ) ]
#![ cfg_attr( feature = "external_doc", doc(include = "../README.md") ) ]




#![ doc    ( html_root_url = "https://docs.rs/naja_async_runtime" ) ]
#![ deny   ( missing_docs                                         ) ]
#![ forbid ( unsafe_code                                          ) ]
#![ allow  ( clippy::suspicious_else_formatting                   ) ]

#![ warn
(
	missing_debug_implementations ,
	missing_docs                  ,
	nonstandard_style             ,
	rust_2018_idioms              ,
	trivial_casts                 ,
	trivial_numeric_casts         ,
	unused_extern_crates          ,
	unused_qualifications         ,
	single_use_lifetimes          ,
	unreachable_pub               ,
	variant_size_differences      ,
)]


mod import
{
	pub(crate) use
	{
		once_cell :: { unsync::OnceCell                                   } ,
		std       :: { cfg, fmt, future::Future, error::Error as StdError } ,
	};


	#[ cfg(any( feature = "bindgen", feature = "localpool", feature = "juliex" )) ]
	//
	pub(crate) use	futures::future::FutureExt;


	#[ cfg( feature = "bindgen" ) ]
	//
	pub(crate) use
	{
		wasm_bindgen_futures :: { futures_0_3::spawn_local } ,
	};


	#[ cfg( feature = "localpool" ) ]
	//
	pub(crate) use
	{
		std     :: { cell::RefCell                                                              } ,
		futures :: { task::LocalSpawnExt, executor::{ LocalPool as FutLocalPool, LocalSpawner } } ,
	};


	#[ cfg( feature = "juliex" ) ]
	//
	pub(crate) use
	{
		once_cell :: { sync::OnceCell as SyncOnceCell } ,
	};
}


/////////////////
// --- API --- //
/////////////////

mod error    ;
mod config   ;
mod executor ;

pub use error::*;
pub use config::*;


#[ cfg( feature = "localpool" ) ] pub use executor::localpool ;
#[ cfg( feature = "async_std" ) ] pub use executor::async_std ;


#[ cfg(all( feature = "macros", feature = "juliex"    )) ] pub use naja_runtime_macros::juliex    ;
#[ cfg(all( feature = "macros", feature = "async_std" )) ] pub use naja_runtime_macros::async_std ;
#[ cfg(all( feature = "macros", feature = "localpool" )) ] pub use naja_runtime_macros::localpool ;
#[ cfg(all( feature = "macros", feature = "bindgen"   )) ] pub use naja_runtime_macros::bindgen   ;


use
{
	import   :: { *        } ,
	executor :: { Executor } ,
};


std::thread_local!
(
	pub(crate) static EXEC: OnceCell<Executor> = OnceCell::new();
);



/// Set the executor to use by on this thread. Run this before calls to [`spawn`]\(_*\).
///
/// If you are a library author, don't call this unless you create the thread, otherwise it's up to client code to
/// decide which executor to use. Just call [`spawn`].
///
/// ### Errors
///
/// This method will fail with [`ErrorKind::DoubleExecutorInit`] if you call it twice on the same thread. There is
/// [`init_allow_same`] which will not return an error if you try to init with the same executor twice.
///
/// ### Example
#[cfg_attr(feature = "localpool", doc = r##"
```rust
use async_runtime as rt;

// Will be used for spawning on the current thread only, if you create other threads,
// you have to init them too, and you can choose a different executor on different threads.
//
rt::init( rt::Config::LocalPool ).expect( "Set thread executor" );

rt::spawn( async {} ).expect( "spawn future" );

rt::localpool::run();
```
"##)]
//
pub fn init( config: Config ) -> Result< (), Error >
{
	EXEC.with( |exec| -> Result< (), Error >
	{
		exec.set( Executor::new( config ) ).map_err( |_| ErrorKind::DoubleExecutorInit.into() )
	})
}


/// Set the executor to use for this thread. The difference with [`init`] is that this will not return
/// a [`ErrorKind::DoubleExecutorInit`] error if you init with the same executor twice. It will still err
/// if you try to set 2 different executors for this thread.
///
/// This can sometimes be convenient for example if you would like to make two async fn sync in the
/// same thread with macro attributes (they use this method). You should rarely need this.
//
pub fn init_allow_same( config: Config ) -> Result< (), Error >
{
	if let Some(cfg) = current_rt() {
	if config == cfg
	{
		return Ok(())
	}}

	init( config )
}


/// Spawn a future to be run on the thread specified executor (set with [`init`]).
///
/// This method returns a result. I understand that this is an inconveniece, but this is a interface that
/// abstracts out over all supported executors. Some of them don't have an infallible spawn method, so we return
/// a result even though on most executors spawning is infallible.
///
/// Most of the time failing to spawn is rather fatal, so often using "expect" is fine. In
/// application code you will know which executor you use, so you know if it's fallible, but library authors
/// need to take into account that this might be fallible and consider how to recover from it.
///
/// [`spawn`] requires a `Send` bound on the future. See [`spawn_local`] if you have to spasn `!Send` futures.
///
/// [`spawn`] requires a `()` Output on the future. If you need to wait for the future to finish and/or recover
/// a result from the computation, see: [`spawn_handle`].
///
/// ### Errors
///
/// - This method is infallible on: _juliex_, _async-std_, _bindgen_.
/// - On the _localpool_ executor, this method can return a [`ErrorKind::Spawn`] if the executor has been shut down.
///   See the [docs for the futures library](https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.18/futures/task/struct.SpawnError.html). I haven't really found a way to trigger this error.
///   You can call [localpool::run] and spawn again afterwards.
/// - If you call this without an initialized executor, [`ErrorKind::NoExecutorInitialized`] is returned.
///
/// ### Example
#[ cfg_attr( feature = "localpool", doc = r##"
```
use async_runtime as rt;

rt::init( rt::Config::LocalPool ).expect( "no double executor init" );

rt::spawn( async
{
   println!( "async execution" );

}).expect( "spawn on localpool" );
```
"##)]
//
pub fn spawn( fut: impl Future< Output=() > + 'static + Send ) -> Result< (), Error >
{
	EXEC.with( move |exec| -> Result< (), Error >
	{
		match exec.get()
		{
			Some(e) => e.spawn( fut )                                 ,
			None    => Err( ErrorKind::NoExecutorInitialized.into() ) ,
		}
	})
}


/// Spawn a future to be run on the current thread. This will return an error if the current executor is a threadpool.
/// Currently works with _bindgen_ and _localpool_.
///
/// Does exactly the same as [spawn], but does not require the future to be [Send]. If your
/// future is [Send], you can just use [spawn]. It will spawn on the executor the current thread is configured with
/// either way..
///
/// __Warning__: If you are a library author and you use this, you oblige client code to configure the thread in which
/// this runs with a single threaded executor.
///
/// ### Errors
///
/// - When using with a threaded executor, this method will return a [`ErrorKind::SpawnLocalOnThreadPool`]. Since
/// the signature doesn't require [`Send`] on the future, it can never be sent on a threadpool.
/// - When using _localpool_, this method can return a spawn error if the executor has been shut down.
///   See the [docs for the futures library](https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.18/futures/task/struct.SpawnError.html). I haven't really found a way to trigger this error.
///   You can call [localpool::run] and spawn again afterwards.
/// - If you call this without an initialized executor, [`ErrorKind::NoExecutorInitialized`] is returned.
//
pub fn spawn_local( fut: impl Future< Output=() > + 'static ) -> Result< (), Error >
{
	EXEC.with( move |exec|
	{
		match exec.get()
		{
			Some(e) => e.spawn_local( fut )                           ,
			None    => Err( ErrorKind::NoExecutorInitialized.into() ) ,
		}
	})
}


/// Spawn a future and recover the output or just `.await` it to make sure it's finished.
/// Since different executors return different types, we have to Box the returned future.
///
/// To avoid boxing, use [`spawn`] with [`FutureExt::remote_handle`](https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.18/futures/future/trait.FutureExt.html#method.remote_handle). Or use providers directly, eg. async-std always
/// returns a `JoinHandle`. You could call async-std's spawn method directly, knowing that worker threads might
/// not be set up to end further calls to [`spawn`] to the async-std executor. Only do this if the spawned future
/// will not call [`spawn`] and friends.
///
/// ### Example
#[ cfg_attr( all( feature = "juliex", feature = "macros" ), doc = r##"
```
use async_runtime as rt;

#[ rt::juliex ]
//
async fn main()
{
   let handle = rt::spawn_handle( async
   {
      "hello"

   }).expect( "spawn on localpool" );

   assert_eq!( "hello", handle.await );
}

```
"##)]
///
/// ### Errors
/// - If you call this without an initialized executor, [`ErrorKind::NoExecutorInitialized`] is returned.

//
pub fn spawn_handle<T: Send + 'static>( fut: impl Future< Output=T > + Send + 'static )

	-> Result< Box< dyn Future< Output=T > + Unpin + Send + 'static >, Error >

{
	EXEC.with( move |exec|
	{
		match exec.get()
		{
			Some(e) => e.spawn_handle( fut )                          ,
			None    => Err( ErrorKind::NoExecutorInitialized.into() ) ,
		}
	})
}



/// Spawn a future and recover the output for `!Send` futures. This does the same as [`spawn_handle`]
/// except the future does not have to be `Send`. Note that `Future::Output` still has to be `Send`.
/// We use [`FutureExt::remote_handle`](https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.18/futures/future/trait.FutureExt.html#method.remote_handle) behind the scenes, and that requires the output to be Send,
/// even though it shoulnd't have to be.
///
/// ### Errors
/// - If you call this without an initialized executor, [`ErrorKind::NoExecutorInitialized`] is returned.
//
pub fn spawn_handle_local<T: 'static + Send>( fut: impl Future< Output=T > + 'static )

	-> Result< Box< dyn Future< Output=T > + 'static + Unpin >, Error >

{
	EXEC.with( move |exec|
	{
		match exec.get()
		{
			Some(e) => e.spawn_handle_local( fut )                    ,
			None    => Err( ErrorKind::NoExecutorInitialized.into() ) ,
		}
	})
}




/// Which executor is configured for the current thread?
///
/// If you are a library author you can use this to generate a clean error message
/// if you have a hard requirement for a certain executor.
//
pub fn current_rt() -> Option<Config>
{
	EXEC.with( move |exec|
	{
		exec.get().map( |e| e.config() )
	})
}



/// Block the current thread until the given future resolves and return the Output.
/// This just forwards to `futures::executor::block_on` under the hood.
///
/// **Warning** - this method does not play well with the rest of the executors. If you
/// use the localpool executor, only the future you pass here will be polled. Futures
/// that are "spawned" will just not run.
///
/// If you use juliex or async_std, the threadpool will continue working, but block_on will not wait
/// until all your futures have finished. As soon as the future you block on finishes,
/// if `block_on` is the last statement of your program, the program will just end, regardless
/// of other futures still on the threadpool.
///
/// In general you shouldn't block the thread when you are in an async context.
///
/// **Note:** This method is not available on WASM, since WASM currently does not allow blocking
/// the current thread.
//
#[ cfg(not( target_arch = "wasm32" )) ]
//
pub fn block_on< F: Future >( fut: F ) -> F::Output
{
	futures::executor::block_on( fut )
}


