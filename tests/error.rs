#![ cfg(not( target_arch = "wasm32" )) ]

// Tested:
//
// - ✔ shut down local pool before spawning does not generate error.


#[ cfg( feature = "localpool" ) ]
//
use futures::future::ready;

#[ cfg(any( feature = "juliex", feature = "localpool" )) ]
//
use async_runtime:: { self as rt, ErrorKind };




// This is an attempt to trigger the shutdown error on the localpool from the futures lib, but run does not
// shut it down, so I don't know how to trigger this error.
//
#[ cfg( feature = "localpool" ) ]
//
#[test]
//
fn shutdown()
{
	rt::init( rt::Config::LocalPool ).expect( "no double executor init" );
	rt::localpool::run().unwrap();

	let result = rt::spawn( ready(()) );

	assert!( result.is_ok() );

	rt::localpool::run().unwrap();
}



// Trigger DoubleExecutorInit with 2 threadpool executors.
//
#[ cfg( feature = "localpool" ) ] #[test]
//
fn double_init_local()
{
	             rt::init( rt::Config::LocalPool ).expect( "no double executor init" );
	let result = rt::init( rt::Config::LocalPool );

	assert_eq!( &ErrorKind::DoubleExecutorInit, result.unwrap_err().kind() );
}




// Trigger SpawnLocalOnThreadPool.
//
#[ cfg( feature = "juliex" ) ] #[test]
//
fn spawn_local_on_thread_pool()
{
	rt::init( rt::Config::Juliex ).expect( "no double executor init" );

	let res = rt::spawn_local( async {} );

	assert_eq!( &ErrorKind::SpawnLocalOnThreadPool, res.unwrap_err().kind() );
}
