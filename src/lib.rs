#![allow(incomplete_features)]
#![feature(adt_const_params)]
#![feature(generic_const_exprs)]

mod displayer;
mod id;
pub mod trait_flag;

pub use displayer::{DisplayProxy, DisplayerOf};

pub use id::Id as IdForFlags;

pub type Id<Unit, Repr> = id::Id<{ trait_flag::TRAIT_FLAGS_NO_COPY_NO_DEFAULT }, Unit, Repr>;
