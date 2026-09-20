# List of `hotcode` usage limitations (for now)

## Common:

### 1. Your `lib` must be of `cdylib` type

This is mandatory because this crate achieves hotreload by reloading dynamic libraries.  
To avoid compilation of your crate as executable and library extract `main` to `bin` section of your crate and turn
remaining parts of code into `lib`.

### 2. No support for generics

This is because function in dynamic library can not know in which ways it's used across dynamic library boundary and
because of that it can't generate function for specific generic parameters.

### 3. `#[hotreload]` attribute can not be used in trait implementation blocks.

This is because `#[hotreload]` splits each function in two: one with source function's code and another that loads
first function from dynamic library and calls it. Newly generated function is not part of trait and because of that
compilation fails:

```rust
trait Foo { fn work(&self); }
struct Bar;

impl Foo for Bar {
    #[hotreload] // doesn't work
    fn work(&self) {}
}
```

Easiest workaround is to create proxy function in concrete type's implementation with `#[hotreload]` attribute and call
it from trait's function:

```rust
trait Foo { fn work(&self); }
struct Bar;

impl Foo for Bar {
    fn work(&self) { self.work_core(); }
}

impl Bar {
    #[hotreload] // works
    fn work_core(&self) {}
}
```

### 4. Changing API of functions is Undefined Behaviour

If your function expected `f32` but after hotreload you replaced it with `f64` but didn't update calling site then you
will get UB.  
If you have at first following code:

```rust
fn foo() { bar("quo vadis") }

#[hotreload]
fn bar(str: &str) { println!("{str:?}") }
```

And then change it to:

```rust
fn foo() { bar(&[1, 2, 3]) }

#[hotreload]
fn bar(array: &[i32]) { println!("{array:?}") }
```

You will get UB because `foo` wasn't updated, and it still supplies string instead of array.  
Correct way to change function's signature is to mark call site as hotreloadable too:
```rust
#[hotreload]
fn foo() { bar("quo vadis") }   // now can safely change "quo vadis" to &[1, 2, 3]

#[hotreload]
fn bar(str: &str) { println!("{str:?}") }   // now can safely change string to array
```

### 5. No support for WebAssembly*

This is because I haven't researched this topic yet. But I saw that other similar libraries
(like [subsecond](https://docs.rs/subsecond/0.7.1/subsecond/)
and [rust-wasm-hotreload](https://github.com/shekohex/rust-wasm-hotreload)) are able to do that, so I guess I'll be able
to do it too (hopefully).

## With `runtime` feature there are few more limitations:

### 6. Function argument types must use fully qualified type*

This is because with `runtime` this library generates separate module that contains all functions labeled with
`#[hotreload(runtime)]`. This module is located at the crates root and because of that it doesn't know about scope
where you used `#[hotreload(runtime)]` attribute.

```rust
mod submodule {
    pub struct Model;
}

#[hotreload(runtime)]   // doesn't work
fn work(model: Model) {}

#[hotreload(runtime)]   // works
fn work(model: submodule::Model) {}
```

### 7. Can not use `#[hotreload]` attribute in comments*

This is because I coded source code parser in a very simple way and didn't bother with adding extra checks for the sake
of performance.

```rust
// ↓ This makes `build.rs` panic

// #[hotreload]
// fn work() {}
```

### 8. `impl`: can be used only in types visible from crate's root*

This is similar to limitation #4: the problem is that generated code is not placed in the same module where you used
`#[hotreload(runtime)]` attribute, and because of that it's required that your structure should be visible from crate's
root.

## Meaning of * (Soon™)

Limitations marked with `*` should be possible to fix or at least I have an idea of how to fix them. I have no promises
that I'll be able to fix them all, but I'll try.
