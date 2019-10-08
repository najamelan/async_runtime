// See: https://github.com/rust-lang/rust/issues/44732#issuecomment-488766871
//!
#![cfg_attr( feature = "external_doc", feature(external_doc)         )]
#![cfg_attr( feature = "external_doc", doc(include = "../README.md") )]


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


#[ cfg(not( target_arch = "wasm32" )) ] pub mod rt                                       ;
#[ cfg(not( target_arch = "wasm32" )) ] pub use { rt::exec03::* }                        ;

#[ cfg(     target_arch = "wasm32" )  ] pub mod wasm_rt                                  ;
#[ cfg(     target_arch = "wasm32" )  ] pub use { wasm_rt::wasm_exec::*, wasm_rt as rt } ;


mod error;
mod rt_config;

pub use
{
	error     :: * ,
	rt_config :: * ,
};


mod import
{
	pub use
	{
		once_cell :: { unsync::OnceCell                                     } ,
		failure   :: { Backtrace, Fail, Context as FailContext              } ,
		std       :: { fmt, future::Future, rc::Rc, cell::RefCell, pin::Pin } ,

		futures ::
		{
			prelude :: { Stream, Sink                                                             } ,
			channel :: { oneshot, mpsc                                                            } ,
			future  :: { FutureExt, TryFutureExt                                                  } ,
			task    :: { Spawn, SpawnExt, LocalSpawn, LocalSpawnExt, Context as TaskContext, Poll } ,
			stream  :: { StreamExt                                                                } ,
			sink    :: { SinkExt                                                                  } ,

			executor::
			{
				LocalPool    as LocalPool03    ,
				LocalSpawner as LocalSpawner03 ,
				ThreadPool   as ThreadPool03   ,
			},
		},
	};
}
