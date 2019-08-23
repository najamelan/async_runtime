#![ cfg(all( not(target_arch = "wasm32"), feature = "macros" )) ]


// Tested:
//
// - ✔ set an attribute for local pool and use spawn
// - ✔ set an attribute for thead pool and use spawn

use
{
	std :: { path::Path, process::Command  } ,
};


#[ cfg(any( feature = "localpool", feature = "juliex" )) ]
//
use
{
	async_runtime as rt,

	futures :: { FutureExt } ,
};


#[ cfg( feature = "localpool" ) ]
//
use
{
	std :: { rc::Rc, cell::RefCell } ,
};


#[ cfg( feature = "juliex" ) ]
//
use std::sync::{ Arc, Mutex };



// Spawn local in a test
// RefCell being not Send, this guarantees that it's running on the local thread
//
#[ cfg( feature = "localpool" ) ]
//
#[ rt::localpool ] #[test]
//
async fn async_test()
{
	let number  = Rc::new( RefCell::new( 0u8 ) );
	let num2    = number.clone();

	let (fut, handle) = async move { *num2.borrow_mut() = 2; }.remote_handle();

	rt::spawn_local( fut ).expect( "spawn" );

	handle.await;

	assert_eq!( *number.borrow(), 2 );
}



// call an async method from a sync method
//
#[ cfg( feature = "localpool" ) ]
//
#[test]
//
fn call_async()
{
	assert_eq!( &hello_world(), "You succesfully spawned a future" );
}


#[ cfg( feature = "localpool" ) ]
//
#[ rt::localpool ]
//
async fn hello_world() -> String
{
	format!( "You succesfully spawned a future" )
}


// Spawn on threadpool in a test
//
#[ cfg( feature = "juliex" ) ]
//
#[ rt::juliex ] #[test]
//
async fn attr_on_threadpool()
{
	let number = Arc::new( Mutex::new( 0u8 ) );
	let num2   = number.clone();

	let (fut, handle) = async move { *num2.lock().expect( "lock mutex" ) = 2; }.remote_handle();

	rt::spawn( fut ).expect( "spawn" );

	handle.await;

	assert_eq!( *number.lock().expect( "lock mutex" ), 2 );
}



// Spawn local on main fn
// return result from main
#[test]
//
fn attr_on_main_local()
{
	let path = Path::new( "tests/attributes" );

	let status =

		Command::new( "cargo" )

			.args( vec![ "run", "--bin", "local", "true" ] )
			.current_dir( path )
			.status()
			.expect( "Failed to execute command" )
	;

	assert!( status.success() );


	// should fail
	//
	let status =

		Command::new( "cargo" )

			.args( vec![ "run", "--bin", "local", "false" ] )
			.current_dir( path )
			.status()
			.expect( "Failed to execute command" )
	;

	assert!( !status.success() );
}



// Spawn pool on main fn
// return result from main
//
#[test]
//
fn attr_on_main_pool()
{
	let path = Path::new( "tests/attributes" );

	let status =

		Command::new( "cargo" )

			.args( vec![ "run", "--bin", "pool", "true" ] )
			.current_dir( path )
			.status()
			.expect( "Failed to execute command" )
	;

	assert!( status.success() );


	// should fail
	//
	let status =

		Command::new( "cargo" )

			.args( vec![ "run", "--bin", "local", "false" ] )
			.current_dir( path )
			.status()
			.expect( "Failed to execute command" )
	;

	assert!( !status.success() );
}

