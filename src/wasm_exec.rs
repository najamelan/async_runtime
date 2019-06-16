use
{
	crate                :: { import::*, RtErr         } ,
	wasm_bindgen_futures :: { futures_0_3::spawn_local } ,
};


/// An executor that works on WASM.
//
pub struct WasmExec
{
	spawned: Rc<RefCell<Vec< Pin<Box< dyn Future< Output = () > + 'static >>>>> ,
	running: RefCell<bool>                                                      ,
}



impl Default for WasmExec
{
	fn default() -> Self
	{
		WasmExec
		{
			spawned: Rc::new( RefCell::new( vec![] ) ),
			running: RefCell::new( false )            ,
		}
	}
}



impl WasmExec
{
	/// Run all spawned futures to completion.
	//
	pub fn run( &self )
	{
		let spawned = self.spawned.clone();

		let task = async move
		{
			let mut v = spawned.borrow_mut();

			for fut in v.drain(..)
			{
				spawn_local( fut );
			}
		};

		{ *self.running.borrow_mut() = true; }

		spawn_local( task );
	}


	/// Spawn a future to be run on the LocalPool (current thread)
	//
	pub fn spawn( &self, fut: Pin<Box< dyn Future< Output = () > + 'static >> ) -> Result< (), RtErr >
	{
		if *self.running.borrow()
		{
			spawn_local( fut );
		}

		else
		{
			self.spawned.borrow_mut().push( fut );
		}

		Ok(())
	}


	/// The Executor trait requires this, but wasm doesn't have threads yet!
	//
	pub fn spawn_pool( &self, _fut: Pin<Box< dyn Future< Output = () > + 'static >> ) -> Result< (), RtErr >
	{
		unreachable!()
	}
}
