use pipeline_derive::Pipeline;

// This struct won't work with the current Pipeline derive macro,
// because it contains more than one field. The macro requires exactly one named field.
// Uncommenting the following will cause compilation errors:
//
// #[derive(Pipeline)]
// struct MultiFieldPipeline {
//     data: Option<i32>,
//     info: Option<i32>,
// }

#[derive(Pipeline)]
struct SingleFieldPipeline {
    // The macro expects exactly one Option<T> field to operate on.
    value: Option<i32>,
}

fn main() {
    // Initialise the pipeline with a starting value.
    let pipeline = SingleFieldPipeline { value: Some(7) };

    // Define a pipeline with two processing steps:
    // Step 1: Add a fixed value to the input.
    // Step 2: in this instance, if the result exceeds a certain threshold, double it;
    //         otherwise, stop the pipeline early by returning None.
    let output = pipeline.process3(
        |input| Some(input + 3),
        |processed| {
            if processed > 10 {
                Some(processed * 2)
            } else {
                None
            }
        },
    );

    match output {
        Some(result) => println!("Pipeline completed successfully with output: {}", result),
        None => println!("Pipeline terminated early due to a failing condition."),
    }
}
