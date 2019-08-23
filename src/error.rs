use crate::{ import::* };


/// The error type for errors happening in `async_runtime`.
//
#[ derive( Debug ) ]
//
pub struct Error
{
	kind: ErrorKind,
}

impl StdError for Error {}



/// The different kind of errors that can happen when you use the `async_runtime` API.
//
#[ derive( Copy, Clone, PartialEq, Eq, Debug ) ]
//
pub enum ErrorKind
{
	/// You should not call [init](crate::init) twice on the same thread. In general if you are a library
	/// author, you should not call it unless you started the thread. Otherwise just call [spawn](crate::spawn)
	/// and let the client code decide which executor shall be used.
	/// If you need to call [init](crate::init) several times, you can either verify no executor is set first
	/// (with [current_rt](crate::current_rt)) or use [init_allow_same](crate::init_allow_same)
	//
	DoubleExecutorInit,

	/// An backend error happened while trying to spawn:
	///
	/// - Spawning is infallible on: _juliex_, _async-std_, _bindgen_..
	/// - Spawning on _localpool_ can fail with [`futures::task::SpawnError`](https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.18/futures/task/struct.SpawnError.html).
	///   The only reason for this is that the executor was shut down. I haven't found a way to trigger this error.
	//
	Spawn,

	/// When some code in your project (possibly a dependency) uses [`spawn_local`](crate::spawn_local) because
	/// the future they spawn is `!Send`, you must use the localpool for the thread in which this code is run.
	/// It's simply not possible to spawn a `!Send` future on a threadpool.
	//
	SpawnLocalOnThreadPool,

	/// You tried to use a functionality specific to a certain executor while another executor was being
	/// used for this thread.
	//
	WrongExecutor,

	/// You tried to call a spawn function on a thread that has no executor initialized. Please use
	/// [`init`](crate::init) first.
	//
	NoExecutorInitialized,

	/// Protect against adding other options being breaking changes.
	//
	__Nonexhaustive,
}


impl fmt::Display for ErrorKind
{
	fn fmt( &self, f: &mut fmt::Formatter<'_> ) -> fmt::Result
	{
		match self
		{
			Self::DoubleExecutorInit => fmt::Display::fmt( "DoubleExecutorInit: Cannot initialize global executor twice.", f ) ,

			Self::Spawn => fmt::Display::fmt( "Spawn: Failed to spawn a future.", f ) ,

			Self::SpawnLocalOnThreadPool => fmt::Display::fmt( "Spawn: You can not spawn `!Send` futures on a thread pool. If your feature is `Send`, use `rt::spawn`, otherwise initialize this thread with a Local executor.", f ) ,

			Self::WrongExecutor => fmt::Display::fmt( "You tried to use a functionality specific to a certain executor while another executor was being used for this thread.", f ) ,

			Self::NoExecutorInitialized => fmt::Display::fmt( "You must initialize an executor on this thread before calls to spawn.", f ) ,

			_ => unreachable!(),
		}
	}
}


impl fmt::Display for Error
{
	fn fmt( &self, f: &mut fmt::Formatter<'_> ) -> fmt::Result
	{
		write!( f, "async_runtime::Error: {}", &self.kind )
	}
}


impl Error
{
	/// Create a new error from a specific kind.
	//
	pub fn new( kind: ErrorKind ) -> Self
	{
		Error { kind }
	}


	/// Allows matching on the error kind.
	//
	pub fn kind( &self ) -> &ErrorKind
	{
		&self.kind
	}
}


impl From<ErrorKind> for Error
{
	fn from( kind: ErrorKind ) -> Error
	{
		Error { kind }
	}
}
