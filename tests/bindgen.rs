#![ cfg( feature = "bindgen" )]

// wasm_bindgen_test currently runs all these tests in the same context, so we can only init once.
//
// Tested:
//
// ✔ basic spawning
// ✔ spawn a future that isn't Send
// ✔ spawn a boxed       future
// ✔ spawn a boxed_local future
// ✔ spawn several
// ✔ spawn from within other task


use
{
	async_runtime as rt,

	wasm_bindgen_test :: { *                                   } ,
	std               :: { rc::Rc, cell::RefCell,              } ,
	futures           :: { future::FutureExt, channel::oneshot } ,
};

wasm_bindgen_test_configure!(run_in_browser);



#[wasm_bindgen_test]
//
fn basic_spawn()
{
	let (tx, rx) = oneshot::channel();

	rt::init_allow_same( rt::Config::Bindgen ).expect( "no double executor init" );


	let task = async move
	{
		tx.send( 1u8 ).expect( "send on channel" );
	};

	rt::spawn( task ).expect( "Spawn task" );


	rt::spawn( async move
	{
		assert_eq!( 1u8, rx.await.expect( "wait on channel" ) );

	}).expect( "spawn assert" )
}



#[wasm_bindgen_test]
//
fn spawn_not_send()
{
	let number  = Rc::new( RefCell::new( 0 ) );
	let num2    = number.clone();
	let (tx, rx) = oneshot::channel();

	rt::init_allow_same( rt::Config::Bindgen ).expect( "no double executor init" );


	let task = async move
	{
		*num2.borrow_mut() = 2;
		tx.send( () ).expect( "send on channel" );
	};

	rt::spawn_local( task ).expect( "Spawn task" );


	rt::spawn_local( async move
	{
		rx.await.expect( "wait on channel" );
		assert_eq!( *number.borrow(), 2 );

	}).expect( "spawn assert" )
}



#[wasm_bindgen_test]
//
fn spawn_boxed()
{
	let (tx, rx) = oneshot::channel();

	rt::init_allow_same( rt::Config::Bindgen ).expect( "no double executor init" );


	let task = async move
	{
		tx.send( 3u8 ).expect( "send on channel" );

	}.boxed();

	rt::spawn( task ).expect( "Spawn task" );


	rt::spawn( async move
	{
		assert_eq!( 3u8, rx.await.expect( "wait on channel" ) );

	}).expect( "spawn assert" )
}



#[wasm_bindgen_test]
//
fn spawn_boxed_local()
{
	let (tx, rx) = oneshot::channel();

	rt::init_allow_same( rt::Config::Bindgen ).expect( "no double executor init" );


	let task = async move
	{
		tx.send( 4u8 ).expect( "send on channel" );

	}.boxed_local();

	rt::spawn_local( task ).expect( "Spawn task" );


	rt::spawn_local( async move
	{
		assert_eq!( 4u8, rx.await.expect( "wait on channel" ) );

	}).expect( "spawn assert" )
}



#[wasm_bindgen_test]
//
fn several()
{
	let (tx , rx ) = oneshot::channel();
	let (tx2, rx2) = oneshot::channel();

	rt::init_allow_same( rt::Config::Bindgen ).expect( "no double executor init" );


	let task = async move
	{
		tx2.send  ( 4u8 + rx.await.expect( "wait channel" ) as u8 )
		   .expect( "send on channel" )
		;
	};

	let task2 = async move
	{
		tx.send( 1u8 ).expect( "send on channel" );
	};

	rt::spawn( task  ).expect( "Spawn task"  );
	rt::spawn( task2 ).expect( "Spawn task2" );

	rt::spawn( async move
	{
		assert_eq!( 5u8, rx2.await.expect( "wait on channel" ) as u8 );

	}).expect( "Spawn assert" );
}



#[wasm_bindgen_test]
//
fn within()
{
	let (tx , rx ) = oneshot::channel();
	let (tx2, rx2) = oneshot::channel();

	rt::init_allow_same( rt::Config::Bindgen ).expect( "no double executor init" );


	let task = async move
	{
		tx2.send  ( 5u8 + rx.await.expect( "wait channel" ) as u8 )
		   .expect( "send on channel" )
		;

		let task2 = async move
		{
			tx.send( 2u8 ).expect( "send on channel" );
		};

		rt::spawn( task2 ).expect( "Spawn task2" );
	};

	rt::spawn( task  ).expect( "Spawn task"  );


	rt::spawn( async move
	{
		assert_eq!( 7u8, rx2.await.expect( "wait on channel" )as u8 );

	}).expect( "Spawn assert" );
}



// Spawn_handle, return string.
//
#[wasm_bindgen_test]
//
fn spawn_handle()
{
	rt::init_allow_same( rt::Config::Bindgen ).expect( "no double executor init" );

	let handle = rt::spawn_handle( async { "hello".to_string() } ).expect( "spawn_handle" );

	rt::spawn( async { assert_eq!( "hello", &handle.await ); } ).expect( "spawn" );
}



// Verify that we can spawn !Send futures.
//
#[wasm_bindgen_test]
//
fn spawn_handle_local()
{
	rt::init_allow_same( rt::Config::Bindgen ).expect( "no double executor init" );

	let handle = rt::spawn_handle_local( async
	{
		let rc = Rc::new( "some string" );

		async { 3+3 }.await;

		let _rc2 = rc.clone();

		"hello".to_string()

	}).expect( "spawn_handle" );

	rt::spawn_local( async { assert_eq!( "hello", &handle.await ); } ).expect( "spawn" );
}



/*

This is removed for now. We don't test correctly here. We only test code that doesn't actually
have to block the thread. When it does, it calls thread::park, which will panic in WASM.

#[test]
//
fn block_on()
{
	let (tx, rx) = oneshot::channel();

	rt::block_on( async { tx.send( 2 ).expect( "send on channel" ); } );

	rt::block_on( async move
	{
		let num: u8 = rx.await.expect( "wait for channel" );
		assert_eq!( 2, num );
	});
}



#[test]
//
fn block_on_boxed()
{
	let (tx, rx) = oneshot::channel();

	rt::block_on( async { tx.send( 2 ).expect( "send on channel" ); }.boxed() );

	rt::block_on( async move
	{
		let num: u8 = rx.await.expect( "wait for channel" );
		assert_eq!( 2, num );
	});
}



#[test]
//
fn block_on_boxed_local()
{
	let (tx, rx) = oneshot::channel();

	rt::block_on( async { tx.send( 2 ).expect( "send on channel" ); }.boxed_local() );

	rt::block_on( async move
	{
		let num: u8 = rx.await.expect( "wait for channel" );
		assert_eq!( 2, num );
	});
}*/
