// See: https://github.com/rust-lang/rust/issues/44732#issuecomment-488766871
//
#![cfg_attr( feature = "external_doc", feature(external_doc) )]
#![cfg_attr( feature = "external_doc", doc(include = "../README.md"))]
//!


#![ doc    ( html_root_url = "https://docs.rs/naja_runtime_macros" ) ]
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
	syn        :: { ItemFn      } ,
};



#[ proc_macro_attribute ]
//
pub fn localpool( _args: TokenStream, item: TokenStream ) -> TokenStream
{
	let input = match parse( item )
	{
		Ok (i) => i                                  ,
		Err(e) => return e.to_compile_error().into() ,
	};


	let vis   = &input.vis        ;
	let name  = &input.sig.ident  ;
	let args  = &input.sig.inputs ;
	let ret   = &input.sig.output ;
	let body  = &input.block      ;
	let attrs = &input.attrs      ;

	let tokens = quote!
	{
		#( #attrs )*
		//
		#vis fn #name( #args ) #ret
		{
			async_runtime::rt::init_allow_same( async_runtime::rt::Config::LocalPool ).expect( "no double executor init" );

			let body = async move #body ;

			let handle = async_runtime::rt::spawn_handle_local( body ).expect( "spawn from proc macro attribute" );
			async_runtime::rt::localpool::run().expect( "LocalPool executor" );
			async_runtime::rt::block_on( handle )
		}
	};

	tokens.into()
}



#[ proc_macro_attribute ]
//
pub fn juliex( _args: TokenStream, item: TokenStream ) -> TokenStream
{
	let input = match parse( item )
	{
		Ok (i) => i                                  ,
		Err(e) => return e.to_compile_error().into() ,
	};


	let vis   = &input.vis        ;
	let name  = &input.sig.ident  ;
	let args  = &input.sig.inputs ;
	let ret   = &input.sig.output ;
	let body  = &input.block      ;
	let attrs = &input.attrs      ;

	let tokens = quote!
	{
		#(#attrs)*
		//
		#vis fn #name( #args ) #ret
		{
			async_runtime::rt::init_allow_same( async_runtime::rt::Config::Juliex ).expect( "no double executor init" );

			async_runtime::rt::block_on( async move #body )
		}
	};

	tokens.into()
}



#[ proc_macro_attribute ]
//
pub fn async_std( _args: TokenStream, item: TokenStream ) -> TokenStream
{
	let input = match parse( item )
	{
		Ok (i) => i                                  ,
		Err(e) => return e.to_compile_error().into() ,
	};


	let vis   = &input.vis        ;
	let name  = &input.sig.ident  ;
	let args  = &input.sig.inputs ;
	let ret   = &input.sig.output ;
	let body  = &input.block      ;
	let attrs = &input.attrs      ;

	let tokens = quote!
	{
		#(#attrs)*
		//
		#vis fn #name( #args ) #ret
		{
			async_runtime::rt::init_allow_same( async_runtime::rt::Config::AsyncStd ).expect( "no double executor init" );

			async_runtime::rt::block_on( async move #body )
		}
	};

	tokens.into()
}



#[ proc_macro_attribute ]
//
pub fn bindgen( _args: TokenStream, item: TokenStream ) -> TokenStream
{
	let input = match parse( item )
	{
		Ok (i) => i                                  ,
		Err(e) => return e.to_compile_error().into() ,
	};

	let vis   = &input.vis        ;
	let name  = &input.sig.ident  ;
	let args  = &input.sig.inputs ;
	let ret   = &input.sig.output ;
	let body  = &input.block      ;
	let attrs = &input.attrs      ;

	let tokens = quote!
	{
		#( #attrs )*
		//
		#vis fn #name( #args ) #ret
		{
			async_runtime::rt::init_allow_same( async_runtime::rt::Config::Bindgen ).expect( "no double executor init" );

			let body = async move #body ;

			async_runtime::rt::spawn_local( body ).expect( "spawn from proc macro attribute" );
		}
	};

	tokens.into()
}



fn parse( item: TokenStream ) -> Result< ItemFn, syn::Error>
{
	let input: ItemFn = syn::parse( item )?;


	if input.sig.asyncness.is_none()
	{
		let msg = "Functions tagged with the async runtime still require the async keyword.";

		return Err( syn::Error::new_spanned( input.sig.fn_token, msg ) )?;
	}


	Ok( input )
}

