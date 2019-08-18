/// The configuration for wich executor that should be used on this thread.
//
#[ derive( Debug, Copy, Clone, Hash, PartialEq, Eq ) ]
//
pub enum RtConfig
{
	/// A threadpool. Currently [juliex](https://github.com/withoutboats/juliex), but might change in the future.
	//
	#[ cfg( feature = "juliex" ) ]
	//
	Pool,

	/// An executor that runs futures on the current thread, capable of running `!`[`Send`] futures. Currently uses
	/// `futures::executor::LocalPool`.
	//
	Local,

	/// Protect against adding other options being breaking changes
	//
	__Nonexhaustive,
}


impl Default for RtConfig
{
	fn default() -> Self
	{
		#[ cfg( feature = "juliex" ) ]
		//
		return RtConfig::Pool;

		#[ cfg( not( feature = "juliex" ) ) ]
		//
		return RtConfig::Local;
	}
}
