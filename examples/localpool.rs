//! In this example we make a future that is not Send. We then spawn that future on a LocalPool executor.

use
{
	async_runtime :: { *                     } ,
	std           :: { rc::Rc, cell::RefCell } ,
};


#[ rt::local ]
//
async fn main()
{
	// RefCell is not Send
	//
	let number  = Rc::new( RefCell::new( 0 ) );
	let num2    = number.clone();

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

	let result = *number.borrow();

	dbg!( result );
	assert_eq!( result, 2 );
}


