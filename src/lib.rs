#![allow(incomplete_features)]
#![feature(adt_const_params)]
#![feature(generic_const_exprs)]

mod displayer;
mod id;
mod trait_flag;

pub use displayer::{DisplayProxy, DisplayerOf};

pub type Id<Unit, Repr> = id::Id<{ trait_flag::TraitFlags::TRAIT_FLAGS_IS_COPY_IS_DEFAULT }, Unit, Repr>;
