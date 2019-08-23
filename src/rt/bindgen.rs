use crate :: { import::*, RtErr };


/// An executor that uses [futures 0.3 LocalPool](https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.16/futures/executor/struct.LocalPool.html) or [Bindgen](https://docs.rs/Bindgen) threadpool under the hood.
/// Normally you don't need to construct this yourself, just use the [`rt`](crate::rt) module methods to spawn futures.
//
#[ derive( Debug ) ]
//
pub(crate) struct Bindgen {}



impl Bindgen
{
	/// Create a new Bindgen from an [Config](crate::Config) configuration.
	//
	pub(crate) fn new() -> Self
	{
		Self {}
	}


	/// Spawn a future to be run on the default executor. Note that this requires the
	/// future to be `Send` in order to work for both the local pool and the threadpool.
	/// When you need to spawn futures that are not `Send` on the local pool, please use
	/// [`spawn_local`](Bindgen::spawn_local).
	///
	/// ### Errors
	///
	/// - When using `Config::Bindgen` (currently Bindgen), this method is infallible.
	/// - When using `Config::LocalPool` (currently futures 0.3 LocalPool), this method can return a spawn
	/// error if the executor has been shut down. See the [docs for the futures library](https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.16/futures/task/struct.SpawnError.html). I haven't really found a way to trigger this error.
	/// You can call [crate::rt::run] and spawn again afterwards.
	///
	//
	pub(crate) fn spawn( &self, fut: impl Future< Output = () > + 'static + Send ) -> Result< (), RtErr >
	{
		self.spawn_local( fut )
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
	/// - When using `Config::Bindgen` (currently Bindgen), this method will return an error of kind [RtErrKind::SpawnLocalOnThreadPool](crate::RtErrKind::SpawnLocalOnThreadPool).
	///   Since the signature doesn't require [Send] on the future, it can never be sent on a threadpool.
	/// - When using `Config::LocalPool` (currently futures 0.3 LocalPool), this method can return a spawn
	/// error if the executor has been shut down. `spawn_local` will return an error of kind
	///  [RtErrKind::Spawn](crate::RtErrKind::Spawn).
	///
	/// See the [docs for the futures library](https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.18/futures/task/struct.SpawnError.html). I haven't really found a way to trigger this error,
	/// since you can call [rt::run](crate::rt::run) and spawn again afterwards.
	//
	pub(crate) fn spawn_local( &self, fut: impl Future< Output = () > + 'static  ) -> Result< (), RtErr >
	{
		spawn_local( fut );

		Ok(())
	}


	/// Spawn a future and recover the output.
	//
	pub(crate) fn spawn_handle<T: 'static + Send>( &self, fut: impl Future< Output=T > + Send + 'static )

		-> Result< Box< dyn Future< Output=T > + Unpin >, RtErr >

	{
		let (fut, handle) = fut.remote_handle();

		spawn_local( fut );
		Ok(Box::new( handle ))
	}



	/// Spawn a future and recover the output for `!Send` futures.
	//
	pub(crate) fn spawn_handle_local<T: 'static + Send>( &self, fut: impl Future< Output=T > + 'static )

		-> Result< Box< dyn Future< Output=T > + Unpin >, RtErr >
	{
		let (fut, handle) = fut.remote_handle();

		spawn_local( fut );
		Ok(Box::new( handle ))
	}
}
