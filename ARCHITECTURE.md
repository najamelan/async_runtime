# Architecture

## Introduction

async_runtime aims to be a lightweight, low - preferably no - overhead wrapper around the available executors in the async ecosystem. For this, dependencies are kept as few as possible.

All executors are enabled with features.

There is a support crate naja_async_runtime_macros that enables proc macros on functions.

## Design

The idea is to provide a convenient API on the `rt` module. Methods that can be called on all executors live here.
I use an executor per thread, which is kept in a once_cell. On each thread where `rt::spawn` or similar methods are to be called, a call to `rt::init` must first be made to define which executor the user wants to use on this thread.

The actual executor object is an enum in `rt::executor`. It implements all the methods from rt for each executor.

Executor functionality that is to specific to be provided for all executors is provided in modules specific to the executor, like `rt::async_std`.

By using this design, we can avoid boxing futures and executors alike.

## Tests

There is a script `ci.bash` showing all of the tests I run!
