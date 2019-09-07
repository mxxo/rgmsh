Run the examples using `cargo run --example <example>`

Run the tests using `cargo test -- --test-threads=1`

Gmsh is a shared resource, and Rust tests run in parallel by default, so `cargo test` 
alone will crash.  
