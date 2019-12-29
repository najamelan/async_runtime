use crate :: { import::*, Error };



// We only really have an interface to the spawn_local method. There are not threads on WASM, we cannot
// block the current thread. These futures are just passed to Javascript to be converted into promises.
//
#[ derive( Debug, Clone ) ]
//
pub(crate) struct Bindgen {}



impl Bindgen
{
	pub(crate) fn new() -> Self
	{
		Self {}
	}


	pub(crate) fn spawn( &self, fut: impl Future< Output = () > + 'static + Send ) -> Result< (), Error >
	{
		self.spawn_local( fut )
	}


	pub(crate) fn spawn_local( &self, fut: impl Future< Output = () > + 'static  ) -> Result< (), Error >
	{
		spawn_local( fut );

		Ok(())
	}


	pub(crate) fn spawn_handle<T: 'static + Send>( &self, fut: impl Future< Output=T > + Send + 'static )

		-> Result< Box< dyn Future< Output=T > + Send + 'static + Unpin >, Error >

	{
		let (fut, handle) = fut.remote_handle();

		spawn_local( fut );
		Ok(Box::new( handle ))
	}


	pub(crate) fn spawn_handle_local<T: 'static + Send>( &self, fut: impl Future< Output=T > + 'static )

		-> Result< Box< dyn Future< Output=T > + 'static + Unpin >, Error >
	{
		let (fut, handle) = fut.remote_handle();

		spawn_local( fut );
		Ok(Box::new( handle ))
	}
}
