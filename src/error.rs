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
	/// You should not call [rt::init](crate::init) twice on the same thread. In general if you are a library
	/// author, you should not call it unless you started the thread. Otherwise just call [rt::spawn](crate::spawn)
	/// and let the client code decide which executor shall be used.
	//
	DoubleExecutorInit,

	/// An backend error happened while trying to spawn:
	///
	/// - Spawning on wasm   is infallible.
	/// - Spawning on juliex is infallible (as long as you don't call [rt::spawn_local](crate::spawn_local)).
	/// - Spawning on futures::executor::LocalPool can fail with [futures::task::SpawnError].
	///   The only reason for this is that the executor was shut down.
	///
	/// Note that even though certain executors are infallible right now, that might change in the
	/// future, notably WASM is bound to change quite alot over time.
	//
	Spawn,

	/// When some code in your project (possibly a dependency) uses spawn_local because the future they spawn is
	/// `!Send`, you must use the localpool for the thread in which this code is run. It's simply not possible
	/// to spawn a `!Send` future on a threadpool.
	//
	SpawnLocalOnThreadPool,

	/// You tried to use a functionality specific to a certain executor while another executor was being
	/// used for this thread.
	//
	WrongExecutor,

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


	/// Allows matching on the error kind
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
