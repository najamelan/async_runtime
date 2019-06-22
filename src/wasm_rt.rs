//! This is a convenience module for setting a default runtime and allowing code throughout to use [rt::spawn].
//! It means you don't have to pass an executor around everywhere.
//!
//! For a complete example running in a browser, please look in the
//! [examples directory of the repository](https://github.com/najamelan/async_runtime/tree/master/examples/wasm).
//!
use crate :: { import::*, RtConfig, RtErr, RtErrKind };


pub(crate) mod wasm_exec;
pub use wasm_exec::*;


thread_local!
(
	static EXEC: OnceCell< WasmExec > = OnceCell::INIT;
);



/// Set the executor to use by default. Run this before calls to spawn. If you are a library
/// author, don't call this unless you create the thread, otherwise it's up to client code to
/// decide which executor to use. Just call [spawn].
///
/// This is optional and if you don't set this, the default executor depends on whether the `juliex`
/// feature is enabled for the crate. If it is, it is the default executor, otherwise it will be the
/// local pool. If it's enabled and you still want the local pool, use this method.
///
/// ### Example
///
/// ```
/// #![ feature( async_await ) ]
///
/// use async_runtime::*;
///
/// rt::init( RtConfig::Local ).expect( "Set default executor" );
///
/// // ...spawn some tasks...
/// //
/// rt::spawn( async {} ).expect( "spawn future" );
///
/// // Important, otherwise the local executor does not poll. For the threadpool this is not necessary,
/// // as futures will be polled immediately after spawning them.
/// //
/// rt::run();
///
/// ```
//
pub fn init( config: RtConfig ) -> Result< (), RtErr >
{
	EXEC.with( move |exec| -> Result< (), RtErr >
	{
		exec

			.set( WasmExec::new( config ) )
			.map_err( |_| RtErrKind::DoubleExecutorInit.into() )
	})
}


/// If no executor is set, initialize with defaults (pool if juliex feature is enabled, local pool otherwise)
//
fn default_init()
{
	EXEC.with( move |exec|
	{
		if exec.get().is_none()
		{
			init( RtConfig::default() ).unwrap();
		}
	});
}


/// Spawn a future to be run on the default executor (set with [init] or default, depending on `juliex feature`,
/// see documentation for rt::init).
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
pub fn spawn( fut: impl Future< Output = () > + 'static + Send ) -> Result< (), RtErr >
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
//
pub fn spawn_local( fut: impl Future< Output = () > + 'static ) -> Result< (), RtErr >
{
	EXEC.with( move |exec| -> Result< (), RtErr >
	{
		default_init();
		exec.get().unwrap().spawn_local( fut )
	})
}



/// Get the configuration for the current default executor.
/// Note that if this returns `None` and you call [`spawn`], a default executor
/// will be initialized, after which this will no longer return `None`.
///
/// If you are a library author you can use this to generate a clean error message
/// if you have a hard requirement for a certain executor.
//
pub fn current_rt() -> Option<RtConfig>
{
	EXEC.with( move |exec|
	{
		if exec.get().is_none()
		{
			None
		}

		else
		{
			Some( exec.get().unwrap().config().clone() )
		}
	})
}


