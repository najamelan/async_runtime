use
{
	crate                :: { import::*, RtErr, RtConfig } ,
	wasm_bindgen_futures :: { spawn_local                } ,
};


/// An executor that works on WASM. Uses `wasm-bindgen-futures` as backend.
//
#[ derive( Debug ) ]
//
pub(crate) struct WasmExec
{
	_config: RtConfig
}



impl Default for WasmExec
{
	fn default() -> Self
	{
		WasmExec::new( RtConfig::default() )
	}
}



impl WasmExec
{
	/// Create a new WasmExec from a configuration
	//
	pub fn new( config: RtConfig ) -> Self
	{
		match config
		{
			RtConfig::Local => WasmExec{ _config: config },
			RtConfig::Pool  => panic!( "Wasm does not have threads atm. Please initiate with a localpool executor" ),
			_               => unreachable!(),
		}
	}


	/// Getter for active executor configuration
	//
	pub fn config( &self ) -> &RtConfig
	{
		&self._config
	}


	/// Spawn a future to be run on the default executor. Note that this requires the
	/// future to be `Send` in order to work for both the local pool and the threadpool.
	/// When you need to spawn futures that are not `Send` on the local pool, please use
	/// [`spawn_local`](WasmExec::spawn_local).
	///
	/// Note that this method is infallible right now, but it returns a result for consistency
	/// with the non-wasm API and to future proof it when the wasm context ressembles other
	/// targets more (threads, more executor options).
	//
	pub fn spawn( &self, fut: impl Future< Output = () > + 'static + Send ) -> Result< (), RtErr >
	{
		spawn_local( fut );

		Ok(())
	}


	/// Spawn a `!Send` future to be run on the LocalPool (current thread). Note that the executor must
	/// be created with a local pool configuration. This will err if you try to call this on an executor
	/// set up with a threadpool.
	///
	/// Note that this method is infallible right now, but it returns a result for consistency
	/// with the non-wasm API and to future proof it when the wasm context ressembles other
	/// targets more (threads, more executor options).
	//
	pub fn spawn_local( &self, fut: impl Future< Output = () > + 'static ) -> Result< (), RtErr >
	{
		spawn_local( fut );

		Ok(())
	}
}
