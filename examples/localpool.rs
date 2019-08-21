//! In this example we make a future that is not Send. We then spawn that future on a LocalPool executor.
//!
//! we show that you can:
//! - use a macro attribute to set up the executor for the thread
//! - spawn from main
//! - await in main
//! - spawn !Send futures
//!
//! run with `cargo run --example localpool --features notwasm localpool`
//
use
{
	async_runtime :: { *                     } ,
	std           :: { rc::Rc, cell::RefCell } ,
	futures       :: { future::FutureExt     } ,
};


#[ rt::localpool ]
//
async fn main()
{
	// Rc and RefCell are not Send
	//
	let number  = Rc::new( RefCell::new( 0 ) );
	let num2    = number.clone();

	// We use remote_handle here so that even though the task is spawend and must return `()`
	// we can still await it's result. In this case we don't have anything useful to return,
	// but you can actually return values like this. Here we just make sure that the assert
	// below only runs after the spawned task has updated number.
	//
	let (task, handle) = async move
	{
		*num2.borrow_mut() = 2;

	}.remote_handle();

	// If we initialized the localpool, we normally could just use the method `spawn`, but since that's
	// the same method used for the threadpool, it requires Send. Thus as long as your future is `Send`, you can
	// use the `spawn` method, even on a localpool. Here our future isn't Send, so we have to use local_spawn.
	//
	// `local_spawn` will return an error at runtime if the initialized executor is a threadpool.
	//
	rt::spawn_local( task ).expect( "Spawn task" );

	handle.await;

	let result = *number.borrow();
	dbg!( result );
	assert_eq!( result, 2 );
}


