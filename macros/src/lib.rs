// See: https://github.com/rust-lang/rust/issues/44732#issuecomment-488766871
//
#![cfg_attr( feature = "external_doc", feature(external_doc) )]
#![cfg_attr( feature = "external_doc", doc(include = "../README.md"))]
//!


#![ doc    ( html_root_url = "https://docs.rs/naja_runtime_macros" ) ]
#![ feature( async_await                                           ) ]
#![ forbid ( unsafe_code                                           ) ]
#![ allow  ( clippy::suspicious_else_formatting                    ) ]

#![ warn
(
	missing_debug_implementations ,
	nonstandard_style             ,
	rust_2018_idioms              ,
)]


extern crate proc_macro;


use
{
	proc_macro :: { TokenStream } ,
	quote      :: { quote       } ,
};


enum Config
{
	Local,

	#[ cfg( feature = "juliex" ) ]
	//
	Pool ,
}



#[ proc_macro_attribute ]
//
pub fn local( _args: TokenStream, item: TokenStream ) -> TokenStream
{
	dry( item, Config::Local )
}



#[ cfg(     feature = "juliex"  ) ]
#[ cfg(not( target  = "wasm32" )) ]
//
#[ proc_macro_attribute ]
//
pub fn thread_pool( _args: TokenStream, item: TokenStream ) -> TokenStream
{
	dry( item, Config::Pool )
}



// Actual implementation
//
fn dry( item: TokenStream, cfg: Config ) -> TokenStream
{
	let input = syn::parse_macro_input!( item as syn::ItemFn );


	if input.sig.asyncness.is_none()
	{
		let msg = "Functions tagged with the async runtime still require the async keyword.";

		return syn::Error::new_spanned( input.sig.fn_token, msg ).to_compile_error().into();
	}


	let ret   = &input.sig.output ;
	let name  = &input.sig.ident  ;
	let body  = &input.block      ;
	let attrs = &input.attrs      ;

	let tokens = match cfg
	{
		Config::Local =>
		{
			quote!
			{
				#(#attrs)*
				fn #name() #ret
				{
					rt::init( RtConfig::Local ).expect( "executor init" );

					let body = async { #body };

					rt::spawn_local( body ).expect( "spawn" );

					#[ cfg(not( target = "wasm32" )) ] rt::run();
				}
			}
		}

		#[ cfg( feature = "juliex"  ) ]
		//
		Config::Pool =>
		{
			quote!
			{
				#(#attrs)*
				fn #name() #ret
				{
					rt::init( RtConfig::Pool ).expect( "executor init" );

					rt::block_on( async { #body } )
				}

			}
		}
	};

	tokens.into()
}
