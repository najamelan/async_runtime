# async_runtime attributes

- on main
- on tests
- on benchmarks
- on wasm

return types?


TODO:

- WASM support
- WASM spawn_handle
- run_until support?
- double executor error on same executor or not?
- ditch failure
- test with dependent crates
- release


## Usage

This crate introduces attributes that you can tag on async methods to make them synchronous. Eg.
```rust
#[rt::local]
//
async fn main() -> Result<(), io::Error>
{
	let work = some_async_io().await?;

	Ok(())
}
```

Rust expects a synchronous `main` function. With this macro it gets one. However you can `await` or call `rt::spawn` inside the body directly. It expands to:

```rust
fn main() -> Result<(), ()>
{
	match rt::current_rt()
	{
		None => rt::init( rt::RtConfig::Local ).unwrap(),

		Some(cfg) =>
		{
			if RtConfig::Local != cfg
			{
				panic!( RtErr::from( RtErrKind::DoubleExecutorInit ) );
			}
		}
	}


	let body = async move
	{
		let work = some_async_io().await?;

		Ok(())
	};

	let handle = rt::spawn_handle_local( body ).expect( "spawn" );

	rt::run();
	rt::block_on( handle )
}
```

So, the sugaring does:
1. initialize the right executor for this thread if it's not already initialized.
2. if it's already initialized, we verify that it's the right kind, otherwise this is a fatal error.
3. create an async block to run your body
4. **block the thread** to wait for the outcome of your function.

You might wonder why we spawn the future instead of just using `block_on`. The reason is that you might want to spawn further tasks from within your function. If would just `block_on` the top level, that wouldn't work.

TODO: run_until? would need support in async_runtime first

Some warnings are in order. If you call this on several methods in the same thread, you will probably get undesirable results:

1. you cannot mix `rt::local` and `rt::thread_pool` in the same thread or you will get a panic. Each thread can only have 1 executore initialized per thread. That's how `rt::spawn` knows where to send tasks, even in library code you might evoke.
2. since we have to make it synchronous, it will block the thread. So if you do that at several places, things might seriously slow down.

All in all, this is meant for top level functions, like `main`, and unit/integration tests, which run in separate threads each, but you can use it to make other methods sync if you understand the warnings above.


### A note about main

As you can see from the example, you can return values from methods tagged with this attribute. In reality, I would recommend you don't do this in `main`. Returning a result is as good as calling `unwrap`. They will be fatal errors, that you haven't handled, nor would be necessarily be well formatted for end users. So you probaby don't want it in production, and in my opinion, since you don't want it in production, you don't want it in example code. To keep things simple, as you can see I would favor `expect` over `unwrap` as you can at least put a human readable error message.

You can do it in unit tests however where you would unwrap, you can now use the `?` operator.


### A note about tests

Just be aware that the attribute captures other attributes and puts them back on your methods, so this works:

```rust
#[rt::local] #[test]
//
fn some_test(){}
```

but this doesn't:

```rust
#[test] #[rt::local]
//
fn some_test(){}
```
... because rustc will try to evaluate the `test` attribute on the function before evaluating our attribute, and thus you will get an error: `test methods cannot be async`.


### Benchmarks

Benchmarks (methods tagged with `#[bench]`) are currently not supported. I'll try to explain why. Let's imagine how it could work:

```rust
#[rt::local] #[bench]
//
async fn basic( b: &mut test::Bencher )
{
	b.iter( ||
	{
		async { /* do something */ }.await;
	})
}
```

First of all, this can never possibly work. Note that the function takes a `Bencher` by reference. In order to spawn it, it would have to be `'static`, which it isn't. So we could do:

```rust
#[rt::local] #[bench]
//
async fn basic()
{
	async { /* do something */ }.await;
}
```

And let the macro add the loop for you, so it would stay out of the future. That's what the runtime crate does. However what happens when:


```rust
#[rt::local] #[bench]
//
async fn basic()
{
	my_work().await;
}

async fn my_work()
{
	rt::spawn( ... );
}
```

So `my_work` spawns further tasks. On the local pool we would run `rt::run` in each iteration. That function is `futures::executor::LocalPool::run` under the hood, which blocks the thread. Maybe it's what you want, but it's not very obvious what's happening here, and your tests won't run concurrently.

On a threadpool, this completely goes hairy. There is no automated way to wait until all tasks have finished, so you would just be benchmarking the spawn function, which is non blocking and the benchmark would finish even if the work wasn't done. That's very likely not what you want.

All in all, I find all these problems become more obscured with the attribute. It's very hard to get benchmarks right, and I think it's mandatory to understand exactly what's going on. The attribute here is an open invitation for people to shoot themselves in the foot.

And just a matter of personal opinion, the benchmark suite from `test` being not stable and all, I would suggest people use the criterion crate for more serious benchmarks.
