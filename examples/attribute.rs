//! This example shows how to make an async main function.
//!
//! run with `cargo run --example attribute --features macros localpool`
//
use async_runtime::*;


async fn hello()
{
	println!( "Asynchronous hello world!" );
}


// You can create an async main fn like this. You can also choose `thread_pool` over `local`
//
#[ rt::localpool ]
//
async fn main()
{
	// We can await directly in main
	//
	hello().await;
}
