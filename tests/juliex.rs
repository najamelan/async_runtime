#![ cfg(not( target_arch = "wasm32" )) ]
#![ cfg(     feature     = "juliex"  ) ]

// Tested:
//
// ✔ basic spawning using default config
// ✔ basic spawning specifying the config
// ✔ spawn a pinned boxed future
// ✔ spawn several tasks
// ✔ spawn from within another task
// ✔ verify that it's actually running on a threadpool and not on a LocalPool


use
{
	async_runtime as rt,

	std           :: { thread                              } ,
	futures       :: { future::FutureExt, channel::oneshot } ,
};




#[test]
//
fn basic_spawn()
{
	rt::init( rt::Config::Juliex ).expect( "no double executor init" );

	let (tx, rx) = oneshot::channel();

	let task = async move
	{
		tx.send( 2u8 ).expect( "send on channel" );
	};

	rt::spawn( task ).expect( "Spawn task" );

	dbg!( std::thread::current().id(), rt::current_rt() );

	rt::block_on( async move
	{
		assert_eq!( 2u8, rx.await.expect( "wait on channel" ) );

	});
}




#[test]
//
fn spawn_config()
{
	let (tx, rx) = oneshot::channel();

	rt::init( rt::Config::Juliex ).expect( "no double executor init" );

	let task = async move
	{
		tx.send( 3u8 ).expect( "send on channel" );
	};

	rt::spawn( task ).expect( "Spawn task" );

	rt::block_on( async move
	{
		assert_eq!( 3u8, rx.await.expect( "wait on channel" ) );
	});
	dbg!( std::thread::current().id(), rt::current_rt() );

}



#[test]
//
fn spawn_boxed()
{
	rt::init( rt::Config::Juliex ).expect( "no double executor init" );

	let (tx, rx) = oneshot::channel();

	let task = async move
	{
		tx.send( 5 ).expect( "send on channel" );

	}.boxed();

	rt::spawn( task ).expect( "Spawn task" );

	rt::block_on( async move
	{
		assert_eq!( 5, rx.await.expect( "wait on channel" ) );
	})
}




#[test]
//
fn several()
{
	rt::init( rt::Config::Juliex ).expect( "no double executor init" );

	let (tx , rx ) = oneshot::channel();
	let (tx2, rx2) = oneshot::channel();

	let task = async move
	{
		tx2.send( 4 + rx.await.expect( "channel" ) ).expect( "send on channel" );
	};

	let task2 = async move
	{
		tx.send( 2 ).expect( "send on channel" );
	};

	rt::spawn( task  ).expect( "Spawn task"  );
	rt::spawn( task2 ).expect( "Spawn task2" );

	rt::block_on( async move
	{
		assert_eq!( 6, rx2.await.expect( "wait on channel" ) );
	})
}



#[test]
//
fn within()
{
	rt::init( rt::Config::Juliex ).expect( "no double executor init" );

	let (tx , rx ) = oneshot::channel();
	let (tx2, rx2) = oneshot::channel();


	let task2 = async move
	{
		tx.send( 3 ).expect( "send on channel" );


		let task = async move
		{
			tx2.send( 5 + rx.await.expect( "channel" ) ).expect( "send on channel" );
		};


		rt::spawn( task  ).expect( "Spawn task" );
	};

	rt::spawn( task2 ).expect( "Spawn task2" );

	rt::block_on( async move
	{
		assert_eq!( 8, rx2.await.expect( "wait on channel" ) );
	})
}


#[test]
//
fn not_running_local()
{
	rt::init( rt::Config::Juliex ).expect( "no double executor init" );

	let (tx, rx) = oneshot::channel();

	let task = async move
	{
		tx.send( thread::current().id() ).expect( "send on channel" );
	};

	rt::spawn( task ).expect( "Spawn task" );

	rt::block_on( async move
	{
		assert_ne!( thread::current().id(), rx.await.expect( "wait on channel" ) );
	})
}



