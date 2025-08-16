# pipeline_derive

A procedural macro crate providing a `#[derive(Pipeline)]` macro to build **_monadic-style pipelines_** over `Option<T>` fields.

## Overview

This crate enables you to wrap a single `Option<T>` field in a struct and automatically generate convenient pipeline methods that chain multiple processing steps. Each step is a closure consuming the value and returning `Option<T>`, allowing early termination on failure (`None`), embodying Rust’s **_monadic pattern_** with `Option`.

New attributes let you customise behaviour:

- `#[pipeline(skip = true)]` — generate pipeline methods that always return `None`, effectively skipping processing.
- `#[pipeline(timeout = <milliseconds>)]` — pipeline methods print timeout info when called.

Generated pipeline methods now take `&self` and internally clone the inner value as needed, so the original struct can be used without transferring ownership. The macro automatically adds the required `T: Clone` trait bound on the generic type.

## Features

- Automatically generate pipeline methods (`process3`, `process4`) for 2 or 3-step pipelines.
- Pipeline steps are closures `FnOnce(T) -> Option<T>`.
- Pipeline chains steps with short-circuiting via `Option::and_then`.
- Attributes to skip processing or log timeout info.
- Methods take `&self` and clone inner value, improving ergonomics.
- Minimal, monadic-style API.

## Usage Example

```rust
use pipeline_derive::Pipeline;

#[derive(Pipeline)]
#[pipeline(timeout = 1000)]
struct MyPipeline<T> {
    value: Option<T>,
}

fn main() {
    let pipeline = MyPipeline { value: Some(7) };

    let result = pipeline.process3(
        |x| Some(x + 3),
        |y| if y > 10 { Some(y * 2) } else { None },
    );

    match result {
        Some(output) => println!("Pipeline succeeded with output: {}", output),
        None => println!("Pipeline terminated early."),
    }
}
```