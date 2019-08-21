/// The configuration for wich executor that should be used on this thread.
//
#[ derive( Debug, Copy, Clone, Hash, PartialEq, Eq ) ]
//
pub enum Config
{
	/// A threadpool executor from the juliex crate.
	//
	#[ cfg( feature = "juliex" ) ]
	//
	Juliex,

	/// An executor that runs futures on the current thread, capable of running `!`[`Send`] futures. Uses
	/// `futures::executor::LocalPool`.
	//
	#[ cfg( feature = "localpool" ) ]
	//
	LocalPool,

	/// An executor that uses wasm-bindgen-futures under the hood. This is the only executor available on wasm
	/// at the moment. It is also only available on the wasm32-unknown-unknown target.
	//
	#[ cfg( feature = "bindgen" ) ]
	//
	Bindgen,

	/// Protect against adding other options being breaking changes.
	//
	__Nonexhaustive,
}


impl Default for Config
{
	fn default() -> Self
	{
		#[ cfg( feature = "bindgen" ) ]
		//
		return Config::Bindgen;


		#[ cfg( feature = "localpool" ) ]
		//
		return Config::LocalPool;
	}
}
