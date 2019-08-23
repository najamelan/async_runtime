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


	#[ cfg(any( feature = "bindgen", feature = "localpool", feature = "juliex", feature = "async_std" )) ]
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

mod error;
mod config   ;
mod executor ;

pub use error::*;
pub use config::*;


#[ cfg( feature = "async_std" ) ] pub use executor::async_std ;
#[ cfg( feature = "localpool" ) ] pub use executor::localpool ;


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



/// Set the executor to use by on this thread. Run this before calls to [spawn]\(_*\).
///
/// If you are a library author, don't call this unless you create the thread, otherwise it's up to client code to
/// decide which executor to use. Just call [spawn].
///
/// ### Errors
///
/// This method will fail with [ErrorKind::DoubleExecutorInit] if you call it twice on the same thread. There is
/// [init_allow_same] which will not return an error if you try to init with the same executor twice.
///
/// ### Example
#[cfg_attr(feature = "localpool", doc = r##"
```rust
use async_runtime as rt;

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

/// Set the executor to use by default. The difference with [init] is that this will not return
/// a [ErrorKind::DoubleExecutorInit] error if you init with the same executor twice. It will still err
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


/// Spawn a future to be run on the default executor (set with [init] or default, depending on `juliex feature`,
/// see documentation for rt::init).
///
/// ### Errors
///
/// - When using `Config::Juliex` (currently juliex), this method is infallible.
/// - When using `Config::LocalPool` (currently futures 0.3 LocalPool), this method can return a spawn
/// error if the executor has been shut down. See the [docs for the futures library](https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.16/futures/task/struct.SpawnError.html). I haven't really found a way to trigger this error.
/// You can call [localpool::run] and spawn again afterwards.
///
/// ### Example
#[ cfg_attr( feature = "localpool", doc = r##"
```
use async_runtime as rt;

rt::init( rt::Config::LocalPool ).expect( "no double executor init" );

rt::spawn( async
{
   println!( "async execution" );

});
```
"##)]
//
pub fn spawn( fut: impl Future< Output=() > + 'static + Send ) -> Result< (), Error >
{
	EXEC.with( move |exec| -> Result< (), Error >
	{
		exec.get().unwrap().spawn( fut )
	})
}


/// Spawn a future to be run on the LocalPool (current thread). This will return an error
/// if the current executor is the threadpool.
///
/// Does exactly the same as [spawn], but does not require the future to be [Send]. If your
/// future is [Send], you can just use [spawn]. It will always spawn on the default executor.
///
/// ### Errors
///
/// - When using `Config::Juliex` (currently juliex), this method will return a [ErrorKind::Spawn](crate::ErrorKind::Spawn). Since
/// the signature doesn't require [Send] on the future, it can never be sent on a threadpool.
/// - When using `Config::LocalPool` (currently futures 0.3 LocalPool), this method can return a spawn
/// error if the executor has been shut down. See the [docs for the futures library](https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.16/futures/task/struct.SpawnError.html). I haven't really found a way to trigger this error.
/// You can call [localpool::run] and spawn again afterwards.
//
pub fn spawn_local( fut: impl Future< Output=() > + 'static ) -> Result< (), Error >
{
	EXEC.with( move |exec| -> Result< (), Error >
	{
		exec.get().unwrap().spawn_local( fut )
	})
}



/// Spawn a future and recover the output.
//
pub fn spawn_handle<T: 'static + Send>( fut: impl Future< Output=T > + Send + 'static )

	-> Result< Box< dyn Future< Output=T > + Unpin >, Error >

{
	EXEC.with( move |exec|
	{
		exec.get().unwrap().spawn_handle( fut )
	})
}



/// Spawn a future and recover the output for `!Send` futures.
//
pub fn spawn_handle_local<T: 'static + Send>( fut: impl Future< Output=T > + 'static )

	-> Result< Box< dyn Future< Output=T > + Unpin >, Error >

{
	EXEC.with( move |exec|
	{
		exec.get().unwrap().spawn_handle_local( fut )
	})
}




/// Get the configuration for the current default executor.
/// Note that if this returns `None` and you call [`spawn`], a default executor
/// will be initialized, after which this will no longer return `None`.
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
/// use the localpool executor, only the future you pass here will be polled. futures
/// that are "spawned" will just not run.
///
/// If you use juliex, the threadpool will continue working, but block_on will not wait
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


