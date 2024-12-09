#![allow(incomplete_features)]
#![feature(adt_const_params)]

impl<const TF: (), Repr> Id<TF, Repr> {
    pub const fn new(repr: Repr) -> Self {
        Self(repr)
    }
}

pub type IdOne<Repr> = Id<{ }, Repr>;

pub trait DisplayerOf<T> {
    fn display(value: &T);
}

/// ```
/// // Removal of this #![...] "fixes" the ICE, and then there is a properly reported compile error .
/// #![feature(generic_const_exprs)]
/// use phantom_newtype::{IdOne, DisplayerOf};
/// use core::fmt;
///
/// type MessageId = IdOne<()>;
///
/// // Removal of this impl "fixes" the ICE, and then there is no compile error at all.
/// impl DisplayerOf<MessageId> for () {
///   fn display() {
///   }
/// }
///
/// MessageId::new(());
/// ```
pub struct Id<const TF: (), Repr>(
    pub Repr
);
