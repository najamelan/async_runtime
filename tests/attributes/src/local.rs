//! The purpose of this test is to verify that:
//!
//! ✔ the code compiles
//! ✔ we can spawn tasks
//! ✔ spawned tasks are actually on a single thread
//! ✔ we can await in main
//! ✔ we can return a Result from main
//
#![ feature( optin_builtin_traits ) ]

use
{
	async_runtime as rt,

	std             :: { env       } ,
	futures::future :: { FutureExt } ,
};


// This is guaranteed not to be sent accross threads
//
struct WontMove(u8);

impl !Send for WontMove {}
impl !Sync for WontMove {}


async fn substract( w: &mut WontMove )
{
	w.0 -= 1;
}



#[ rt::localpool ]
//
async fn main() -> Result< (), () >
{
	let args: Vec<String> = env::args().collect();

	// Whether to return Ok or Err from main
	//
	let ok: bool = args[1].parse().expect( "true of false" );

	let mut x = WontMove( 1 );

	let (spawned, handle) = async move
	{
		substract( &mut x ).await;

		x.0

	}.remote_handle();

	rt::spawn_local( spawned ).expect( "spawn" );

	let num = handle.await;

	assert_eq!( num, 0 );

	if   ok { Ok (()) }
	else    { Err(()) }
}
