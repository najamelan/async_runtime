#![ feature( async_await ) ]

// Tested:
//
// - ✔ basic spawning using default config
// - ✔ basic spawning specifying the config
// - ✔ spawn a pinned boxed future
// - ✔ spawn several tasks
// - ✔ spawn from within another task
// - ✔ verify that it's actually running on a threadpool and not on a LocalPool


use
{
	async_runtime :: { *                                                       } ,
	std           :: { sync::{ Arc, Mutex }, thread                            } ,
	futures       :: { future::FutureExt, channel::oneshot, executor::block_on } ,
};




#[test]
//
fn basic_spawn()
{
	let number   = Arc::new( Mutex::new( 0 ) );
	let num2     = number.clone();
	let (tx, rx) = oneshot::channel();

	let task = async move
	{
		*num2.lock().expect( "lock mutex" ) = 2;
		tx.send( () ).expect( "send on channel" );
	};

	rt::spawn( task ).expect( "Spawn task" );

	block_on( async move
	{
		rx.await.expect( "wait on channel" );
		assert_eq!( *number.lock().expect( "lock mutex" ), 2 );
	})
}




#[test]
//
fn spawn_config()
{
	let number   = Arc::new( Mutex::new( 0 ) );
	let num2     = number.clone();
	let (tx, rx) = oneshot::channel();

	rt::init( Exec03Config::Pool ).expect( "no double executor init" );

	let task = async move
	{
		*num2.lock().expect( "lock mutex" ) = 3;
		tx.send( () ).expect( "send on channel" );
	};

	rt::spawn( task ).expect( "Spawn task" );

	block_on( async move
	{
		rx.await.expect( "wait on channel" );
		assert_eq!( *number.lock().expect( "lock mutex" ), 3 );
	})
}



#[test]
//
fn spawn_boxed()
{
	let number   = Arc::new( Mutex::new( 0 ) );
	let num2     = number.clone();
	let (tx, rx) = oneshot::channel();

	let task = async move
	{
		*num2.lock().expect( "lock mutex" ) = 5;
		tx.send( () ).expect( "send on channel" );

	}.boxed();

	rt::spawn( task ).expect( "Spawn task" );

	block_on( async move
	{
		rx.await.expect( "wait on channel" );
		assert_eq!( *number.lock().expect( "lock mutex" ), 5 );
	})
}




#[test]
//
fn several()
{
	let number     = Arc::new( Mutex::new( 0 ) );
	let num2       = number.clone();
	let (tx , rx ) = oneshot::channel();
	let (tx2, rx2) = oneshot::channel();

	let task = async move
	{
		*num2.lock().expect( "lock mutex" ) = 4 + rx.await.expect( "channel" );
		tx2.send( () ).expect( "send on channel" );
	};

	let task2 = async move
	{
		tx.send( 2 ).expect( "send on channel" );
	};

	rt::spawn( task  ).expect( "Spawn task"  );
	rt::spawn( task2 ).expect( "Spawn task2" );

	block_on( async move
	{
		rx2.await.expect( "wait on channel" );
		assert_eq!( *number.lock().expect( "lock mutex" ), 6 );
	})
}



#[test]
//
fn within()
{
	let number     = Arc::new( Mutex::new( 0 ) );
	let num2       = number.clone();
	let (tx , rx ) = oneshot::channel();
	let (tx2, rx2) = oneshot::channel();


	let task2 = async move
	{
		tx.send( 3 ).expect( "send on channel" );


		let task = async move
		{
			*num2.lock().expect( "lock mutex" ) = 5 + rx.await.expect( "channel" );
			tx2.send( () ).expect( "send on channel" );
		};


		rt::spawn( task  ).expect( "Spawn task" );
	};

	rt::spawn( task2 ).expect( "Spawn task2" );

	block_on( async move
	{
		rx2.await.expect( "wait on channel" );
		assert_eq!( *number.lock().expect( "lock mutex" ), 8 );
	})
}


#[test]
//
fn not_running_local()
{
	let (tx, rx) = oneshot::channel();

	let task = async move
	{
		tx.send( thread::current().id() ).expect( "send on channel" );
	};

	rt::spawn( task ).expect( "Spawn task" );

	block_on( async move
	{
		let tid = rx.await.expect( "wait on channel" );
		assert_ne!( thread::current().id(), tid );
	})
}



