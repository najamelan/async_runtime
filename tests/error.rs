#![ cfg(not( target_arch = "wasm32" )) ]
#![ feature( async_await ) ]

// Tested:
//
// - ✔ shut down local pool before spawning does not generate error.
// - ✔ double executor init error: Local - Local.
// - ✔ double executor init error: Local - Pool.
// - ✔ double executor init error: Pool  - Local.
//


use
{
	async_runtime :: { *             } ,
	futures       :: { future::ready } ,
};




// This is an attempt to trigger the shutdown error on the localpool from the futures lib, but run does not
// shut it down, so I don't know how to trigger this error.
//
#[test]
//
fn shutdown()
{
	rt::init( RtConfig::Local ).expect( "no double executor init" );
	rt::run();

	let result = rt::spawn( ready(()) );

	assert!( result.is_ok() );

	rt::run();
}



// Trigger DoubleExecutorInit.
//
#[test]
//
fn double_init()
{
	             rt::init( RtConfig::Local ).expect( "no double executor init" );
	let result = rt::init( RtConfig::Local );

	assert_eq!( &RtErrKind::DoubleExecutorInit, result.unwrap_err().kind() );
}



// Trigger DoubleExecutorInit with 2 different executors.
//
#[test]
//
fn double_init_different()
{
	             rt::init( RtConfig::Local ).expect( "no double executor init" );
	let result = rt::init( RtConfig::Pool  );

	assert_eq!( &RtErrKind::DoubleExecutorInit, result.unwrap_err().kind() );
}



// Trigger DoubleExecutorInit with 2 different executors.
//
#[test]
//
fn double_init_inverse()
{
	             rt::init( RtConfig::Pool  ).expect( "no double executor init" );
	let result = rt::init( RtConfig::Local );

	assert_eq!( &RtErrKind::DoubleExecutorInit, result.unwrap_err().kind() );
}


