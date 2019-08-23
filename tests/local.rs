#![ cfg(not( target_arch = "wasm32"   )) ]
#![ cfg(     feature     = "localpool" ) ]


// Tested:
//
// - ✔ basic spawning
// - ✔ spawn !Send task (RefCell is !Send)
// - ✔ spawn a pinned boxed future
// - ✔ spawn a pinned boxed_local future
// - ✔ spawn several tasks
// - ✔ spawn from within another task
// - ✔ localpools on several threads


use
{
	async_runtime :: { *                                                   } ,
	std           :: { rc::Rc, cell::RefCell, sync::{ Arc, Mutex }, thread } ,
	futures       :: { future::FutureExt, channel::oneshot                 } ,
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

