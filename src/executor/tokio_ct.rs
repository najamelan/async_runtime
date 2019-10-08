//! Provides TokioCt executor specific functionality.

use crate :: { self as rt, import::*, Error, ErrorKind };


/// An executor that uses [tokio::runtime::current_thread::Runtime]
//
#[ derive( Debug ) ]
//
pub(crate) struct TokioCt
{
	runtime: RefCell< TokioCtRuntime >,
}



impl TokioCt
{
	/// Create a new TokioCt from an [Config](crate::Config) configuration.
	//
	pub(crate) fn new() -> Self
	{
		Self { runtime: RefCell::new( TokioCtRuntime::new().expect( "create tokio ct runtime" ) ) }
	}


	/// Run all spawned futures to completion. Note that this does nothing for the threadpool,
	/// but if you are using a local pool, you will need to run this or futures will not be polled.
	/// This blocks the current thread.
	//
	pub(crate) fn run( &self ) -> Result< (), Error >
	{
		self.runtime.borrow_mut().run().map_err( |_| ErrorKind::Run.into() )
	}


	pub(crate) fn spawn( &self, fut: impl Future< Output = () > + 'static + Send ) -> Result< (), Error >
	{
		self.spawn_local( fut )
	}



	pub(crate) fn spawn_local( &self, fut: impl Future< Output = () > + 'static  ) -> Result< (), Error >
	{
		self.runtime.borrow_mut().spawn( fut );
		Ok(())
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
	rt::EXEC.with( |some|
	{
		match some.get()
		{
			Some(super::Executor::TokioCt(e)) => e.run()                                       ,
			None                              => Err( ErrorKind::NoExecutorInitialized.into() ),
			_                                 => Err( ErrorKind::WrongExecutor.into()         ),
		}
	})
}
