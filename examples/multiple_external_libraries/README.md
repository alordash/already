# Multiple external libraries example

This example demonstrates how to use `hotcode::hotreload` when multiple libraries are imported as external dependencies
in crates that have runnable binaries.

This project consists of two library crates: [`first_library`](first_library) and [`second_library`](second_library)
that expose `add(a, b)` and `mul_add(a, b, c)` functions respectively. Second library's `mul_add` uses `add` from first
library.

Run project with `cargo run` from this directory, then make changes to
[`first_library/src/lib.rs`](first_library/src/lib.rs) and rebuild library with `cargo build --lib` without stopping running binary. You
should see that both `add` and `mul_add` result is changed:

```
...
[binary\src\main.rs:5:9] add = 7.0
[binary\src\main.rs:7:9] mul_add = 17.0
# code change and rebuild
[binary\src\main.rs:5:9] add = 8.0
[binary\src\main.rs:7:9] mul_add = 18.0
...
```

If you try to change [`second_library/src/lib.rs`](second_library/src/lib.rs) you'll notice that only result of second library's `mul_add` is
changed, first library won't be affected by these changes:

```
...
[binary\src\main.rs:5:9] add = 7.0
[binary\src\main.rs:7:9] mul_add = 17.0
# code change and rebuild
[binary\src\main.rs:5:9] add = 7.0
[binary\src\main.rs:7:9] mul_add = 20.0
...
```