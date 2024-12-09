use crate::displayer::{DisplayProxy, DisplayerOf};
use crate::trait_flag:: TraitFlags;
use core::cmp::Ordering;
use core::fmt;
use core::hash::{Hash, Hasher};
use core::marker::PhantomData;

pub struct Id<const TF: TraitFlags, Entity, Repr>(
    Repr,
    PhantomData<core::sync::atomic::AtomicPtr<Entity>>,
);

impl<const TF: TraitFlags, Entity, Repr> Id<TF, Entity, Repr> {
    pub const fn get(&self) -> &Repr {
        &self.0
    }
    pub const fn new(repr: Repr) -> Id<TF, Entity, Repr> {
        Id(repr, PhantomData)
    }
}

impl<const TF: TraitFlags, Entity, Repr> Id<TF, Entity, Repr>
where
    Entity: DisplayerOf<Self>, //TODO rmeove: DisplayerOf<Id<TF, Entity, Repr>>,
{
    /// `display` provides a machanism to implement a custom display
    /// for phantom types.
    ///
    /// ```
    /// use phantom_newtype::{Id, DisplayerOf};
    /// use core::fmt;
    ///
    /// enum Message {}
    /// type MessageId = Id<Message, [u8; 32]>;
    ///
    /// impl DisplayerOf<MessageId> for Message {
    ///   fn display(id: &MessageId, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    ///     id.get().iter().try_for_each(|b| write!(f, "{:02x}", b))
    ///   }
    /// }
    ///
    /// let vec: Vec<_> = (0u8..32u8).collect();
    /// let mut arr: [u8; 32] = [0u8; 32];
    /// (&mut arr[..]).copy_from_slice(&vec[..]);
    ///
    /// assert_eq!(format!("{}", MessageId::from(arr).display()),
    ///            "000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f");
    /// ```
    pub fn display(&self) -> DisplayProxy<'_, Self, Entity> {
        DisplayProxy::new(self)
    }
}

pub fn test_display<const TF: TraitFlags, Entity, Repr>(id: Id<TF, Entity, Repr>)
{
    use crate::{Id, DisplayerOf};
    use core::fmt;

    enum Message {}
    type MessageId = Id<Message, [u8; 32]>;

    impl DisplayerOf<MessageId> for Message {
    fn display(id: &MessageId, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        id.get().iter().try_for_each(|b| write!(f, "{:02x}", b))
    }
    }

    let arr = [1u8; 32];
    MessageId::from(arr).display();
}

impl<const TF: TraitFlags, Entity, Repr: Clone> Clone for Id<TF, Entity, Repr> {
    fn clone(&self) -> Self {
        Self::from(self.get().clone())
    }
}

impl<Entity, Repr: Copy> Copy for Id<{ TraitFlags::TRAIT_FLAGS_IS_COPY_IS_DEFAULT }, Entity, Repr> {}

impl<Unit, Repr: Default> Default
    for Id<{ TraitFlags::TRAIT_FLAGS_IS_COPY_IS_DEFAULT }, Unit, Repr>
{
    fn default() -> Self {
        Self(Default::default(), PhantomData)
    }
}

impl<const TF: TraitFlags, Entity, Repr: PartialEq> PartialEq for Id<TF, Entity, Repr> {
    fn eq(&self, rhs: &Self) -> bool {
        self.get().eq(&rhs.get())
    }
}

impl<const TF: TraitFlags, Entity, Repr> From<Repr> for Id<TF, Entity, Repr> {
    fn from(repr: Repr) -> Self {
        Self::new(repr)
    }
}

impl<const TF: TraitFlags, Entity, Repr: Eq> Eq for Id<TF, Entity, Repr> {}

impl<const TF: TraitFlags, Entity, Repr: fmt::Debug> fmt::Debug for Id<TF, Entity, Repr> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.get())
    }
}

