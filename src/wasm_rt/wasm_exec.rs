use
{
	crate                :: { import::*, RtErr, WasmExecConfig } ,
	wasm_bindgen_futures :: { futures_0_3::spawn_local                    } ,
};


/// An executor that works on WASM.
//
pub struct WasmExec
{
	_config: WasmExecConfig
}



impl Default for WasmExec
{
	fn default() -> Self
	{
		WasmExec::new( WasmExecConfig::default() )
	}
}



impl WasmExec
{
	/// Create a new WasmExec from a configuration
	//
	pub fn new( config: WasmExecConfig ) -> Self
	{
		match &config
		{
			&WasmExecConfig::Local => WasmExec{ _config: config },
			&WasmExecConfig::Pool  => panic!( "Wasm does not have threads atm. Please initiate with a localpool executor" ),
		}
	}


	/// Getter for active executor configuration
	//
	pub fn config( &self ) -> &WasmExecConfig
	{
		&self._config
	}


	/// Spawn a future to be run on the LocalPool (current thread)
	//
	pub fn spawn( &self, fut: impl Future< Output = () > + 'static + Send ) -> Result< (), RtErr >
	{
		spawn_local( fut );

		Ok(())
	}


	/// Spawn a future to be run on the LocalPool (current thread)
	//
	pub fn spawn_local( &self, fut: impl Future< Output = () > + 'static ) -> Result< (), RtErr >
	{
		spawn_local( fut );

		Ok(())
	}
}
