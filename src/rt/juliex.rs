use crate :: { import::*, rt, RtErr, RtErrKind };



/// An executor that uses [futures 0.3 LocalPool](https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.16/futures/executor/struct.LocalPool.html) or [juliex](https://docs.rs/juliex) threadpool under the hood.
/// Normally you don't need to construct this yourself, just use the [`rt`](crate::rt) module methods to spawn futures.
//
#[ derive( Debug, Default ) ]
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


	/// Spawn a future to be run on the default executor. Note that this requires the
	/// future to be `Send` in order to work for both the local pool and the threadpool.
	/// When you need to spawn futures that are not `Send` on the local pool, please use
	/// [`spawn_local`](Juliex::spawn_local).
	///
	/// ### Errors
	///
	/// - When using `Config::Juliex` (currently juliex), this method is infallible.
	/// - When using `Config::LocalPool` (currently futures 0.3 LocalPool), this method can return a spawn
	/// error if the executor has been shut down. See the [docs for the futures library](https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.16/futures/task/struct.SpawnError.html). I haven't really found a way to trigger this error.
	/// You can call [crate::rt::run] and spawn again afterwards.
	///
	//
	pub(crate) fn spawn( &self, fut: impl Future< Output = () > + 'static + Send ) -> Result< (), RtErr >
	{
		// We can unwrap, since the constructor guarantees that the pool is created, we are sure it exists.
		//
		JULIEX_POOL.get().unwrap().spawn( fut );

		Ok(())
	}


	/// Spawn a `!Send` future to be run on the LocalPool (current thread). Note that the executor must
	/// be created with a local pool configuration. This will err if you try to call this on an executor
	/// set up with a threadpool.
	///
	/// Note that this will not complain if you call this with a `Send` future, but there is no reason to
	/// do so, and it will put restrictions on users of your code, as they will no longer be able to run
	/// your code on a thread that spawns on a threadpool.
	///
	/// ### Errors
	///
	/// - When using `Config::Juliex` (currently juliex), this method will return an error of kind [RtErrKind::SpawnLocalOnThreadPool](crate::RtErrKind::SpawnLocalOnThreadPool).
	///   Since the signature doesn't require [Send] on the future, it can never be sent on a threadpool.
	/// - When using `Config::LocalPool` (currently futures 0.3 LocalPool), this method can return a spawn
	/// error if the executor has been shut down. `spawn_local` will return an error of kind
	///  [RtErrKind::Spawn](crate::RtErrKind::Spawn).
	///
	/// See the [docs for the futures library](https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.18/futures/task/struct.SpawnError.html). I haven't really found a way to trigger this error,
	/// since you can call [rt::run](crate::rt::run) and spawn again afterwards.
	//
	pub(crate) fn spawn_local( &self, _: impl Future< Output = () > + 'static  ) -> Result< (), RtErr >
	{
		Err( RtErrKind::SpawnLocalOnThreadPool.into() )
	}


	/// Spawn a future and recover the output.
	//
	pub(crate) fn spawn_handle<T: 'static + Send>( &self, fut: impl Future< Output=T > + Send + 'static )

		-> Result< Box< dyn Future< Output=T > + Unpin >, RtErr >

	{
		let (fut, handle) = fut.remote_handle();

		self.spawn( fut )?;
		Ok(Box::new( handle ))
	}



	/// Spawn a future and recover the output for `!Send` futures.
	//
	pub(crate) fn spawn_handle_local<T: 'static + Send>( &self, _: impl Future< Output=T > + 'static )

		-> Result< Box< dyn Future< Output=T > + Unpin >, RtErr >

	{
		Err( RtErrKind::SpawnLocalOnThreadPool.into() )
	}
}
