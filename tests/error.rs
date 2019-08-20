#![ cfg(not( target_arch = "wasm32" )) ]

// Tested:
//
// - ✔ shut down local pool before spawning does not generate error.
// - ✔ double executor init error: Local - Local.
// - ✔ double executor init error: Pool  - Pool.
// - ✔ double executor init error: Local - Pool.
// - ✔ double executor init error: Pool  - Local.


use
{
	async_runtime :: { *             } ,
	futures       :: { future::ready } ,
};




// This is an attempt to trigger the shutdown error on the localpool from the futures lib, but run does not
// shut it down, so I don't know how to trigger this error.
//
#[test]
//
fn shutdown()
{
	rt::init( rt::Config::LocalPool ).expect( "no double executor init" );
	rt::run();

	let result = rt::spawn( ready(()) );

	assert!( result.is_ok() );

	rt::run();
}



// Trigger DoubleExecutorInit with 2 threadpool executors.
//
#[ cfg( feature = "localpool" ) ] #[test]
//
fn double_init_local()
{
	             rt::init( rt::Config::LocalPool ).expect( "no double executor init" );
	let result = rt::init( rt::Config::LocalPool );

	assert_eq!( &RtErrKind::DoubleExecutorInit, result.unwrap_err().kind() );
}



// Trigger DoubleExecutorInit with 2 threadpool executors.
//
#[ cfg( feature = "juliex" ) ] #[test]
//
fn double_init_pool()
{
	             rt::init( rt::Config::Juliex ).expect( "no double executor init" );
	let result = rt::init( rt::Config::Juliex );

	assert_eq!( &RtErrKind::DoubleExecutorInit, result.unwrap_err().kind() );
}



// Trigger DoubleExecutorInit with 2 different executors.
//
#[ cfg(all( feature = "localpool", feature = "juliex" )) ] #[test]
//
fn double_init_different()
{
	             rt::init( rt::Config::LocalPool ).expect( "no double executor init" );
	let result = rt::init( rt::Config::Juliex  );

	assert_eq!( &RtErrKind::DoubleExecutorInit, result.unwrap_err().kind() );
}



// Trigger DoubleExecutorInit with 2 different executors.
//
#[ cfg(all( feature = "localpool", feature = "juliex" )) ] #[test]
//
fn double_init_inverse()
{
	             rt::init( rt::Config::Juliex    ).expect( "no double executor init" );
	let result = rt::init( rt::Config::LocalPool );

	assert_eq!( &RtErrKind::DoubleExecutorInit, result.unwrap_err().kind() );
}



// Trigger SpawnLocalOnThreadPool.
//
#[ cfg( feature = "juliex" ) ] #[test]
//
fn spawn_local_on_thread_pool()
{
	rt::init( rt::Config::Juliex ).expect( "no double executor init" );

	let res = rt::spawn_local( async {} );

	assert_eq!( &RtErrKind::SpawnLocalOnThreadPool, res.unwrap_err().kind() );
}
