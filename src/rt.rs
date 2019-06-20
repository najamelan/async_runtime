//! This is a convenience module for setting a default runtime and allowing code throughout to use [spawn].
//! It means you don't have to pass an executor around everywhere.
//!
//! ### Usage
//!
//! ```
//! use async_runtime::*;
//!
//!
//! ```
//!

pub(crate) mod exec03;
pub use exec03::*;


use crate :: { import::*, RtConfig, RtErr, RtErrKind };


thread_local!
(
	static EXEC: OnceCell< Exec03 > = OnceCell::INIT;
);



/// Set the executor to use by default. Run this before calls to spawn. If you are a library
/// author, don't call this unless you create the thread, otherwise it's up to client code to
/// decide which executor to use. Just call [spawn].
///
/// This is optional and if you don't set this, the juliex thread pool executor will be set to
/// be the executor for this thread. If you want a LocalPool (runs on the current thread), you must call this:
///
/// ### Example
///
/// ```
/// use async_runtime::*;
///
/// rt::init( RtConfig::Local ).expect( "Set default executor" );
///
/// // ...spawn some tasks...
///
/// // Important, otherwise the local executor does not poll. For the threadpool this is not necessary,
/// // as futures will be polled immediately after spawning them.
/// //
/// rt::run();
///
/// ```
///
//
pub fn init( config: RtConfig ) -> Result< (), RtErr >
{
	EXEC.with( move |exec| -> Result< (), RtErr >
	{
		exec

			.set( Exec03::new( config ) )
			.map_err( |_| RtErrKind::DoubleExecutorInit.into() )
	})
}


/// If no executor is set, initialize with defaults
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


/// Spawn a future to be run on the default executor (set with [init] or juliex threadpool).
///
/// ```
/// # #![ feature( async_await) ]
/// #
/// use async_runtime::*;
///
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
/// future is [Send], you can just use [spawn]. It will always spawn on the executor set with
/// init.
//
pub fn spawn_local( fut: impl Future< Output=() > + 'static ) -> Result< (), RtErr >
{
	EXEC.with( move |exec| -> Result< (), RtErr >
	{
		default_init();
		exec.get().unwrap().spawn_local( fut )
	})
}


/// Run all spawned futures to completion.
/// Do not call it from within a spawned task.
/// TODO: test what happens if you do and document.
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
/// Note that if this returns None and you call [spawn], a default executor
/// will be initialized (with [default_init]), after which this will no longer return None.
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



/// Block the current thread until the given future resolves and return the Output.
//
pub fn block_on< F: Future >( fut: F ) -> F::Output
{
	futures::executor::block_on( fut )
}


