#![ feature( async_await ) ]

#[ cfg(not( target_arch = "wasm32" )) ] pub mod rt             ;
#[ cfg(not( target_arch = "wasm32" )) ] pub use { rt::* }      ;

#[ cfg(     target_arch = "wasm32" )  ] pub mod wasm_rt        ;
#[ cfg(     target_arch = "wasm32" )  ] pub use { wasm_rt::*, wasm_rt as rt } ;


mod error;
pub use { error::* };


mod import
{
	pub use
	{
		once_cell :: { unsync::OnceCell, unsync::Lazy, unsync_lazy } ,
		failure :: { Backtrace, Fail, Context as FailContext } ,

		std :: { fmt, future::Future, rc::Rc, cell::RefCell, pin::Pin },

		futures ::
		{
			prelude :: { Stream, StreamExt, Sink, SinkExt                                         } ,
			channel :: { oneshot, mpsc                                                            } ,
			future  :: { FutureExt, TryFutureExt                                                  } ,
			task    :: { Spawn, SpawnExt, LocalSpawn, LocalSpawnExt, Context as TaskContext, Poll } ,

			executor::
			{
				LocalPool    as LocalPool03    ,
				LocalSpawner as LocalSpawner03 ,
				ThreadPool   as ThreadPool03   ,
			},
		},
	};
}
