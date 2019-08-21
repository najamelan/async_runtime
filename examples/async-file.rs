//! This example shows how to use async file io.
//!
//! run with `cargo run --example async-file --features notwasm localpool`
//
use
{
	async_runtime:: { *                   } ,
	async_std    :: { fs::File, io::Error } ,
};

// You can create an async main fn like this. You can also choose `thread_pool` over `local`
//
#[ rt::localpool ]
//
async fn main() -> Result< (), Error >
{
	let file = File::open( "Cargo.yml" ).await?;

	let meta = file.metadata().await?;

	println!( "length of Cargo.yml is {}", meta.len() );

	Ok(())
}
