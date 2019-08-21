//! This is a convenience module for setting a default runtime and allowing code throughout to use [rt::spawn].
//! It means you don't have to pass an executor around everywhere.
//!
//! For examples, please look in the
//! [examples directory of the repository](https://github.com/najamelan/async_runtime/tree/master/examples).
//!

mod config   ;
mod executor ;

#[ cfg( feature = "juliex"    ) ] mod juliex                          ;
#[ cfg( feature = "juliex"    ) ] use juliex::Juliex                  ;
#[ cfg( feature = "juliex"    ) ] pub use naja_runtime_macros::juliex ;

#[ cfg( feature = "localpool" ) ] mod localpool                          ;
#[ cfg( feature = "localpool" ) ] use localpool::LocalPool               ;
#[ cfg( feature = "localpool" ) ] pub use naja_runtime_macros::localpool ;

#[ cfg( feature = "bindgen" ) ] mod bindgen                          ;
#[ cfg( feature = "bindgen" ) ] use bindgen::Bindgen                 ;
#[ cfg( feature = "bindgen" ) ] pub use naja_runtime_macros::bindgen ;


pub use
{
	naja_runtime_macros :: { * } ,
	config              :: { * } ,
};


use
{
	crate    :: { import::*, RtErr, RtErrKind } ,
	executor :: { Executor                    } ,
};


std::thread_local!
(
	static EXEC: OnceCell<Executor> = OnceCell::INIT;
);



/// Set the executor to use by default. Run this before calls to spawn. If you are a library
/// author, don't call this unless you create the thread, otherwise it's up to client code to
/// decide which executor to use. Just call [spawn].
///
/// This is optional and if you don't set this, the default executor depends on whether the `juliex`
/// feature is enabled for the crate. If it is, it is the default executor, otherwise it will be the
/// local pool. If it's enabled and you still want the local pool, use this method.
///
/// ### Errors
///
/// This method will fail with [RtErrKind::DoubleExecutorInit](crate::RtErrKind::DoubleExecutorInit) if you
/// call it twice on the same thread or if you have called [spawn] and thus the executor has been initialized
/// by default before you call init.
///
/// ### Example
///
/// ```
/// # #![ feature( async_await ) ]
/// #
/// use async_runtime::*;
///
/// rt::init( rt::Config::LocalPool ).expect( "Set default executor" );
///
/// // ...spawn some tasks...
/// //
/// rt::spawn( async {} ).expect( "spawn future" );
///
/// // Important, otherwise the local executor does not poll. For the threadpool this is not necessary,
/// // as futures will be polled immediately after spawning them.
/// //
/// rt::run();
/// ```
//
pub fn init( config: Config ) -> Result< (), RtErr >
{
	EXEC.with( move |exec| -> Result< (), RtErr >
	{
		exec.set( Executor::new( config ) ).map_err( |_| RtErrKind::DoubleExecutorInit.into() )
	})
}

/// Set the executor to use by default. The difference with init is that this will not return
/// an DoubleExecutorInit error if you init with the same executor twice. It will still err
/// if you try to set 2 different executors for this thread.
//
pub fn init_allow_same( config: Config ) -> Result< (), RtErr >
{
	if let Some(cfg) = current_rt() {
	if config == cfg
	{
		return Ok(())
	}}


	init( config )
}


/// If no executor is set, initialize with defaults (pool if juliex feature is enabled, local pool otherwise)
//
fn default_init()
{
	if current_rt().is_none()
	{
		init( Config::default() ).unwrap();
	}
}


/// Spawn a future to be run on the default executor (set with [init] or default, depending on `juliex feature`,
/// see documentation for rt::init).
///
/// ### Errors
///
/// - When using `Config::Juliex` (currently juliex), this method is infallible.
/// - When using `Config::LocalPool` (currently futures 0.3 LocalPool), this method can return a spawn
/// error if the executor has been shut down. See the [docs for the futures library](https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.16/futures/task/struct.SpawnError.html). I haven't really found a way to trigger this error.
/// You can call [crate::rt::run] and spawn again afterwards.
///
/// ### Example
///
/// ```
/// # #![ feature( async_await) ]
/// #
/// use async_runtime::*;
///
/// // This will run on the threadpool. For the local pool you must call [rt::init] and [rt::run].
/// //
/// rt::spawn( async
/// {
///    println!( "async execution" );
///
/// });
/// ```
//
pub fn spawn( fut: impl Future< Output=() > + 'static + Send ) -> Result< (), RtErr >
{
	EXEC.with( move |exec| -> Result< (), RtErr >
	{
		default_init();
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
/// - When using `Config::Juliex` (currently juliex), this method will return a [RtErrKind::Spawn](crate::RtErrKind::Spawn). Since
/// the signature doesn't require [Send] on the future, it can never be sent on a threadpool.
/// - When using `Config::LocalPool` (currently futures 0.3 LocalPool), this method can return a spawn
/// error if the executor has been shut down. See the [docs for the futures library](https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.16/futures/task/struct.SpawnError.html). I haven't really found a way to trigger this error.
/// You can call [rt::run](crate::rt::run) and spawn again afterwards.
//
pub fn spawn_local( fut: impl Future< Output=() > + 'static ) -> Result< (), RtErr >
{
	EXEC.with( move |exec| -> Result< (), RtErr >
	{
		default_init();
		exec.get().unwrap().spawn_local( fut )
	})
}



/// Spawn a future and recover the output.
//
pub fn spawn_handle<T: Send>( fut: impl Future< Output=T > + Send + 'static ) -> Result< RemoteHandle<T>, RtErr >
{
	EXEC.with( move |exec| -> Result< RemoteHandle<T>, RtErr >
	{
		default_init();

		let (fut, handle) = fut.remote_handle();
		exec.get().unwrap().spawn( fut )?;

		Ok( handle )
	})
}



/// Spawn a future and recover the output for `!Send` futures.
//
pub fn spawn_handle_local<T>( fut: impl Future< Output=T > + 'static ) -> Result< RemoteHandle<T>, RtErr >
{
	EXEC.with( move |exec| -> Result< RemoteHandle<T>, RtErr >
	{
		default_init();

		let (fut, handle) = fut.remote_handle();
		exec.get().unwrap().spawn_local( fut )?;

		Ok( handle )
	})
}



/// Run all spawned futures to completion. This is a no-op for the threadpool. However you must
/// run this after spawning on the local pool or futures won't be polled.
/// Do not call it from within a spawned task, or your program will hang or panic.
//
pub fn run()
{
	EXEC.with( move |exec|
	{
		default_init();
		exec.get().unwrap().run();
	});
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
/// **Note:** This method is not available on WASM, since WASM currently does not allow blocking
/// the current thread.
//
pub fn block_on< F: Future >( fut: F ) -> F::Output
{
	futures::executor::block_on( fut )
}


