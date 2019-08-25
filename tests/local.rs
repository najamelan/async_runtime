#![ cfg(not( target_arch = "wasm32"   )) ]
#![ cfg(     feature     = "localpool" ) ]


// Tested:
//
// ✔ basic spawning
// ✔ spawn !Send task (RefCell is !Send)
// ✔ spawn a pinned boxed future
// ✔ spawn a pinned boxed_local future
// ✔ spawn several tasks
// ✔ spawn from within another task
// ✔ localpools on several threads
// ✔ spawn_handle returns the right value
// ✔ spawn_handle_local returns the right value and can spawn !Send futures
// ✔ rt::localpool::run should error if no executor initialized
// ✔ rt::localpool::run should error if the wrong executor is initialized

//
use
{
	async_runtime as rt,

	std     :: { rc::Rc, cell::RefCell, sync::{ Arc, Mutex }, thread } ,
	futures :: { future::FutureExt, channel::oneshot                 } ,
};



// RefCell being not Send, this guarantees that it's running on the local thread
//
#[test]
//
fn basic_spawn()
{
	let number  = Rc::new( RefCell::new( 0 ) );
	let num2    = number.clone();

	rt::init( rt::Config::LocalPool ).expect( "no double executor init" );

	let task = async move
	{
		*num2.borrow_mut() = 2;
	};

	rt::spawn_local( task ).expect( "Spawn task" );
	rt::localpool::run().unwrap();

	assert_eq!( *number.borrow(), 2 );
}



#[test]
//
fn spawn_boxedlocal()
{
	let (tx, rx) = oneshot::channel();

	rt::init( rt::Config::LocalPool ).expect( "no double executor init" );


	rt::spawn_local( async move
	{
		tx.send( 4 ).expect( "send on channel" );

	}.boxed_local() ).expect( "Spawn task" );


	rt::spawn( async move
	{
		assert_eq!( 4, rx.await.expect( "wait for channel" ) );

	}).expect( "spawn assert" );


	rt::localpool::run().unwrap();
}




#[test]
//
fn spawn_boxed()
{
	let number  = Arc::new( Mutex::new( 0 ) );
	let num2    = number.clone();

	rt::init( rt::Config::LocalPool ).expect( "no double executor init" );

	let task = async move
	{
		*num2.lock().expect( "lock mutex" ) = 5;

	}.boxed();

	// Here we don't use `spawn_local` because this future is actually Send, so we don't need to,
	// yet it will still be spawned on the LocalPool and not on a threadpool.
	//
	rt::spawn( task ).expect( "Spawn task" );
	rt::localpool::run().unwrap();

	assert_eq!( *number.lock().expect( "lock mutex" ), 5 );
}




#[test]
//
fn several()
{
	let number   = Rc::new( RefCell::new( 0 ) );
	let num2     = number.clone();
	let (tx, rx) = oneshot::channel();

	rt::init( rt::Config::LocalPool ).expect( "no double executor init" );

	let task = async move
	{
		*num2.borrow_mut() = 4 + rx.await.expect( "channel" );
	};

	let task2 = async move
	{
		tx.send( 2 ).expect( "send on channel" );
	};

	rt::spawn_local( task  ).expect( "Spawn task"  );

	// Here we don't use `spawn_local` because this future is actually Send, so we don't need to,
	// yet it will still be spawned on the LocalPool and not on a threadpool.
	//
	rt::spawn( task2 ).expect( "Spawn task2" );

	rt::localpool::run().unwrap();

	assert_eq!( *number.borrow(), 6 );
}



#[test]
//
fn within()
{
	let number   = Rc::new( RefCell::new( 0 ) );
	let num2     = number.clone();
	let (tx, rx) = oneshot::channel();

	rt::init( rt::Config::LocalPool ).expect( "no double executor init" );

	let task2 = async move
	{
		tx.send( 3 ).expect( "send on channel" );


		let task = async move
		{
			*num2.borrow_mut() = 5 + rx.await.expect( "channel" );
		};


		rt::spawn_local( task  ).expect( "Spawn task"  );
	};

	rt::spawn_local( task2 ).expect( "Spawn task2" );
	rt::localpool::run().unwrap();

	assert_eq!( *number.borrow(), 8 );
}



// Each thread has it's own local executor
//
#[test]
//
fn threads()
{
	let number   = Rc::new( RefCell::new( 0 ) );
	let num2     = number.clone();
	let (tx, rx) = oneshot::channel();

	rt::init( rt::Config::LocalPool ).expect( "no double executor init" );

	let task = async move
	{
		*num2.borrow_mut() = 6 + rx.await.expect( "channel" );
	};

	thread::spawn( move ||
	{
		rt::init( rt::Config::LocalPool ).expect( "no double executor init" );

		let task2 = async move
		{
			tx.send( 4 ).expect( "send on channel" );
		};

		// Here we don't use `spawn_local` because this future is actually Send, so we don't need to,
		// yet it will still be spawned on the LocalPool and not on a threadpool.
		//
		rt::spawn( task2 ).expect( "Spawn thread 2 program" );
		rt::localpool::run().unwrap();

	}).join().expect( "join thread" );


	rt::spawn_local( task  ).expect( "Spawn task"  );
	rt::localpool::run().unwrap();

	assert_eq!( *number.borrow(), 10 );
}



// Spawn_handle, return string.
//
#[test]
//
fn spawn_handle()
{
	rt::init( rt::Config::LocalPool ).expect( "no double executor init" );

	let handle = rt::spawn_handle( async { "hello".to_string() } ).expect( "spawn_handle" );

	rt::localpool::run().expect( "run localpool" );

	rt::block_on( async { assert_eq!( "hello", &handle.await ); } );
}



// Verify that we can spawn !Send futures.
//
#[test]
//
fn spawn_handle_local()
{
	rt::init( rt::Config::LocalPool ).expect( "no double executor init" );

	let handle = rt::spawn_handle_local( async
	{
		let rc = Rc::new( "some string" );

		async { 3+3 }.await;

		let _rc2 = rc.clone();

		"hello".to_string()

	}).expect( "spawn_handle" );

	rt::localpool::run().expect( "run localpool" );

	rt::block_on( async { assert_eq!( "hello", &handle.await ); } );
}



// rt::localpool::run should error if no executor initialized
//
#[test]
//
fn run_without_init()
{
	let result = rt::localpool::run();

	assert_eq!( &rt::ErrorKind::NoExecutorInitialized, result.unwrap_err().kind() );
}



// This is how the spawn error can be triggered on Localpool
//
// #[test]
// //
// fn break_localpool()
// {
// 	use futures::executor::LocalPool;
// 	use futures::task::SpawnExt;

// 	let     pool  = LocalPool::new();
// 	let mut spawn = pool.spawner();

// 	drop(pool);

// 	spawn.spawn( async {} ).expect( "boom" );
// }




