/// The configuration for wich executor that should be used on this thread.
//
#[ derive( Debug, Clone, Hash, PartialEq, Eq ) ]
//
pub enum RtConfig
{
	/// A threadpool. Currently [juliex](https://github.com/withoutboats/juliex), but might change in the future.
	//
	Pool  ,

	/// An executor that runs futures on the current thread, capable of running `!`[`Send`] futures. Currently uses
	/// `futures::executor::LocalPool`.
	//
	Local ,
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