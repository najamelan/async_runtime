#![ cfg(not( target_arch = "wasm32" )) ]

// Tested:
// - ✔ current_rt for localpool
// - ✔ current_rt for juliex
// - ✔ current_rt for async_std
// - ✔ double executor init error: Local - Local.
// - ✔ double executor init error: Pool  - Pool.
// - ✔ double executor init error: Local - Pool.
// - ✔ double executor init error: Pool  - Local.
// - ✔ rt::block_on
// - ✔ rt::block_on with a boxed future
// - ✔ spawn* return error when no executor initialized
//
use
{
	async_runtime as rt,

	futures :: { channel::oneshot, future::FutureExt } ,
};



#[ cfg( feature = "localpool" ) ]
//
#[test]
//
fn init_localpool()
{
	assert_eq!( None, rt::current_rt() );

	rt::init( rt::Config::LocalPool ).expect( "no double executor init" );

	assert_eq!( Some( rt::Config::LocalPool ), rt::current_rt() );
}


#[ cfg( feature = "juliex" ) ]
//
#[test]
//
fn init_juliex()
{
	assert_eq!( None, rt::current_rt() );

	rt::init( rt::Config::Juliex ).expect( "no double executor init" );

	assert_eq!( Some( rt::Config::Juliex ), rt::current_rt() );
}


#[ cfg( feature = "async_std" ) ]
//
#[test]
//
fn init_async_std()
{
	assert_eq!( None, rt::current_rt() );

	rt::init( rt::Config::AsyncStd ).expect( "no double executor init" );

	assert_eq!( Some( rt::Config::AsyncStd ), rt::current_rt() );
}



// Trigger DoubleExecutorInit with 2 threadpool executors.
//
#[ cfg( feature = "juliex" ) ] #[test]
//
fn double_init_pool()
{
	             rt::init( rt::Config::Juliex ).expect( "no double executor init" );
	let result = rt::init( rt::Config::Juliex );

	assert_eq!( &rt::ErrorKind::DoubleExecutorInit, result.unwrap_err().kind() );
}



// Trigger DoubleExecutorInit with 2 different executors.
//
#[ cfg(all( feature = "localpool", feature = "juliex" )) ] #[test]
//
fn double_init_different()
{
	             rt::init( rt::Config::LocalPool ).expect( "no double executor init" );
	let result = rt::init( rt::Config::Juliex  );

	assert_eq!( &rt::ErrorKind::DoubleExecutorInit, result.unwrap_err().kind() );
}



// Trigger DoubleExecutorInit with 2 different executors.
//
#[ cfg(all( feature = "localpool", feature = "juliex" )) ] #[test]
//
fn double_init_inverse()
{
	             rt::init( rt::Config::Juliex    ).expect( "no double executor init" );
	let result = rt::init( rt::Config::LocalPool );

	assert_eq!( &rt::ErrorKind::DoubleExecutorInit, result.unwrap_err().kind() );
}



// spawn should error if no executor initialized
//
#[test]
//
fn spawn_without_init()
{
	let result = rt::spawn( async {} );

	assert_eq!( &rt::ErrorKind::NoExecutorInitialized, result.unwrap_err().kind() );
}



// spawn_local should error if no executor initialized
//
#[test]
//
fn spawn_local_without_init()
{
	let result = rt::spawn_local( async {} );

	assert_eq!( &rt::ErrorKind::NoExecutorInitialized, result.unwrap_err().kind() );
}



// spawn_handle should error if no executor initialized
//
#[test]
//
fn spawn_handle_without_init()
{
	let handle = rt::spawn_handle( async {} );

	if let Err(error) = handle
	{
		assert_eq!( &rt::ErrorKind::NoExecutorInitialized, error.kind() );
	}

	else
	{
		panic!( "spawn_handle should return an error when no executor initialized" );
	}
}



// spawn_handle_local should error if no executor initialized
//
#[test]
//
fn spawn_handle_local_without_init()
{
	let handle = rt::spawn_handle_local( async {} );

	if let Err(error) = handle
	{
		assert_eq!( &rt::ErrorKind::NoExecutorInitialized, error.kind() );
	}

	else
	{
		panic!( "spawn_handle should return an error when no executor initialized" );
	}
}



#[test]
//
fn block_on()
{
	let (tx, rx) = oneshot::channel();

	rt::block_on( async { tx.send( 2 ).expect( "send on channel" ); } );

	rt::block_on( async move
	{
		let num: u8 = rx.await.expect( "wait for channel" );
		assert_eq!( 2, num );
	});
}



#[test]
//
fn block_on_boxed()
{
	let (tx, rx) = oneshot::channel();

	rt::block_on( async { tx.send( 2 ).expect( "send on channel" ); }.boxed() );

	rt::block_on( async move
	{
		let num: u8 = rx.await.expect( "wait for channel" );
		assert_eq!( 2, num );
	});
}
