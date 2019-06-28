use crate::{ import::* };


/// The error type for errors happening in `async_runtime`.
///
/// Use [`RtErr::kind()`] to know which kind of error happened. [RtErrKind] implements [Eq],
/// so you can do the following if all you want to know is the kind of error:
///
/// ```ignore
/// use async_runtime::*;
///
/// rt::init( RtConfig::Local ).expect( "Set default executor" );
///
/// match rt::init( RtConfig::Pool )
/// {
///    Err(e) =>
///    {
///       if let RtErrKind::DoubleExecutorInit = e.kind()
///       {
///          println!( "{}", e );
///       }
///
///       // This also works:
///       //
///       match e.kind()
///       {
///          RtErrKind::DoubleExecutorInit => println!( "{}", e ),
///          _ => {},
///       }
///    },
///
///    Ok(_) => {}
/// }
/// ```
//
#[ derive( Debug ) ]
//
pub struct RtErr
{
	inner: FailContext<RtErrKind>,
}



/// The different kind of errors that can happen when you use the `async_runtime` API.
//
#[ derive( Clone, PartialEq, Eq, Debug, Fail ) ]
//
pub enum RtErrKind
{
	/// You should not call [rt::init](crate::rt::init) twice on the same thread. In general if you are a library
	/// author, you should not call it unless you started the thread. Otherwise just call [rt::spawn](crate::rt::spawn)
	/// and let the client code decide which executor shall be used.
	//
	#[ fail( display = "DoubleExecutorInit: Cannot initialize global executor twice" ) ]
	//
	DoubleExecutorInit,

	/// An backend error happened while trying to spawn:
	///
	/// - Spawning on wasm   is infallible.
	/// - Spawning on juliex is infallible (as long as you don't call [rt::spawn_local](crate::rt::spawn_local)).
	/// - Spawning on futures::executor::LocalPool can fail with [futures::task::SpawnError].
	///   The only reason for this is that the executor was shut down.
	///
	/// Note that even though certain executors are infallible right now, that might change in the
	/// future, notably WASM is bound to change quite alot over time.
	//
	#[ fail( display = "Spawn: Failed to spawn a future in: {}", context ) ]
	//
	Spawn
	{
		/// Add contextual information to which future failed to spawn.
		///
		context: String
	},
}



impl Fail for RtErr
{
	fn cause( &self ) -> Option< &dyn Fail >
	{
		self.inner.cause()
	}

	fn backtrace( &self ) -> Option< &Backtrace >
	{
		self.inner.backtrace()
	}
}



impl fmt::Display for RtErr
{
	fn fmt( &self, f: &mut fmt::Formatter<'_> ) -> fmt::Result
	{
		fmt::Display::fmt( &self.inner, f )
	}
}


impl RtErr
{
	/// Allows matching on the error kind
	//
	pub fn kind( &self ) -> &RtErrKind
	{
		self.inner.get_context()
	}
}

impl From<RtErrKind> for RtErr
{
	fn from( kind: RtErrKind ) -> RtErr
	{
		RtErr { inner: FailContext::new( kind ) }
	}
}

impl From< FailContext<RtErrKind> > for RtErr
{
	fn from( inner: FailContext<RtErrKind> ) -> RtErr
	{
		RtErr { inner }
	}
}


// TODO: this no longer compiles. It compiles fine in thespis, but not in this crate even though this
// file is largely copy/paste. The problem is that there is a blanket impl for Fail in failure for every
// E: std::error::Error + 'static + Send + Sync
//
// impl std::error::Error for RtErr {}


