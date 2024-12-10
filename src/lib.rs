#![allow(incomplete_features)]
#![feature(adt_const_params)]
#![feature(generic_const_exprs)]

mod id;
pub mod trait_flag;

pub use id::Id as IdForFlags;

pub type Id<Unit, Repr> = id::Id<{ trait_flag::TraitFlags::ONE }, Unit, Repr>;

pub trait DisplayerOf<T> {
    fn display();
}
