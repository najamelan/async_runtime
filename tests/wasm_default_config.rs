#![ cfg( target_arch = "wasm32" )]


// Verify that the default configuration works (no bindgen future enabled).
//
// Tested:
//
// ✔ default_config


use
{
	wasm_bindgen_test :: { *                } ,
	async_runtime     :: { *                } ,
	futures           :: { channel::oneshot } ,
};



wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
//
fn default_conf()
{
	let (tx, rx) = oneshot::channel();

	let task = async move
	{
		tx.send( 1u8 ).expect( "send on channel" );
	};

	rt::spawn( task ).expect( "Spawn task" );


	rt::spawn( async move
	{
		assert_eq!( 1u8, rx.await.expect( "wait on channel" ) );

	}).expect( "spawn assert" )
}
