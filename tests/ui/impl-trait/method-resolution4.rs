//! The recursive method call yields the opaque type. The
//! `next` method call then constrains the hidden type to `&mut _`
//! because `next` takes `&mut self`. We never resolve the inference
//! variable, but get a type mismatch when comparing `&mut _` with
//! `std::iter::Empty`.

//@ revisions: current next
//@[next] compile-flags: -Znext-solver
//@[current] check-pass

fn foo(b: bool) -> impl Iterator<Item = ()> {
    //[next]~^ ERROR the size for values of type `impl Iterator<Item = ()>` cannot be known at compilation time
    if b {
        foo(false).next().unwrap();
        //[next]~^ type annotations needed
        //[next]~| ERROR the size for values of type `impl Iterator<Item = ()>` cannot be known at compilation time
    }
    std::iter::empty()
}

fn main() {}
