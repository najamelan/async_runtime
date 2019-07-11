#![ feature( async_await ) ]


//! In this example we make a future that is not Send. We then spawn that future on a LocalPool executor.


#[ cfg(not( target_arch = "wasm32" )) ]
//
fn main()
{
	// unfortunately we can't rename the crate itself in Cargo.yml.
	//
	use naja_async_runtime as async_runtime;

	use
	{
		async_runtime :: { *                     } ,
		std           :: { rc::Rc, cell::RefCell } ,
	};

	// RefCell is not Send
	//
	let number  = Rc::new( RefCell::new( 0 ) );
	let num2    = number.clone();

	// Since the default executor is the threadpool we have to initialize a local one explicitly.
	//
	rt::init( RtConfig::Local ).expect( "executor init" );

	let task = async move
	{
		*num2.borrow_mut() = 2;
	};

	// If we initialized the localpool, we normally could just use the method `spawn`, but since that's
	// the same method used for the threadpool, it requires Send. Thus as long as your future is send, you can
	// use the `spawn` method, even on a localpool. Here our future isn't Send, so we have to use local_spawn.
	//
	// `local_spawn` will return an error at runtime if the initialized executor is a threadpool.
	//
	rt::spawn_local( task ).expect( "Spawn task" );


	// On a threadpool, futures are polled immediately, but since here we only have one thread, first we spawn
	// our topmost tasks and then we have to tell the runtime that it's time to start polling them. This will
	// block the thread until all futures are finished.
	//
	rt::run();

	let result = *number.borrow();

	dbg!( result );
	assert_eq!( result, 2 );
}


#[ cfg( target_arch = "wasm32" ) ]
//
fn main(){}

