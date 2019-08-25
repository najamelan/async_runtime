//! Module containing functionality specific to async-std.
//
use crate :: { self as rt, import::*, Error, ErrorKind };



// Async-std does not currently expose a handle to control the threadpool, so it's zero sized and zero control.
//
#[ derive( Debug, Default ) ]
//
pub(crate) struct AsyncStd {}



impl AsyncStd
{
	pub(crate) fn new() -> Self
	{
		Self {}
	}


	pub(crate) fn spawn( &self, fut: impl Future< Output = () > + 'static + Send ) -> Result< (), Error >
	{
		// async-std does not allow initializing worker threads, so we need to check on each spawn.
		//
		async_std_crate::task::spawn( async
		{
			if rt::current_rt().is_none()
			{
				rt::init( rt::Config::AsyncStd ).expect( "no double executor init" );
			}

			fut.await
		});

		Ok(())
	}



	pub(crate) fn spawn_local( &self, _: impl Future< Output = () > + 'static  ) -> Result< (), Error >
	{
		Err( ErrorKind::SpawnLocalOnThreadPool.into() )
	}



	// We don't need to user remote_handle, because async-std provides a handle out of the box.
	//
	pub(crate) fn spawn_handle<T: 'static + Send>( &self, fut: impl Future< Output=T > + Send + 'static )

		-> Result< Box< dyn Future< Output=T > + Send + 'static + Unpin >, Error >

	{
		let task = async
		{
			if rt::current_rt().is_none()
			{
				rt::init( rt::Config::AsyncStd ).expect( "no double executor init" );
			}

			fut.await
		};

		Ok( Box::new( async_std_crate::task::spawn( task )))
	}



	pub(crate) fn spawn_handle_local<T: 'static + Send>( &self, _: impl Future< Output=T > + 'static )

		-> Result< Box< dyn Future< Output=T > + 'static + Unpin >, Error >

	{
		Err( ErrorKind::SpawnLocalOnThreadPool.into() )
	}
}



/// AsyncStd specific version of [`spawn_handle`](crate::spawn_handle). This avoids the need for
/// boxing. The worker thread will be set up to further spawn on AsyncStd.
///
/// ### Errors
///
/// - If you call this from a thread which has no executor set up, this will return
/// [ErrorKind::NoExecutorInitialized].
/// - If you call this from a thread which has another executor set up, this will return
/// [ErrorKind::WrongExecutor].
//
pub fn spawn_handle<F, T>( fut: F ) -> Result< async_std_crate::task::JoinHandle<T>, Error >

	where F: Future<Output = T> + Send + 'static ,
	      T: Send + 'static                      ,

{
	// Order of the match arms is important!
	//
	match rt::current_rt()
	{
		None => Err( ErrorKind::NoExecutorInitialized.into() ),

		Some( rt::Config::AsyncStd ) =>
		{
			let task = async
			{
				if rt::current_rt().is_none()
				{
					rt::init( rt::Config::AsyncStd ).expect( "no double executor init" );
				}

				fut.await
			};

			Ok( async_std_crate::task::spawn( task ) )
		}


		Some(_) => Err( ErrorKind::WrongExecutor.into() ),
	}
}
