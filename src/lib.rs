// See: https://github.com/rust-lang/rust/issues/44732#issuecomment-488766871
//!
#![ cfg_attr( feature = "external_doc", feature(external_doc)         ) ]
#![ cfg_attr( feature = "external_doc", doc(include = "../README.md") ) ]




#![ doc    ( html_root_url = "https://docs.rs/naja_async_runtime" ) ]
#![ deny   ( missing_docs                                         ) ]
#![ forbid ( unsafe_code                                          ) ]
#![ allow  ( clippy::suspicious_else_formatting                   ) ]

#![ warn
(
	missing_debug_implementations ,
	missing_docs                  ,
	nonstandard_style             ,
	rust_2018_idioms              ,
)]


pub mod rt   ;
    mod error;

pub use error::*;


mod import
{
	pub(crate) use
	{
		once_cell :: { unsync::OnceCell                       } ,
		std       :: { cfg, fmt, future::Future, error::Error } ,
		futures   :: { future::FutureExt                      } ,
	};


	#[ cfg(not( target_arch = "wasm32" )) ]
	//
	pub(crate) use
	{
		std     :: { cell::RefCell                                                              } ,
		futures :: { task::LocalSpawnExt, executor::{ LocalPool as FutLocalPool, LocalSpawner } } ,
	};


	#[ cfg( feature = "bindgen" ) ]
	//
	pub(crate) use
	{
		wasm_bindgen_futures :: { futures_0_3::spawn_local } ,
	};


	#[ cfg( feature = "juliex" ) ]
	//
	pub(crate) use
	{
		once_cell :: { sync::OnceCell as SyncOnceCell } ,
	};
}
