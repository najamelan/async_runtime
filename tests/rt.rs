#![ cfg(not( target_arch = "wasm32" )) ]

// Tested:
// - ✔ Verify a default config get's chosen when no features are manually enabled
// - ✔ current_rt for localpool
// - ✔ current_rt for threadpool
// - ✔ current_rt for after spawning it should be a threadpool
// - ✔ rt::block_on
// - ✔ rt::block_on with a boxed future
// - basic methods like spawn and init are being tested in the other integration test files


use
{
	async_runtime :: { *                                   } ,
	futures       :: { channel::oneshot, future::FutureExt } ,
};




// Verifies that a default executor is chosen when no features are enabled.
//
#[ cfg(not( feature = "juliex" ))]
//
#[test]
//
fn default_config()
{
	assert!( rt::current_rt().is_none()  );

	rt::spawn( async {} ).expect( "spawn" );

	assert_eq!( Some( rt::Config::LocalPool ), rt::current_rt() );
}



#[test]
//
fn localpool()
{
	assert_eq!( None, rt::current_rt() );

	rt::init( rt::Config::LocalPool ).expect( "no double executor init" );

	assert_eq!( Some( rt::Config::LocalPool ), rt::current_rt() );
}


#[ cfg( feature = "juliex" ) ]
//
#[test]
//
fn thread_pool()
{
	assert_eq!( None, rt::current_rt() );

	rt::init( rt::Config::Juliex ).expect( "no double executor init" );

	assert_eq!( Some( rt::Config::Juliex ), rt::current_rt() );
}



#[ cfg( feature = "juliex" ) ]
//
#[test]
//
fn spawn()
{
	assert_eq!( None, rt::current_rt() );

	rt::spawn( async {} ).expect( "spawn" );

	assert_eq!( Some( rt::Config::Juliex ), rt::current_rt() );
}



#[test]
//
fn block_on()
{
	let (tx, rx) = oneshot::channel();

	rt::block_on( async { tx.send( 2 ).expect( "send on channel" ); } );

	rt::block_on( async move
	{
		let num: u8 = rx.await.expect( "wait for channel" );
		assert_eq!( 2, num );
	});
}



#[test]
//
fn block_on_boxed()
{
	let (tx, rx) = oneshot::channel();

	rt::block_on( async { tx.send( 2 ).expect( "send on channel" ); }.boxed() );

	rt::block_on( async move
	{
		let num: u8 = rx.await.expect( "wait for channel" );
		assert_eq!( 2, num );
	});
}
