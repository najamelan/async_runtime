use
{
	crate :: { import::*, RtErr } ,
	super :: { Config           } ,
};


#[ cfg( feature = "juliex"    ) ] use super::Juliex    ;
#[ cfg( feature = "localpool" ) ] use super::LocalPool ;
#[ cfg(all( feature = "bindgen", target_arch = "wasm32" )) ] use super::Bindgen;


/// The different executors we support.
//
pub(crate) enum Executor
{
	#[ cfg( feature = "juliex" ) ]
	//
	Juliex( Juliex ),

	/// An executor that runs futures on the current thread, capable of running `!`[`Send`] futures. Uses
	/// `futures::executor::LocalPool`.
	//
	#[ cfg( feature = "localpool" ) ]
	//
	LocalPool( LocalPool ),

	/// An executor that uses wasm-bindgen-futures under the hood. This is the only executor available on wasm
	/// at the moment. It is also only available on the wasm32-unknown-unknown target.
	//
	#[ cfg(all( feature = "bindgen", target_arch = "wasm32" )) ]
	//
	Bindgen( Bindgen ),

	/// Protect against adding other options being breaking changes.
	//
	__Nonexhaustive,
}


impl Executor
{
	pub(crate) fn new( config: Config ) -> Self
	{
		match config
		{
			#[ cfg( feature = "localpool" ) ] Config::LocalPool => Self::LocalPool( LocalPool::new() ),
			#[ cfg( feature = "juliex"    ) ] Config::Juliex    => Self::Juliex   ( Juliex   ::new() ),
			#[ cfg(all( feature = "bindgen", target_arch = "wasm32" )) ] Config::Bindgen   => Self::Bindgen  ( Bindgen  ::new() ),

			_ => unreachable!(),
		}
	}

	pub(crate) fn config( &self ) -> Config
	{
		match self
		{
			#[ cfg( feature = "localpool" ) ] Self::LocalPool(_) => Config::LocalPool ,
			#[ cfg( feature = "juliex"    ) ] Self::Juliex   (_) => Config::Juliex    ,
			#[ cfg(all( feature = "bindgen", target_arch = "wasm32" )) ] Self::Bindgen  (_) => Config::Bindgen   ,

			_ => unreachable!(),
		}
	}


	pub fn run( &self )
	{
		match self
		{
			#[ cfg( feature = "localpool" ) ] Self::LocalPool (e) => e.run(),
			#[ cfg( feature = "juliex"    ) ] Self::Juliex    (_) => {}
			#[ cfg(all( feature = "bindgen", target_arch = "wasm32" )) ] Self::Bindgen   (_) => {}

			_ => unreachable!(),
		}
	}


	pub fn spawn( &self, fut: impl Future< Output = () > + 'static + Send ) -> Result< (), RtErr >
	{
		match self
		{
			#[ cfg( feature = "localpool" ) ] Self::LocalPool (e) => e.spawn( fut ),
			#[ cfg( feature = "juliex"    ) ] Self::Juliex    (e) => e.spawn( fut ),
			#[ cfg(all( feature = "bindgen", target_arch = "wasm32" )) ] Self::Bindgen   (e) => e.spawn( fut ),

			_ => unreachable!(),
		}
	}


	pub fn spawn_local( &self, fut: impl Future< Output = () > + 'static ) -> Result< (), RtErr >
	{
		match self
		{
			#[ cfg( feature = "localpool" ) ] Self::LocalPool (e) => e.spawn_local( fut ),
			#[ cfg( feature = "juliex"    ) ] Self::Juliex    (e) => e.spawn_local( fut ),
			#[ cfg(all( feature = "bindgen", target_arch = "wasm32" )) ] Self::Bindgen   (e) => e.spawn_local( fut ),

			_ => unreachable!(),
		}
	}
}
