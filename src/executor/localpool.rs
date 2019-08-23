//! Provides localpool executor specific functionality.

use crate :: { self as rt, import::*, Error, ErrorKind };


/// An executor that uses [futures 0.3 LocalPool](https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.16/futures/executor/struct.LocalPool.html) or [LocalPool](https://docs.rs/LocalPool) threadpool under the hood.
/// Normally you don't need to construct this yourself, just use the [`rt`](crate::rt) module methods to spawn futures.
//
#[ derive( Debug ) ]
//
pub(crate) struct LocalPool
{
	pool   : RefCell<FutLocalPool>,
	spawner: RefCell<LocalSpawner>,
}



impl LocalPool
{
	/// Create a new LocalPool from an [Config](crate::Config) configuration.
	//
	pub(crate) fn new() -> Self
	{
		let pool    = FutLocalPool::new();
		let spawner = pool.spawner();

		Self { pool: RefCell::new( pool ), spawner: RefCell::new( spawner ) }
	}


	/// Run all spawned futures to completion. Note that this does nothing for the threadpool,
	/// but if you are using a local pool, you will need to run this or futures will not be polled.
	/// This blocks the current thread.
	//
	pub(crate) fn run( &self )
	{
		self.pool.borrow_mut().run()
	}



	pub(crate) fn spawn( &self, fut: impl Future< Output = () > + 'static + Send ) -> Result< (), Error >
	{
		self.spawn_local( fut )
	}



	pub(crate) fn spawn_local( &self, fut: impl Future< Output = () > + 'static  ) -> Result< (), Error >
	{
		self.spawner.borrow_mut().spawn_local( fut ).map_err( |_| ErrorKind::Spawn.into() )
	}



	pub(crate) fn spawn_handle<T: 'static + Send>( &self, fut: impl Future< Output=T > + Send + 'static )

		-> Result< Box< dyn Future< Output=T > + Send + 'static + Unpin >, Error >

	{
		let (fut, handle) = fut.remote_handle();

		self.spawn_local( fut )?;
		Ok(Box::new( handle ))
	}



	pub(crate) fn spawn_handle_local<T: 'static + Send>( &self, fut: impl Future< Output=T > + 'static )

		-> Result< Box< dyn Future< Output=T > + 'static + Unpin >, Error >
	{
		let (fut, handle) = fut.remote_handle();

		self.spawn_local( fut )?;
		Ok(Box::new( handle ))
	}
}


/// Run all spawned futures to completion. You must run this after spawning on the local pool or
/// futures won't be polled. Do not call it from within a spawned task, or your program will hang or panic.
//
pub fn run() -> Result< (), Error >
{
	rt::EXEC.with( move |exec|
	{
		if let super::Executor::LocalPool( e ) = exec.get().unwrap()
		{
			Ok( e.run() )
		}

		else { Err( Error::from( ErrorKind::WrongExecutor ) ) }
	})
}
