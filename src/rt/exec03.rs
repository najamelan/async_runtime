use crate :: { import::*, RtConfig, RtErr, RtErrKind };


/// An executor that uses [futures 0.3 LocalPool](https://rust-lang-nursery.github.io/futures-api-docs/0.3.0-alpha.16/futures/executor/struct.LocalPool.html) or [juliex](https://docs.rs/juliex) threadpool under the hood.
/// Normally you don't need to construct this yourself, just use the [`rt`](crate::rt) module methods to spawn futures.
//
#[ derive( Debug ) ]
//
pub struct Exec03
{
	config : RtConfig                          ,
	local  : Option<RefCell< LocalPool03    >> ,
	spawner: Option<RefCell< LocalSpawner03 >> ,
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
		match &config
		{
			&RtConfig::Local =>
			{
				let local   = LocalPool03 ::new();
				let spawner = local.spawner();

				Exec03
				{
					config                                   ,
					local  : Some( RefCell::new( local   ) ) ,
					spawner: Some( RefCell::new( spawner ) ) ,
				}
			}

			&RtConfig::Pool{..} => Exec03{ config, local: None, spawner: None },
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
			RtConfig::Local    => self.local.as_ref().unwrap().borrow_mut().run(),
			RtConfig::Pool{..} => {}, // nothing to be done as juliex polls immediately
		}
	}


	/// Spawn a future to be run on the default executor. Note that this requires the
	/// future to be `Send` in order to work for both the local pool and the threadpool.
	/// When you need to spawn futures that are not `Send` on the local pool, please use
	/// [`spawn_local`](Exec03::spawn_local).
	///
	// TODO: should we include the spawnerror from futures as a cause in RtErrKind?
	//
	pub fn spawn( &self, fut: impl Future< Output = () > + 'static + Send ) -> Result< (), RtErr >
	{
		match self.config
		{
			RtConfig::Local =>

				self.spawner.as_ref().unwrap().borrow_mut().spawn_local( fut )

			   	.map_err( |_| RtErrKind::Spawn{ context: "Futures 0.3 LocalPool spawn".into() }.into() ),



			RtConfig::Pool =>
			{
				#[ cfg( feature = "juliex" ) ]
				//
				return Ok( juliex::spawn( fut ) );

				#[ cfg( not( feature = "juliex" ) ) ]
				//
				return Err( RtErrKind::Spawn{ context: "async_runtime was compiled without the juliex feature".into() }.into() );
			}
		}
	}


	/// Spawn a `!Send` future to be run on the LocalPool (current thread). Note that the executor must
	/// be created with a local pool configuration. This will err if you try to call this on an executor
	/// set up with a threadpool.
	//
	pub fn spawn_local( &self, fut: impl Future< Output = () > + 'static  ) -> Result< (), RtErr >
	{
		match self.config
		{
			RtConfig::Local =>

				self.spawner.as_ref().unwrap().borrow_mut().spawn_local( fut )

			   	.map_err( |_| RtErrKind::Spawn{ context: "Futures 0.3 LocalPool spawn".into() }.into() ),


			RtConfig::Pool{..} => Err( RtErrKind::Spawn{ context: "Exec03 spawn_local when initialized executor is the threadpool. Use `spawn` to spawn on the threadpool or initialize the default executor for the thread to be the thread local executor".into() }.into() ),
		}
	}
}
