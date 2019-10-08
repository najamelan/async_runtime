use tokio::runtime::current_thread::Runtime;
use tokio::prelude::*;

let mut runtime = Runtime::new().unwrap();

// Use the runtime...
// runtime.block_on(f); // where f is a future
