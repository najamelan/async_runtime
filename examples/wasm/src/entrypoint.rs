use wasm_bindgen::prelude::*;


use
{
	async_runtime :: { * },
};


// Called when the wasm module is instantiated
//
#[ wasm_bindgen( start ) ]
//
pub fn main() -> Result<(), JsValue>
{
	// Since there is no threads in wasm for the moment, this is optional if you include async_runtime
	// with `default-dependencies = false`, the local pool will be the default. However this might
	// change in the future.
	//
	rt::init( RtConfig::Local ).expect( "Set default executor" );

	let program = async move
	{
		let window   = web_sys::window  ().expect( "no global `window` exists"        );
		let document = window  .document().expect( "should have a document on window" );
		let body     = document.body    ().expect( "document should have a body"      );

		// Manufacture the element we're gonna append
		//
		let val = document.create_element( "div" ).expect( "Failed to create div" );

		val.set_inner_html( &format!( "You succesfully spawned a future" ) );

		body.append_child( &val ).expect( "Coundn't append child" );
	};


	rt::spawn( program ).expect( "spawn program" );

	Ok(())
}
