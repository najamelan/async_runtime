use
{
	crate :: { import::*, Error } ,
	super :: { Config           } ,
};



#[ cfg( feature = "localpool" ) ] pub mod localpool ;
#[ cfg( feature = "async_std" ) ] pub mod async_std ;
#[ cfg( feature = "bindgen"   ) ]     mod bindgen   ;
#[ cfg( feature = "juliex"    ) ]     mod juliex    ;


#[ cfg( feature = "async_std" ) ] use async_std :: AsyncStd  ;
#[ cfg( feature = "bindgen"   ) ] use bindgen   :: Bindgen   ;
#[ cfg( feature = "juliex"    ) ] use juliex    :: Juliex    ;
#[ cfg( feature = "localpool" ) ] use localpool :: LocalPool ;


/// The different executors we support.
//
pub(crate) enum Executor
{
	#[ cfg( feature = "juliex" ) ]
	//
	Juliex( Juliex ),

	#[ cfg( feature = "async_std" ) ]
	//
	AsyncStd( AsyncStd ),

	/// An executor that runs futures on the current thread, capable of running `!`[`Send`] futures. Uses
	/// `futures::executor::LocalPool`.
	//
	#[ cfg( feature = "localpool" ) ]
	//
	LocalPool( LocalPool ),

	/// An executor that uses wasm-bindgen-futures under the hood. This is the only executor available on wasm
	/// at the moment. It is also only available on the wasm32-unknown-unknown target.
	//
	#[ cfg( feature = "bindgen" ) ]
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
			#[ cfg( feature = "async_std" ) ] Config::AsyncStd  => Self::AsyncStd ( AsyncStd ::new() ),
			#[ cfg( feature = "juliex"    ) ] Config::Juliex    => Self::Juliex   ( Juliex   ::new() ),
			#[ cfg( feature = "bindgen"   ) ] Config::Bindgen   => Self::Bindgen  ( Bindgen  ::new() ),

			_ => unreachable!(),
		}
	}

	pub(crate) fn config( &self ) -> Config
	{
		match self
		{
			#[ cfg( feature = "localpool" ) ] Self::LocalPool(_) => Config::LocalPool ,
			#[ cfg( feature = "juliex"    ) ] Self::Juliex   (_) => Config::Juliex    ,
			#[ cfg( feature = "async_std" ) ] Self::AsyncStd (_) => Config::AsyncStd  ,
			#[ cfg( feature = "bindgen"   ) ] Self::Bindgen  (_) => Config::Bindgen   ,

			_ => unreachable!(),
		}
	}


	// For the case where we compile without an executor enabled, the fut variable will be unused.
	//
	#[ allow( unused_variables ) ]
	//
	pub(crate) fn spawn( &self, fut: impl Future< Output = () > + 'static + Send ) -> Result< (), Error >
	{
		match self
		{
			#[ cfg( feature = "localpool" ) ] Self::LocalPool (e) => e.spawn( fut ),
			#[ cfg( feature = "juliex"    ) ] Self::Juliex    (e) => e.spawn( fut ),
			#[ cfg( feature = "async_std" ) ] Self::AsyncStd  (e) => e.spawn( fut ),
			#[ cfg( feature = "bindgen"   ) ] Self::Bindgen   (e) => e.spawn( fut ),

			_ => unreachable!(),
		}
	}


	// For the case where we compile without an executor enabled, the fut variable will be unused.
	//
	#[ allow( unused_variables ) ]
	//
	pub(crate) fn spawn_local( &self, fut: impl Future< Output = () > + 'static ) -> Result< (), Error >
	{
		match self
		{
			#[ cfg( feature = "localpool" ) ] Self::LocalPool (e) => e.spawn_local( fut ),
			#[ cfg( feature = "juliex"    ) ] Self::Juliex    (e) => e.spawn_local( fut ),
			#[ cfg( feature = "async_std" ) ] Self::AsyncStd  (e) => e.spawn_local( fut ),
			#[ cfg( feature = "bindgen"   ) ] Self::Bindgen   (e) => e.spawn_local( fut ),

			_ => unreachable!(),
		}
	}




	/// Spawn a future and recover the output.
	//
	// For the case where we compile without an executor enabled, the fut variable will be unused.
	//
	#[ allow( unused_variables ) ]
	//
	pub(crate) fn spawn_handle<T: 'static + Send>( &self, fut: impl Future< Output=T > + Send + 'static )

		-> Result< Box< dyn Future< Output=T > + Unpin + Send + 'static >, Error >

	{
		match self
		{
			#[ cfg( feature = "localpool" ) ] Self::LocalPool (e) => e.spawn_handle( fut ),
			#[ cfg( feature = "juliex"    ) ] Self::Juliex    (e) => e.spawn_handle( fut ),
			#[ cfg( feature = "async_std" ) ] Self::AsyncStd  (e) => e.spawn_handle( fut ),
			#[ cfg( feature = "bindgen"   ) ] Self::Bindgen   (e) => e.spawn_handle( fut ),

			_ => unreachable!(),
		}
	}



	/// Spawn a future and recover the output for `!Send` futures.
	//
	// For the case where we compile without an executor enabled, the fut variable will be unused.
	//
	#[ allow( unused_variables ) ]
	//
	pub(crate) fn spawn_handle_local<T: 'static + Send>( &self, fut: impl Future< Output=T > + 'static )

		-> Result< Box< dyn Future< Output=T > + 'static + Unpin >, Error >

	{
		match self
		{
			#[ cfg( feature = "localpool" ) ] Self::LocalPool (e) => e.spawn_handle_local( fut ),
			#[ cfg( feature = "juliex"    ) ] Self::Juliex    (e) => e.spawn_handle_local( fut ),
			#[ cfg( feature = "async_std" ) ] Self::AsyncStd  (e) => e.spawn_handle_local( fut ),
			#[ cfg( feature = "bindgen"   ) ] Self::Bindgen   (e) => e.spawn_handle_local( fut ),

			_ => unreachable!(),
		}
	}
}
