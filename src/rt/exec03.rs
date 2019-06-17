use crate :: { import::*, Exec03Config, RtErr, RtErrKind };


/// An executor that uses futures 0.3 LocalPool or juliex threadpool under the hood.
//
pub struct Exec03
{
	config : Exec03Config                      ,
	local  : Option<RefCell< LocalPool03    >> ,
	spawner: Option<RefCell< LocalSpawner03 >> ,
}



impl Default for Exec03
{
	fn default() -> Self
	{
		Exec03::new( Exec03Config::default() )
	}
}



impl Exec03
{
	/// Create a new Exec03 from a configuration
	//
	pub fn new( config: Exec03Config ) -> Self
	{
		match &config
		{
			&Exec03Config::Local =>
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

			&Exec03Config::Pool{..} => Exec03{ config, local: None, spawner: None },
		}
	}


	/// Getter for active executor configuration
	//
	pub fn config( &self ) -> &Exec03Config
	{
		&self.config
	}



	/// Run all spawned futures to completion.
	//
	pub fn run( &self )
	{
		match self.config
		{
			Exec03Config::Local    => self.local.as_ref().unwrap().borrow_mut().run(),
			Exec03Config::Pool{..} => {}, // nothing to be done as juliex polls immediately
		}
	}


	/// Spawn a future to be run on the LocalPool (current thread)
	//
	pub fn spawn( &self, fut: impl Future< Output = () > + 'static + Send ) -> Result< (), RtErr >
	{
		match self.config
		{
			Exec03Config::Local =>

				self.spawner.as_ref().unwrap().borrow_mut().spawn_local( fut )

			   	.map_err( |_| RtErrKind::Spawn{ context: "Futures 0.3 LocalPool spawn".into() }.into() ),


			Exec03Config::Pool{..} => Ok( juliex::spawn( fut ) ),
		}
	}


	/// Spawn a future to be run on the LocalPool (current thread)
	//
	pub fn spawn_local( &self, fut: impl Future< Output = () > + 'static  ) -> Result< (), RtErr >
	{
		match self.config
		{
			Exec03Config::Local =>

				self.spawner.as_ref().unwrap().borrow_mut().spawn_local( fut )

			   	.map_err( |_| RtErrKind::Spawn{ context: "Futures 0.3 LocalPool spawn".into() }.into() ),


			Exec03Config::Pool{..} => Err( RtErrKind::Spawn{ context: "Exec03 spawn_local when initialized executor is the threadpool. Use `spawn` to spawn on the threadpool or initialize the default executor for the thread to be the thread local executor".into() }.into() ),
		}
	}
}