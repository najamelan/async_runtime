#![ cfg(all( target_arch = "wasm32", feature = "bindgen" )) ]

// wasm_bindgen_test currently runs all tests in the same context, so we can only init once.
//
// Tested:
//
// - ✔ trigger double init error



use
{
	async_runtime as rt,

	wasm_bindgen_test :: { * } ,
};

wasm_bindgen_test_configure!(run_in_browser);



#[wasm_bindgen_test]
//
fn double_init()
{
	             rt::init( rt::Config::Bindgen ).expect( "no double executor init" );
	let result = rt::init( rt::Config::Bindgen );

	assert_eq!( &rt::ErrorKind::DoubleExecutorInit, result.unwrap_err().kind() );
}
