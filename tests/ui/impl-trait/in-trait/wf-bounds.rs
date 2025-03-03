// issue #101663
// [next] compile-flags: -Zlower-impl-trait-in-trait-to-assoc-ty
// revisions: current next

#![feature(return_position_impl_trait_in_trait)]
#![allow(incomplete_features)]

use std::fmt::Display;

trait Wf<T> {
    type Output;
}

struct NeedsDisplay<T: Display>(T);

trait Uwu {
    fn nya() -> impl Wf<Vec<[u8]>>;
    //~^ ERROR the size for values of type `[u8]` cannot be known at compilation time

    fn nya2() -> impl Wf<[u8]>;
    //~^ ERROR the size for values of type `[u8]` cannot be known at compilation time

    fn nya3() -> impl Wf<(), Output = impl Wf<Vec<[u8]>>>;
    //~^ ERROR the size for values of type `[u8]` cannot be known at compilation time

    fn nya4<T>() -> impl Wf<NeedsDisplay<T>>;
    //~^ ERROR `T` doesn't implement `std::fmt::Display`
}

fn main() {}
