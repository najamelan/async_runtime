#![ cfg(not( target_arch = "wasm32" )) ]
#![ feature( async_await ) ]


// Tested:
//
// - ✔ set an attribute for local pool and use spawn
// - ✔ set an attribute for thead pool and use spawn


use
{
	async_runtime :: { *                     } ,
	std           :: { rc::Rc, cell::RefCell, sync::{ Arc, Mutex }, process::Command, path::Path } ,
	futures       :: { FutureExt             } ,
};



// Spawn local in a test
// RefCell being not Send, this guarantees that it's running on the local thread
//
#[ rt::local ] #[test]
//
async fn async_test()
{
	let number  = Rc::new( RefCell::new( 0 ) );
	let num2    = number.clone();

	let (fut, handle) = async move { *num2.borrow_mut() = 2; }.remote_handle();

	rt::spawn_local( fut ).expect( "spawn" );

	handle.await;

	assert_eq!( *number.borrow(), 2 );
}



// Spawn on threadpool in a test
//
#[ rt::thread_pool ] #[test]
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

