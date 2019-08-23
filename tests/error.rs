#![ cfg(not( target_arch = "wasm32" )) ]

// Tested:
//
// - ✔ shut down local pool before spawning does not generate error.


#[ cfg( feature = "localpool" ) ]
//
use async_runtime as rt;




// This is an attempt to trigger the shutdown error on the localpool from the futures lib, but run does not
// shut it down, so I don't know how to trigger this error.
//
#[ cfg( feature = "localpool" ) ]
//
#[test]
//
fn shutdown()
{
	rt::init( rt::Config::LocalPool ).expect( "no double executor init" );
	rt::localpool::run().unwrap();

	let result = rt::spawn( async {} );

	assert!( result.is_ok() );

	rt::localpool::run().unwrap();
}


