use crate :: { self as rt, import::*, Error, ErrorKind };



/// We currently only support a global juliex threadpool. In principle this is the only supported
/// executor that allows full control. We could expose an interface that allows users to control
/// the lifetime and scope of a juliex threadpool.
//
#[ derive( Debug, Default, Clone ) ]
//
pub(crate) struct Juliex {}

static JULIEX_POOL: SyncOnceCell<juliex_crate::ThreadPool> = SyncOnceCell::new();


impl Juliex
{
	/// Create a new Juliex from a [Config](rt::Config) configuration.
	//
	pub(crate) fn new() -> Self
	{
		// We create one global juliex executor. Then we set the worker threads of this executor
		// to spawn on Juliex executors again. This means that while constructing the global juliex::ThreadPool,
		// we will re-enter this constructor for each worker thread.
		//
		// OnceCell should be fine being called from several threads. When get_or_init returns an error,
		// it means that we are already
		//
		JULIEX_POOL.get_or_init( ||

			juliex_crate::ThreadPool::with_setup( ||
			{
				rt::init( rt::Config::Juliex ).expect( "set executor on juliex working thread" );
			})

		);

		Self {}
	}



	pub(crate) fn spawn( &self, fut: impl Future< Output = () > + 'static + Send ) -> Result< (), Error >
	{
		// We can unwrap, since the constructor guarantees that the pool is created, we are sure it exists.
		//
		JULIEX_POOL.get().unwrap().spawn( fut );

		Ok(())
	}



	pub(crate) fn spawn_local( &self, _: impl Future< Output = () > + 'static  ) -> Result< (), Error >
	{
		Err( ErrorKind::SpawnLocalOnThreadPool.into() )
	}



	pub(crate) fn spawn_handle<T: 'static + Send>( &self, fut: impl Future< Output=T > + Send + 'static )

		-> Result< Box< dyn Future< Output=T > + Send + 'static + Unpin >, Error >

	{
		let (fut, handle) = fut.remote_handle();

		self.spawn( fut )?;
		Ok(Box::new( handle ))
	}



	pub(crate) fn spawn_handle_local<T: 'static + Send>( &self, _: impl Future< Output=T > + 'static )

		-> Result< Box< dyn Future< Output=T > + 'static + Unpin >, Error >

	{
		Err( ErrorKind::SpawnLocalOnThreadPool.into() )
	}
}
