#![ cfg(all( target_arch = "wasm32", feature = "macros", feature = "bindgen" )) ]


// wasm_bindgen_test currently runs all these tests in the same context, so we can only init once.
//
// Tested:
//
// - ✔ async test method doing an await
// - ✔ a second one to verify double executor init


use
{
	wasm_bindgen_test :: { * } ,
	async_runtime     :: { * } ,
};

wasm_bindgen_test_configure!(run_in_browser);


#[ rt::bindgen       ]
#[ wasm_bindgen_test ]
//
async fn attribute_test_method()
{
	assert_eq!( &hello_world().await, "You succesfully spawned a future" );
}



// As wasm has no threads, verify what happens on a possible double executor init.
//
#[ rt::bindgen       ]
#[ wasm_bindgen_test ]
//
async fn double_executor_init()
{
	assert_eq!( &hello_world().await, "You succesfully spawned a future" );
}



async fn hello_world() -> String
{
	format!( "You succesfully spawned a future" )
}



