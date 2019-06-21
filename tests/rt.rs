#![ cfg(not( target_arch = "wasm32" )) ]
#![ feature( async_await ) ]

// Tested:
//
// - ✔ current_rt for localpool
// - ✔ current_rt for threadpool
// - ✔ current_rt for after spawning it should be a threadpool
// - ✔ rt::block_on
// - ✔ rt::block_on with a boxed future
// - basic methods like spawn and init are being tested in the other integration test files


use
{
	async_runtime :: { *                } ,
	futures       :: { channel::oneshot, future::FutureExt } ,
};



#[test]
//
fn localpool()
{
	assert_eq!( None, rt::current_rt() );

	rt::init( RtConfig::Local ).expect( "no double executor init" );

	assert_eq!( Some( RtConfig::Local ), rt::current_rt() );
}



#[test]
//
fn thread_pool()
{
	assert_eq!( None, rt::current_rt() );

	rt::init( RtConfig::Pool ).expect( "no double executor init" );

	assert_eq!( Some( RtConfig::Pool ), rt::current_rt() );
}



#[test]
//
fn spawn()
{
	assert_eq!( None, rt::current_rt() );

	rt::spawn( async {} ).expect( "spawn" );

	assert_eq!( Some( RtConfig::Pool ), rt::current_rt() );
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
