//! This example shows how to make an async main function.
//
#![ feature( async_await ) ]


use async_runtime::*;


async fn hello()
{
	println!( "Asynchronous hello world!" );
}


// You can create an async main fn like this. You can also choose `thread_pool` over `local`
//
#[ rt::local ]
//
async fn main()
{
	// We can await directly in main
	//
	hello().await;
}
