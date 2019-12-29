#![ cfg(not( target_arch = "wasm32"     )) ]
#![ cfg(     feature     = "threadpool"  ) ]

// Tested:
//
// ✔ basic spawning
// ✔ spawn a pinned boxed future
// ✔ spawn several tasks
// ✔ spawn from within another task
// ✔ worker thread spawns on same threadpool
// ✔ worker thread spawns on same threadpool from spawn_handle
// ✔ it's actually running on a threadpool and not on a LocalPool
// ✔ spawn_local returns the right error
// ✔ spawn_handle returns the right value
// ✔ spawn_handle_local returns an error
//
use
{
	async_runtime as rt,

	std     :: { thread                              } ,
	futures :: { future::FutureExt, channel::oneshot } ,
};



#[test]
//
fn basic_spawn()
{
	rt::init( rt::Config::ThreadPool ).expect( "no double executor init" );

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
fn spawn_boxed()
{
	rt::init( rt::Config::ThreadPool ).expect( "no double executor init" );

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
	rt::init( rt::Config::ThreadPool ).expect( "no double executor init" );

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
	rt::init( rt::Config::ThreadPool ).expect( "no double executor init" );

	let (tx , rx ) = oneshot::channel();
	let (tx2, rx2) = oneshot::channel();


	let task2 = async move
	{
		tx.send( 3 ).expect( "send on channel" );


		let task = async move
		{
			assert_eq!( rt::Config::ThreadPool, rt::current_rt().expect( "some executor" ) );

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
	rt::init( rt::Config::ThreadPool ).expect( "no double executor init" );

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




// Trigger SpawnLocalOnThreadPool.
//
#[test]
//
fn spawn_local_on_thread_pool()
{
	rt::init( rt::Config::ThreadPool ).expect( "no double executor init" );

	let res = rt::spawn_local( async {} );

	assert_eq!( &rt::ErrorKind::SpawnLocalOnThreadPool, res.unwrap_err().kind() );
}






// Spawn_handle, return string.
//
#[test]
//
fn spawn_handle()
{
	rt::init( rt::Config::ThreadPool ).expect( "no double executor init" );

	let handle = rt::spawn_handle( async { "hello".to_string() } ).expect( "spawn_handle" );

	rt::block_on( async { assert_eq!( "hello", &handle.await ); } );
}



// Verify that nested calls to spawn spawn on the right executor
//
#[test]
//
fn spawn_handle_spawn_on_self()
{
	rt::init( rt::Config::ThreadPool ).expect( "no double executor init" );

	let handle = rt::spawn_handle( async
	{
		rt::spawn( async
		{
			assert_eq!( rt::Config::ThreadPool, rt::current_rt().expect( "some executor" ) );

		}).expect( "spawn" );

		"hello".to_string()

	}).expect( "spawn_handle" );

	rt::block_on( async { assert_eq!( "hello", &handle.await ); } );
}



// Verify that we can spawn !Send futures.
//
#[test]
//
fn spawn_handle_local()
{
	rt::init( rt::Config::ThreadPool ).expect( "no double executor init" );

	let handle = rt::spawn_handle_local( async {});

	if let Err(error) = handle
	{
		assert_eq!( &rt::ErrorKind::SpawnLocalOnThreadPool, error.kind() );
	}

	else
	{
		panic!( "spawn_handle_local should return an error on threadpools" );
	}
}



