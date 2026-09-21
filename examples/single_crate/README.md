# Single crate example

This example demonstrates how to use `hotcode::hotreload` in library crates that have runnable binaries.

Run project with `cargo run`, then make changes to [`src/lib.rs`](src/lib.rs) and rebuild library with
`cargo build --lib` without stopping running binary. You should see new value in console output:

```
...
[examples\single_target\bin\main.rs:6:9] result = 2
# code change and rebuild
[examples\single_target\bin\main.rs:6:9] result = 4
...
```