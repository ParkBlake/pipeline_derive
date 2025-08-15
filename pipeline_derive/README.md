# pipeline_derive

A procedural macro crate providing a `#[derive(Pipeline)]` macro to build **_monadic-style pipelines_** over `Option<T>` fields.

## Overview

This crate enables you to wrap a single `Option<T>` field in a struct and automatically generate convenient pipeline methods that chain multiple processing steps. Each step is a closure consuming the value and returning `Option<T>`, allowing early termination on failure (`None`), embodying Rust’s **_monadic pattern_** with `Option`.

## Features

- Automatically generate pipeline methods (`process3`, `process4`) for 2 or 3-step pipelines.
- Each pipeline step is a closure `FnOnce(T) -> Option<T>`.
- Pipeline chains steps with short-circuiting via `Option::and_then`.
- Simple, minimal API.

## Usage Example

```rust
use pipeline_derive::Pipeline;

#[derive(Pipeline)]
struct MyPipeline {
    value: Option<i32>,
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