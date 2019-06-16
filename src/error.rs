use crate::{ import::* };


/// The main error type for thespis_impl. Use [`RtErr::kind()`] to know which kind of
/// error happened. RtErrKind implements Eq, so you can the following if all you want to
/// know is the kind of error. You can obviously also match the data contained in the RtErrKind
/// if you want, but you don't have to:
///
/// ```ignore
/// match return_a_result()
/// {
///    Err(e) =>
///    {
///       match e.kind()
///       {
///          RtErrKind::MailboxFull{..} => println!( "{}", e ),
///          _ => {},
///       }
///
///       if let RtErrKind::MailboxFull{..} = e.kind()
///       {
///          println!( "{}", e );
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



/// The different kind of errors that can happen when you use the thespis_impl API.
//
#[ derive( Clone, PartialEq, Eq, Debug, Fail ) ]
//
pub enum RtErrKind
{
	#[ fail( display = "A connection error happened: {}", what ) ]
	//
	Connection { what: String },

	#[ fail( display = "DoubleExecutorInit: Cannot initialize global executor twice" ) ]
	//
	DoubleExecutorInit,

	#[ fail( display = "Spawn: Failed to spawn a future in: {}", context ) ]
	//
	Spawn { context: String },

	#[ fail( display = "Timeout: {}", context ) ]
	//
	Timeout { context: String },
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
	fn fmt( &self, f: &mut fmt::Formatter ) -> fmt::Result
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
		RtErr { inner: inner }
	}
}


// TODO: this no longer compiles. It compiles fine in thespis, but not in this crate even though this
// file is largely copy/paste. The problem is that there is a blanket impl for Fail in failure for every
// E: std::error::Error + 'static + Send + Sync
//
// impl std::error::Error for RtErr {}


