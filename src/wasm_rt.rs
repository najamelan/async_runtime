//! This is a convenience module for setting a default runtime and allowing code throughout to use spawn.
//! It means you don't have to pass an executor around everywhere.
//!
//! Without this you need to do something like:
//! ```rust, ignore
//! fn main()
//! {
//!    let mut pool  = LocalPool::new();
//!    let mut exec  = pool.spawner();
//!    let mut exec2 = exec.clone();
//!
//!    let program = async move
//!    {
//!       let a = MyActor;
//!
//!       // Create mailbox
//!       //
//!       let mb  : Inbox<MyActor> = Inbox::new();
//!       let mut addr  = Addr::new( mb.sender() );
//!
//!       // Manually spawn the future.
//!       //
//!       let move_mb = async move { mb.start_fut( a ).await; };
//!       exec2.spawn_local( move_mb ).expect( "Spawning mailbox failed" );
//!
//!       let result  = addr.call( Ping( "ping".into() ) ).await;
//!
//!       assert_eq!( "pong".to_string(), result );
//!       dbg!( result );
//!    };
//!
//!    exec.spawn_local( program ).expect( "Spawn program" );
//!
//!    pool.run();
//! }
//! ```
//!
//! Now you get:
//! ```rust, ignore
//! fn main()
//! {
//!    let program = async move
//!    {
//!       let a = MyActor;
//!
//!       // Create mailbox
//!       //
//!       let     mb  : Inbox<MyActor> = Inbox::new();
//!       let mut addr                 = Addr::new( mb.sender() );
//!
//!       mb.start( a ).expect( "Failed to start mailbox" );
//!
//!       let result  = addr.call( Ping( "ping".into() ) ).await;
//!
//!       assert_eq!( "pong".to_string(), result );
//!       dbg!( result );
//!    };
//!
//!    rt::spawn( program ).expect( "Spawn program" );
//!
//!    rt::run();
//! }
//! ```
//!
use crate :: { import::*, RtErr, RtErrKind };


mod wasm_exec;
mod wasm_exec_config;

pub use
{
	wasm_exec        :: * ,
	wasm_exec_config :: * ,
};


thread_local!
(
	static EXEC: OnceCell< WasmExec > = OnceCell::INIT;
);



/// Set the executor to use by default. Run this before calls to run or spawn.
/// This is optional and if you don't set this, the Exec03 executor will be used.
///
/// ### Example
///
/// Use the tokio runtime in order to get support for epoll and the like.
/// ```rust, ignore
/// rt::init( box TokioRT::default() ).expect( "Only set the executor once" );
/// ```
///
//
pub fn init( config: WasmExecConfig ) -> Result< (), RtErr >
{
	EXEC.with( move |exec| -> Result< (), RtErr >
	{
		exec

			.set( WasmExec::new( config ) )
			.map_err( |_| RtErrKind::DoubleExecutorInit.into() )
	})
}


// If no executor is set, set it to Exec03
//
fn default_init()
{
	EXEC.with( move |exec|
	{
		if exec.get().is_none()
		{
			init( WasmExecConfig::default() ).unwrap();
		}
	});
}


/// Spawn a future to be run on the LocalPool (current thread)
/// It will be boxed, because the Executor trait cannot take generic parameters and be object safe...
//
pub fn spawn( fut: impl Future< Output = () > + 'static + Send ) -> Result< (), RtErr >
{
	EXEC.with( move |exec| -> Result< (), RtErr >
	{
		default_init();
		exec.get().unwrap().spawn( fut )
	})
}


/// Spawn a future to be run on the LocalPool (current thread)
/// It will be boxed, because the Executor trait cannot take generic parameters and be object safe...
//
pub fn spawn_local( fut: impl Future< Output = () > + 'static ) -> Result< (), RtErr >
{
	EXEC.with( move |exec| -> Result< (), RtErr >
	{
		default_init();
		exec.get().unwrap().spawn_local( fut )
	})
}


