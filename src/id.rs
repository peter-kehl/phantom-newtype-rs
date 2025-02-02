// Copyright 2019 DFINITY
// Copyright 2023,2024 Peter Lyons Kehl
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::displayer::{DisplayProxy, DisplayerOf};
#[cfg_attr(feature = "unstable_generic_const_own_type", allow(deprecated))]
use crate::trait_flag::{self, TraitFlags};
use core::cmp::Ordering;
use core::fmt;
use core::hash::{Hash, Hasher};
use core::marker::PhantomData;
#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// `Id<Entity, Repr>` provides a type-safe way to keep ids of
/// entities. Note that there's no default for `Repr` type, the type
/// of the identifier should be always provided explicitly.
///
/// Example:
///
/// ```
/// use phantom_newtype::Id;
///
/// struct User {
///   id: Id<User, u64>,
///   name: String,
///   posts: Vec<Id<Post, u64>>,
/// }
///
/// struct Post {
///   id: Id<Post, u64>,
///   title: String,
/// }
/// ```
///
/// `Enity` doesn't have to be a struct, any type will do. It's just a
/// marker that differentiate incompatible ids.
///
/// ```compile_fail
/// use phantom_newtype::Id;
///
/// enum Recepient {}
/// enum Message {}
///
/// type RecepientId = Id<Recepient, u64>;
/// type MessageId = Id<Message, u64>;
///
/// assert_eq!(RecepientId::from(15), MessageId::from(15));
/// ```
///
/// `Id` is cheap to copy if `Repr` is:
///
/// ```
/// #![cfg_attr(
///     feature = "unstable_generic_const_own_type",
///     feature(generic_const_exprs)
/// )]
///
/// use phantom_newtype::Id;
///
/// enum Message {}
/// type MessageId = Id<Message, u64>;
///
/// let x = MessageId::from(5);
/// let y = x;
/// assert_eq!(x, y);
/// ```
///
/// `Id` can be used as a key in a hash map as long as `Repr` has
/// this property:
///
/// ```
/// #![cfg_attr(
///     feature = "unstable_generic_const_own_type",
///     feature(generic_const_exprs)
/// )]
///
/// use phantom_newtype::Id;
/// use std::collections::HashMap;
///
/// #[derive(PartialEq, Debug)]
/// struct User {}
/// type UserId = Id<User, String>;
///
/// let mut users_by_id = HashMap::new();
/// let id = UserId::from("john".to_string());
/// users_by_id.insert(id.clone(), User {});
///
/// assert!(users_by_id.get(&id).is_some());
/// ```
///
/// Ids are ordered if the `Repr` is. Note that this is mostly useful
/// e.g. for storing Ids in a `BTreeMap`, there is usually little
/// semantic value in comparing ids.
///
/// ```
/// #![cfg_attr(
///     feature = "unstable_generic_const_own_type",
///     feature(generic_const_exprs)
/// )]
///
/// use std::collections::BTreeMap;
/// use phantom_newtype::Id;
///
/// #[derive(PartialEq, Debug)]
/// struct User {}
/// type UserId = Id<User, u64>;
///
/// let mut map = BTreeMap::new();
/// let id = UserId::from(5);
/// map.insert(id.clone(), User {});
///
/// assert!(map.get(&id).is_some());
/// ```
///
/// Ids can be sent between threads if the `Repr` allows it, no
/// matter which `Entity` is used.
///
/// ```
/// #![cfg_attr(
///     feature = "unstable_generic_const_own_type",
///     feature(generic_const_exprs)
/// )]
///
/// use phantom_newtype::Id;
///
/// type Cell = core::cell::RefCell<i64>;
/// type CellId = Id<Cell, i64>;
/// const ID: CellId = CellId::new(42);
///
/// let id_from_thread = std::thread::spawn(|| &ID).join().unwrap();
/// assert_eq!(ID, *id_from_thread);
/// ```
///
/// Ids can be serialized and deserialized with `serde`. Serialized
/// forms of `Id<Entity, Repr>` and `Repr` are identical.
///
/// ```
/// #![cfg_attr(
///     feature = "unstable_generic_const_own_type",
///     feature(generic_const_exprs)
/// )]
///
/// #[cfg(feature = "serde")] {
/// use phantom_newtype::Id;
/// use serde::{Serialize, Deserialize};
/// use serde_json;
/// enum User {}
///
/// let repr: u64 = 10;
/// let user_id = Id::<User, u64>::from(repr);
/// assert_eq!(serde_json::to_string(&user_id).unwrap(), serde_json::to_string(&repr).unwrap());
/// }
/// ```
#[cfg_attr(feature = "unstable_generic_const_own_type", allow(deprecated))]
#[repr(transparent)]
pub struct Id<const TF: TraitFlags, Entity, Repr>(
    Repr,
    PhantomData<core::sync::atomic::AtomicPtr<Entity>>,
);

#[cfg_attr(feature = "unstable_generic_const_own_type", allow(deprecated))]
impl<const TF: TraitFlags, Entity, Repr> Id<TF, Entity, Repr> {
    /// `get` returns the underlying representation of the identifier.
    ///
    /// ```
    /// #![cfg_attr(
    ///     feature = "unstable_generic_const_own_type",
    ///     feature(generic_const_exprs)
    /// )]
    ///
    /// use phantom_newtype::Id;
    ///
    /// enum User {}
    /// type UserId = Id<User, u64>;
    ///
    /// assert_eq!(*UserId::from(15).get(), 15);
    /// ```
    pub const fn get(&self) -> &Repr {
        &self.0
    }

    /// `new` is a synonym for `from` that can be evaluated in
    /// compile time. The main use-case of this functions is defining
    /// constants:
    ///
    /// ```
    /// #![cfg_attr(
    ///     feature = "unstable_generic_const_own_type",
    ///     feature(generic_const_exprs)
    /// )]
    ///
    /// use phantom_newtype::Id;
    /// enum User {}
    /// type UserId = Id<User, u64>;
    ///
    /// const ADMIN_ID: UserId = UserId::new(42);
    ///
    /// assert_eq!(*ADMIN_ID.get(), 42);
    /// ```
    pub const fn new(repr: Repr) -> Id<TF, Entity, Repr> {
        Id(repr, PhantomData)
    }
}

#[cfg_attr(feature = "unstable_generic_const_own_type", allow(deprecated))]
impl<const TF: TraitFlags, Entity, Repr> Id<TF, Entity, Repr>
where
    Entity: DisplayerOf<Self>,
{
    /// `display` provides a mechanism to implement a custom display
    /// for phantom types.
    ///
    /// ```
    /// #![cfg_attr(
    ///     feature = "unstable_generic_const_own_type",
    ///     feature(generic_const_exprs),
    /// )]
    ///
    /// use phantom_newtype::DisplayerOf;
    /// use core::fmt;
    ///
    /// enum Message {}
    /// struct YearUnit;
    /// // This causes ICE (with feature `unstable_generic_const_own_type`), see https://github.com/rust-lang/rust/issues/134044:
    /// #[cfg(not(feature = "unstable_generic_const_own_type"))]
    /// type MessageId = phantom_newtype::Id<Message, [u8; 32]>;
    /// // No ICE:
    /// #[cfg(feature = "unstable_generic_const_own_type")]
    /// type MessageId = phantom_newtype::IdForFlags<{phantom_newtype::trait_flag::TRAIT_FLAGS_NO_COPY_NO_DEFAULT}, Message, [u8; 32]>;
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

#[cfg_attr(feature = "unstable_generic_const_own_type", allow(deprecated))]
impl<const TF: TraitFlags, Entity, Repr: Clone> Clone for Id<TF, Entity, Repr> {
    fn clone(&self) -> Self {
        Self::from(self.get().clone())
    }
}

#[cfg_attr(feature = "unstable_generic_const_own_type", allow(deprecated))]
impl<Entity, Repr: Copy> Copy for Id<{ trait_flag::TRAIT_FLAGS_IS_COPY_IS_DEFAULT }, Entity, Repr> {}
#[cfg_attr(feature = "unstable_generic_const_own_type", allow(deprecated))]
impl<Entity, Repr: Copy> Copy for Id<{ trait_flag::TRAIT_FLAGS_IS_COPY_NO_DEFAULT }, Entity, Repr> {}

#[cfg_attr(feature = "unstable_generic_const_own_type", allow(deprecated))]
impl<Unit, Repr: Default> Default
    for Id<{ trait_flag::TRAIT_FLAGS_IS_COPY_IS_DEFAULT }, Unit, Repr>
{
    fn default() -> Self {
        Self(Default::default(), PhantomData)
    }
}
#[cfg_attr(feature = "unstable_generic_const_own_type", allow(deprecated))]
impl<Unit, Repr: Default> Default
    for Id<{ trait_flag::TRAIT_FLAGS_NO_COPY_IS_DEFAULT }, Unit, Repr>
{
    fn default() -> Self {
        Self(Default::default(), PhantomData)
    }
}

#[cfg_attr(feature = "unstable_generic_const_own_type", allow(deprecated))]
impl<const TF: TraitFlags, Entity, Repr: PartialEq> PartialEq for Id<TF, Entity, Repr> {
    fn eq(&self, rhs: &Self) -> bool {
        self.get().eq(&rhs.get())
    }
}

#[cfg_attr(feature = "unstable_generic_const_own_type", allow(deprecated))]
impl<const TF: TraitFlags, Entity, Repr: PartialOrd> PartialOrd for Id<TF, Entity, Repr> {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        self.get().partial_cmp(&rhs.get())
    }
}

#[cfg_attr(feature = "unstable_generic_const_own_type", allow(deprecated))]
impl<const TF: TraitFlags, Entity, Repr: Ord> Ord for Id<TF, Entity, Repr> {
    fn cmp(&self, rhs: &Self) -> Ordering {
        self.get().cmp(&rhs.get())
    }
}

#[cfg_attr(feature = "unstable_generic_const_own_type", allow(deprecated))]
impl<const TF: TraitFlags, Entity, Repr: Hash> Hash for Id<TF, Entity, Repr> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.get().hash(state)
    }
}

#[cfg_attr(feature = "unstable_generic_const_own_type", allow(deprecated))]
impl<const TF: TraitFlags, Entity, Repr> From<Repr> for Id<TF, Entity, Repr> {
    fn from(repr: Repr) -> Self {
        Self::new(repr)
    }
}

#[cfg_attr(feature = "unstable_generic_const_own_type", allow(deprecated))]
impl<const TF: TraitFlags, Entity, Repr: Eq> Eq for Id<TF, Entity, Repr> {}

#[cfg_attr(feature = "unstable_generic_const_own_type", allow(deprecated))]
impl<const TF: TraitFlags, Entity, Repr: fmt::Debug> fmt::Debug for Id<TF, Entity, Repr> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.get())
    }
}

#[cfg_attr(feature = "unstable_generic_const_own_type", allow(deprecated))]
impl<const TF: TraitFlags, Entity, Repr: fmt::Display> fmt::Display for Id<TF, Entity, Repr> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get())
    }
}

#[cfg(feature = "serde")]
#[cfg_attr(feature = "unstable_generic_const_own_type", allow(deprecated))]
impl<const TF: TraitFlags, Entity, Repr> Serialize for Id<TF, Entity, Repr>
where
    Repr: Serialize,
{
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.get().serialize(serializer)
    }
}

#[cfg(feature = "serde")]
#[cfg_attr(feature = "unstable_generic_const_own_type", allow(deprecated))]
impl<'de, const TF: TraitFlags, Entity, Repr> Deserialize<'de> for Id<TF, Entity, Repr>
where
    Repr: Deserialize<'de>,
{
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Repr::deserialize(deserializer).map(Self::from)
    }
}
