use
{
	async_runtime :: { *                  } ,
	wasm_bindgen  :: { prelude::*, JsCast } ,
	web_sys       :: { HtmlElement        } ,
};


// Called when the wasm module is instantiated
//
#[ rt::bindgen           ]
#[ wasm_bindgen( start ) ]
//
pub async fn main()
{
	let window   = web_sys::window  ().expect( "no global `window` exists"        );
	let document = window  .document().expect( "should have a document on window" );
	let body     = document.body    ().expect( "document should have a body"      );

	let val: HtmlElement = document.create_element( "div" ).expect( "Failed to create div" ).unchecked_into();

	let hello = hello_world().await;
	val.set_inner_text( &hello );

	body.append_child( &val ).expect( "Coundn't append child" );
}


async fn hello_world() -> String
{
	format!( "You succesfully spawned a future" )
}
