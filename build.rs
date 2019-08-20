
// Does the following:
//
// we always enable bingen on wasm, and localpool on not wasm targets, so the library works even if application
// developers don't depend on it, only libraries. We would set them as the default features, but they are
// platform specific, and cargo does not support platform specific features.
// See: https://github.com/rust-lang/cargo/issues/1197
//
use std::env;

fn main()
{
	match on_wasm()
	{
		true  => setup_wasm   (),
		false => setup_notwasm(),
	}
}


fn setup_wasm()
{
	// If library authors use async_runtime, and application developers don't depend on it explicitly,
	// we still need some default executor to be available for the library. Therefor we enable bindgen.
	// Currently this is the only executor available on wasm, so there is no cost of extra dependencies,
	// and wasm-bindgen-futures is not an optional dependency, notably because setting the feature here
	// does not turn it on in cargo, only rustc.
	//
	println!( "cargo:rustc-cfg=feature=\"bindgen\"" );
}


fn setup_notwasm()
{
	// TODO figure out if this actually works and makes cargo run --example work without having to specify the feature.
	//
	println!( "cargo:rustc-cfg=feature=\"notwasm\"" );

	// If library authors use async_runtime, and application developers don't depend on it explicitly,
	// we still need some default executor to be available for the library. Therefor we enable localpool.
	// Currently we already depend on the futures library, so there is no cost of extra dependencies.
	//
	println!( "cargo:rustc-cfg=feature=\"localpool\"" );
}


fn target() -> String
{
	env::var( "TARGET" ).unwrap()
}

fn on_wasm() -> bool
{
	target() == "wasm32-unknown-unknown"
}
