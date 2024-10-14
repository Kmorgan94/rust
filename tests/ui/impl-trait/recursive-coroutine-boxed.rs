//@ revisions: current next
//@ ignore-compare-mode-next-solver (explicit revisions)
//@[current] check-pass
//@[next] compile-flags: -Znext-solver
#![feature(coroutines, coroutine_trait)]

use std::ops::{Coroutine, CoroutineState};

fn foo() -> impl Coroutine<Yield = (), Return = ()> {
    //[next]~^ ERROR the size for values of type `impl Coroutine<Yield = (), Return = ()>` cannot be known at compilation time

    // FIXME(-Znext-solver): this fails with a mismatched types as the
    // hidden type of the opaque ends up as {type error}. We should not
    // emit errors for such goals.
    #[coroutine] || {
        let mut gen = Box::pin(foo());
        //[next]~^ ERROR type annotations needed
        //[next]~| ERROR the size for values of type `impl Coroutine<Yield = (), Return = ()>` cannot be known at compilation time
        let mut r = gen.as_mut().resume(());
        while let CoroutineState::Yielded(v) = r {
            yield v;
            r = gen.as_mut().resume(());
        }
    }
}

fn main() {
    foo();
}
