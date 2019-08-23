#![ feature( duration_constants ) ]

//! In this example we run a bunch of tasks in parallel. To verify that they run on different threads
//! we make them all sleep for a second and measure the time passed when they finish.
//!
//! run with: `cargo run --example juliex --features macros juliex`
//
use
{
	async_runtime as rt,

	std           :: { time::{ Duration, Instant }, thread::sleep } ,
	futures       :: { future::{ FutureExt, join_all }            } ,
};

#[ rt::juliex ]
//
async fn main()
{
	let     start = Instant::now();
	let mut tasks = Vec::new();

	for i in 0..8
	{
		// There isn't currently a convenient way to run tasks on a threadpool until all tasks have
		// finished, or until some shutdown signal is given.
		//
		// This is one of the ways tasks can synchronize and wait on eachother. Another way is to wait
		// on channels.
		//
		let (fut, handle) = async move
		{
			sleep( Duration::SECOND );

			println!( "Time elapsed at task {} end: {} second(s).", i, start.elapsed().as_secs() );

		}.remote_handle();

		rt::spawn( fut ).expect( "spawn task" );
		tasks.push( handle );
	}

	join_all( tasks ).await;
}
