use crate :: { import::*, Exec03Config, RtErr, RtErrKind };


/// An executor that uses futures 0.3 executor under the hood.
///
/// TODO: threadpool impl. Currently puts everything on LocalPool.
//
pub struct Exec03
{
	config : Exec03Config                      ,
	local  : Option<RefCell< LocalPool03    >> ,
	spawner: Option<RefCell< LocalSpawner03 >> ,
	// _pool   : ThreadPool03            ,
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
					config : config,
					local  : Some( RefCell::new( local ) ),
					spawner: Some( RefCell::new( spawner ) ),
					// _pool   : ThreadPool03::new().expect( "Create futures::ThreadPool with default configurtion" ),
				}
			}

			&Exec03Config::Pool{..} => unimplemented!(),
		}
	}


	/// Run all spawned futures to completion.
	//
	pub fn run( &self )
	{
		match self.config
		{
			Exec03Config::Local    => self.local.as_ref().unwrap().borrow_mut().run(),
			Exec03Config::Pool{..} => unimplemented!(),
		}
	}


	/// Spawn a future to be run on the LocalPool (current thread)
	//
	pub fn spawn( &self, fut: impl Future< Output = () > + 'static ) -> Result< (), RtErr >
	{
		match self.config
		{
			Exec03Config::Local =>

				self.spawner.as_ref().unwrap().borrow_mut().spawn_local( fut )

			   	.map_err( |_| RtErrKind::Spawn{ context: "Exec03 spawn".into() }.into() ),


			Exec03Config::Pool{..} => unimplemented!(),
		}
	}
}
