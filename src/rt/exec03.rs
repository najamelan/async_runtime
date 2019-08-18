use crate :: { import::*, RtConfig, RtErr, RtErrKind };


/// An executor that uses [futures 0.3 LocalPool](https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.16/futures/executor/struct.LocalPool.html) or [juliex](https://docs.rs/juliex) threadpool under the hood.
/// Normally you don't need to construct this yourself, just use the [`rt`](crate::rt) module methods to spawn futures.
//
#[ derive( Debug ) ]
//
pub(crate) struct Exec03
{
	config : RtConfig                        ,
	local  : Option<RefCell< LocalPool    >> ,
	spawner: Option<RefCell< LocalSpawner >> ,
}



impl Default for Exec03
{
	fn default() -> Self
	{
		Exec03::new( RtConfig::default() )
	}
}



impl Exec03
{
	/// Create a new Exec03 from an [RtConfig](crate::RtConfig) configuration.
	//
	pub fn new( config: RtConfig ) -> Self
	{
		match config
		{
			RtConfig::Local =>
			{
				let local   = LocalPool::new();
				let spawner = local.spawner();

				Exec03
				{
					config                                   ,
					local  : Some( RefCell::new( local   ) ) ,
					spawner: Some( RefCell::new( spawner ) ) ,
				}
			}

			#[ cfg( feature = "juliex" ) ]
			//
			RtConfig::Pool => Exec03{ config, local: None, spawner: None },

			_ => unreachable!(),
		}
	}


	/// Getter for the active executor configuration.
	//
	pub fn config( &self ) -> &RtConfig
	{
		&self.config
	}



	/// Run all spawned futures to completion. Note that this does nothing for the threadpool,
	/// but if you are using a local pool, you will need to run this or futures will not be polled.
	/// This blocks the current thread.
	//
	pub fn run( &self )
	{
		match self.config
		{
			RtConfig::Local => self.local.as_ref().unwrap().borrow_mut().run(),

			#[ cfg( feature = "juliex" ) ]
			//
			RtConfig::Pool  => {}, // nothing to be done as juliex polls immediately

			_               => unreachable!(),
		}
	}


	/// Spawn a future to be run on the default executor. Note that this requires the
	/// future to be `Send` in order to work for both the local pool and the threadpool.
	/// When you need to spawn futures that are not `Send` on the local pool, please use
	/// [`spawn_local`](Exec03::spawn_local).
	///
	/// ### Errors
	///
	/// - When using `RtConfig::Pool` (currently juliex), this method is infallible.
	/// - When using `RtConfig::Local` (currently futures 0.3 LocalPool), this method can return a spawn
	/// error if the executor has been shut down. See the [docs for the futures library](https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.16/futures/task/struct.SpawnError.html). I haven't really found a way to trigger this error.
	/// You can call [crate::rt::run] and spawn again afterwards.
	///
	//
	pub fn spawn( &self, fut: impl Future< Output = () > + 'static + Send ) -> Result< (), RtErr >
	{
		match self.config
		{
			RtConfig::Local =>

				self.spawner.as_ref().unwrap().borrow_mut().spawn_local( fut )

			   	.map_err( |_| RtErrKind::Spawn.into() ),


			#[ cfg( feature = "juliex" ) ]
			//
			RtConfig::Pool =>
			{
				juliex_crate::spawn( fut );
				Ok(())
			}

			_ => unreachable!(),
		}
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
	/// - When using `RtConfig::Pool` (currently juliex), this method will return an error of kind [RtErrKind::SpawnLocalOnThreadPool](crate::RtErrKind::SpawnLocalOnThreadPool).
	///   Since the signature doesn't require [Send] on the future, it can never be sent on a threadpool.
	/// - When using `RtConfig::Local` (currently futures 0.3 LocalPool), this method can return a spawn
	/// error if the executor has been shut down. `spawn_local` will return an error of kind
	///  [RtErrKind::Spawn](crate::RtErrKind::Spawn).
	///
	/// See the [docs for the futures library](https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.18/futures/task/struct.SpawnError.html). I haven't really found a way to trigger this error,
	/// since you can call [rt::run](crate::rt::run) and spawn again afterwards.
	//
	pub fn spawn_local( &self, fut: impl Future< Output = () > + 'static  ) -> Result< (), RtErr >
	{
		match self.config
		{
			RtConfig::Local =>

				self.spawner.as_ref().unwrap().borrow_mut().spawn_local( fut )

			   	.map_err( |_| RtErrKind::Spawn.into() ),


			#[ cfg( feature = "juliex" ) ]
			//
			RtConfig::Pool => Err( RtErrKind::SpawnLocalOnThreadPool.into() ),

			_ => unreachable!(),
		}
	}
}
