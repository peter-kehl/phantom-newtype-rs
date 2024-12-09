use crate::displayer::{DisplayProxy, DisplayerOf};
use crate::trait_flag:: TraitFlags;
use core::marker::PhantomData;

/// ```
/// #![feature(generic_const_exprs)]
/// use phantom_newtype::{Id, DisplayerOf};
/// use core::fmt;
///
/// enum Message {}
/// type MessageId = Id<Message, ()>;
///
/// // Removal of this impl "fixes" the ICE
/// impl DisplayerOf<MessageId> for Message {
///   fn display(id: &MessageId, f: &mut fmt::Formatter<'_>) -> fmt::Result {
///     todo!()
///   }
/// }
///
/// MessageId::new(());
/// ```
pub struct Id<const TF: TraitFlags, Entity, Repr>(
    Repr,
    PhantomData<core::sync::atomic::AtomicPtr<Entity>>,
);

impl<const TF: TraitFlags, Entity, Repr> Id<TF, Entity, Repr> {
    pub const fn new(repr: Repr) -> Id<TF, Entity, Repr> {
        Id(repr, PhantomData)
    }
}
