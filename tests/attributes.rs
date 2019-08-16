#![ cfg(not( target_arch = "wasm32" )) ]
#![ feature( async_await ) ]


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
	async_runtime :: { *                     } ,
	std           :: { rc::Rc, cell::RefCell, sync::{ Arc, Mutex } } ,
	futures       :: { FutureExt             } ,
};



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



// RefCell being not Send, this guarantees that it's running on the local thread
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

