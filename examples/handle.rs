//! In this example we make a future that is not Send. We then spawn that future on a LocalPool executor.
//!
//! we show that you can:
//! - use a macro attribute to set up the executor for the thread
//! - spawn from main
//! - await in main
//! - spawn !Send futures
//!
//! run with `cargo run --example localpool --features macros localpool`
//
use
{
	async_runtime as rt,

	futures :: { future::FutureExt, channel::oneshot } ,
};


#[ rt::localpool ]
//
async fn main()
{
	// Rc and RefCell are !Send
	//
	let (tx, rx) = oneshot::channel();


	let (task, handle) = async move
	{
		let num = rx.await;
		panic!( num );

	}.remote_handle();


	rt::spawn_local( task ).expect( "Spawn task" );

	// drop( handle );

	tx.send( 2 ).expect( "send on channel");
	handle.await
}


