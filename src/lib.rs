#![allow(incomplete_features)]
#![feature(adt_const_params)]
#![feature(generic_const_exprs)]

#[derive(Eq, PartialEq, core::marker::ConstParamTy)]
pub enum TraitFlags {
    ONE,
}

pub trait DisplayerOf<T> {}

pub struct IdForFlags<const TF: TraitFlags, Repr>(Repr);

impl<const TF: TraitFlags, Repr> IdForFlags<TF, Repr> {
    pub const fn new(repr: Repr) -> Self {
        Self(repr)
    }
}

pub type Id<Repr> = IdForFlags<{ TraitFlags::ONE }, Repr>;

/// ```
/// #![feature(generic_const_exprs)]
///
/// use phantom_newtype::DisplayerOf;
///
/// enum Message {}
/// // This causes ICE (with feature `unstable_generic_const_own_type`):
/// type MessageId = phantom_newtype::Id<()>;
/// // No ICE:
/// //type MessageId = phantom_newtype::IdForFlags<{phantom_newtype::TraitFlags::ONE}, ()>;
///
/// impl DisplayerOf<MessageId> for Message {}
///
/// MessageId::new(());
/// ```
pub const SEE_DOC_TEST_FOR_ICE: () = {};
