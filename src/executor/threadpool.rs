use crate :: { self as rt, import::*, Error, ErrorKind };



/// A threapool from the futures library.
//
#[ derive( Debug, Default, Clone ) ]
//
pub(crate) struct ThreadPool {}

static THREADPOOL: SyncOnceCell<FutThreadPool> = SyncOnceCell::new();


impl ThreadPool
{
	/// Create a new ThreadPool from a [Config](rt::Config) configuration.
	//
	pub(crate) fn new() -> Self
	{
		// We create one global threadpool executor. Then we set the worker threads of this executor
		// to spawn on the same ThreadPool again. This means that while constructing the global ThreadPool,
		// we will re-enter this constructor for each worker thread.
		//
		THREADPOOL.get_or_init( ||

			FutThreadPool::builder()

				.name_prefix( "async_runtime_threadpool_worker" )

				.after_start( |_|
				{
					rt::init( rt::Config::ThreadPool ).expect( "set executor on juliex working thread" );
				})

				.create().expect( "Create futures threadpool" )

		);

		Self {}
	}



	pub(crate) fn spawn( &self, fut: impl Future< Output = () > + 'static + Send ) -> Result< (), Error >
	{
		// We can unwrap, since the constructor guarantees that the pool is created, we are sure it exists.
		//
		THREADPOOL.get().unwrap().spawn( fut ).map_err( |_| ErrorKind::Spawn.into() )
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
