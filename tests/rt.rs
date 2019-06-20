#![ cfg(not( target_arch = "wasm32" )) ]
#![ feature( async_await ) ]

// Tested:
//
// - ✔ current_rt for localpool
// - ✔ current_rt for threadpool
// - ✔ current_rt for after spawning it should be a threadpool
// - basic methods like spawn and init are being tested in the other integration test files


use
{
	async_runtime :: { * } ,
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
