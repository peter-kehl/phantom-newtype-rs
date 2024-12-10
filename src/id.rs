use crate::displayer::{DisplayProxy, DisplayerOf};
use crate::trait_flag::TraitFlags;
use core::marker::PhantomData;

#[repr(transparent)]
pub struct Id<const TF: TraitFlags, Entity, Repr>(
    Repr,
    PhantomData<core::sync::atomic::AtomicPtr<Entity>>,
);

impl<const TF: TraitFlags, Entity, Repr> Id<TF, Entity, Repr> {
    pub const fn new(repr: Repr) -> Id<TF, Entity, Repr> {
        Id(repr, PhantomData)
    }
}

impl<const TF: TraitFlags, Entity, Repr> Id<TF, Entity, Repr>
where
    Entity: DisplayerOf<Self>,
{
    /// `display` provides a mechanism to implement a custom display
    /// for phantom types.
    ///
    /// ```
    /// #![feature(generic_const_exprs)]
    ///
    /// use phantom_newtype::DisplayerOf;
    /// use core::fmt;
    ///
    /// enum Message {}
    /// // This causes ICE (with feature `unstable_generic_const_own_type`):
    /// //type MessageId = phantom_newtype::Id<Message, [u8; 32]>;
    /// // No ICE:
    /// type MessageId = phantom_newtype::IdForFlags<{phantom_newtype::trait_flag::TraitFlags::ONE}, Message, [u8; 32]>;
    ///
    /// impl DisplayerOf<MessageId> for Message {
    ///   fn display(id: &MessageId, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    ///     todo!()
    ///   }
    /// }
    ///
    /// MessageId::from([0u8; 32]);
    /// ```
    pub fn display(&self) -> DisplayProxy<'_, Self, Entity> {
        DisplayProxy::new(self)
    }
}


#[cfg_attr(feature = "unstable_generic_const_own_type", allow(deprecated))]
impl<const TF: TraitFlags, Entity, Repr> From<Repr> for Id<TF, Entity, Repr> {
    fn from(repr: Repr) -> Self {
        Self::new(repr)
    }
}
