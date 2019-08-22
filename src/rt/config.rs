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

	/// A threadpool executor from the async-std crate.
	//
	#[ cfg( feature = "async_std" ) ]
	//
	AsyncStd,

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
	#[ cfg(all( feature = "bindgen", target_arch = "wasm32" )) ]
	//
	fn default() -> Self
	{
		return Config::Bindgen;
	}


	#[ cfg( feature = "localpool" ) ]
	//
	fn default() -> Self
	{
		return Config::LocalPool;
	}


	#[ cfg(all( feature = "juliex", not(feature = "localpool") )) ]
	//
	fn default() -> Self
	{
		return Config::Juliex;
	}


	#[ cfg(all( feature = "async_std", not(feature = "localpool"), not(feature = "juliex") )) ]
	//
	fn default() -> Self
	{
		return Config::AsyncStd;
	}


	#[ cfg(all( not(feature = "async_std"), not(feature = "bindgen"), not(feature = "localpool"), not(feature = "juliex") )) ]
	//
	fn default() -> Self
	{
		panic!( "No executor enabled. You need to add a dependency on naja_async_runtime with at least one feature to enable an executor" );
	}
}
