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

use crate::amount::Amount;
use crate::displayer::{DisplayProxy, DisplayerOf};
#[cfg_attr(feature = "unstable_generic_const_own_type", allow(deprecated))]
use crate::trait_flag::{self, TraitFlags};
use core::cmp::Ordering;
use core::fmt;
use core::hash::{Hash, Hasher};
use core::marker::PhantomData;
use core::ops::{Add, AddAssign, Div, Mul, MulAssign, Sub, SubAssign};
#[cfg(feature = "serde")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// `Instant<Unit>` provides a type-safe way to keep absolute time of
/// some events, expressed in `Unit`s (CPU ticks, seconds from epoch,
/// years from birth, etc).
///
/// You can compare instants:
///
/// ```
/// #![cfg_attr(
///     feature = "unstable_generic_const_own_type",
///     feature(generic_const_exprs)
/// )]
///
/// use phantom_newtype::Instant;
///
/// enum SecondsFromEpoch {}
/// type UnixTime = Instant<SecondsFromEpoch, i64>;
///
/// assert_eq!(true, UnixTime::from(3) < UnixTime::from(5));
/// assert_eq!(false, UnixTime::from(3) > UnixTime::from(5));
/// assert_eq!(true, UnixTime::from(3) != UnixTime::from(5));
/// assert_eq!(true, UnixTime::from(5) == UnixTime::from(5));
/// assert_eq!(false, UnixTime::from(5) != UnixTime::from(5));
///
/// assert_eq!(vec![UnixTime::from(3), UnixTime::from(5)].iter().max().unwrap(),
///            &UnixTime::from(5));
/// ```
///
/// Instants support basic arithmetics, you can:
/// * Subtract an instant from another instant to get amount of units between them.
/// * Add/subtract amount of units to/from an instant to get another instant.
///
/// ```
/// #![cfg_attr(
///     feature = "unstable_generic_const_own_type",
///     feature(generic_const_exprs)
/// )]
///
/// use phantom_newtype::{Amount, Instant};
///
/// enum SecondsFromEpoch {}
///
/// type UnixTime = Instant<SecondsFromEpoch, i64>;
/// type TimeDiff = Amount<SecondsFromEpoch, i64>;
///
/// let epoch = UnixTime::from(0);
/// let some_date = UnixTime::from(123456789);
/// let diff = TimeDiff::from(123456789);
///
/// assert_eq!(some_date - epoch, diff);
/// assert_eq!(some_date - diff, epoch);
/// assert_eq!(epoch + diff, some_date);
/// ```
///
/// Direct multiplication of instants is not supported, however, you
/// can scale them by a scalar or divide to get a scalar back:
///
/// ```
/// #![cfg_attr(
///     feature = "unstable_generic_const_own_type",
///     feature(generic_const_exprs)
/// )]
///
/// use phantom_newtype::Instant;
///
/// enum SecondsFromEpoch {}
/// type UnixTime = Instant<SecondsFromEpoch, i64>;
///
/// let x = UnixTime::from(123456);
/// assert_eq!(x * 3, UnixTime::from(3 * 123456));
/// assert_eq!(1, x / x);
/// assert_eq!(3, (x * 3) / x);
/// ```
///
/// Note that the unit is only available at compile time, thus using
/// `Instant` instead of `u64` doesn't incur any runtime penalty:
///
/// ```
/// #![cfg_attr(
///     feature = "unstable_generic_const_own_type",
///     feature(generic_const_exprs)
/// )]
///
/// use phantom_newtype::Instant;
///
/// enum SecondsFromEpoch {}
///
/// let ms = Instant::<SecondsFromEpoch, u64>::from(10);
/// assert_eq!(core::mem::size_of_val(&ms), core::mem::size_of::<u64>());
/// ```
///
/// Instants can be serialized and deserialized with `serde`. Serialized
/// forms of `Instant<Unit, Repr>` and `Repr` are identical.
///
/// ```
/// #![cfg_attr(
///     feature = "unstable_generic_const_own_type",
///     feature(generic_const_exprs),
/// )]
///
/// #[cfg(feature = "serde")] {
/// use phantom_newtype::Instant;
/// use serde::{Serialize, Deserialize};
/// use serde_json;
///
/// enum SecondsFromEpoch {}
/// type UnixTime = Instant<SecondsFromEpoch, u64>;
///
/// let repr: u64 = 123456;
/// let time = UnixTime::from(repr);
/// assert_eq!(serde_json::to_string(&time).unwrap(), serde_json::to_string(&repr).unwrap());
///
/// let copy: UnixTime = serde_json::from_str(&serde_json::to_string(&time).unwrap()).unwrap();
/// assert_eq!(copy, time);
/// }
/// ```
///
/// You can also declare constants of `Instant<Unit, Repr>` using `new`
/// function:
/// ```
/// #![cfg_attr(
///     feature = "unstable_generic_const_own_type",
///     feature(generic_const_exprs)
/// )]
///
/// use phantom_newtype::Instant;
///
/// enum SecondsFromEpoch {}
/// type UnixTime = Instant<SecondsFromEpoch, u64>;
///
/// const EPOCH: UnixTime = UnixTime::new(0);
/// ```
///
/// Instants can be sent between threads if the `Repr` allows it, no
/// matter which `Unit` is used.
///
/// ```
/// #![cfg_attr(
///     feature = "unstable_generic_const_own_type",
///     feature(generic_const_exprs)
/// )]
///
/// use phantom_newtype::Instant;
///
/// type Cell = core::cell::RefCell<i64>;
/// type CellInstant = Instant<Cell, i64>;
/// const I: CellInstant = CellInstant::new(1234);
///
/// let instant_from_thread = std::thread::spawn(|| &I).join().unwrap();
/// assert_eq!(I, *instant_from_thread);
/// ```
#[cfg_attr(feature = "unstable_generic_const_own_type", allow(deprecated))]
#[repr(transparent)]
//`pub struct Instant<Unit, Repr>(Repr, PhantomData<*const Unit>);
//pub struct Instant<Unit, Repr>(Repr, PhantomData<core::sync::Exclusive<Unit>>);
pub struct Instant<const TF: TraitFlags, Unit, Repr>(
    Repr,
    PhantomData<core::sync::atomic::AtomicPtr<Unit>>,
);
#[cfg_attr(feature = "unstable_generic_const_own_type", allow(deprecated))]
impl<const TF: TraitFlags, Unit, Repr: Copy> Instant<TF, Unit, Repr> {
    // @TODO

    /// Returns the wrapped value.
    ///
    /// ```
    /// #![cfg_attr(
    ///     feature = "unstable_generic_const_own_type",
    ///     feature(generic_const_exprs)
    /// )]
    ///
    /// use phantom_newtype::Instant;
    ///
    /// enum Apples {}
    ///
    /// let three_apples = Instant::<Apples, u64>::from(3);
    /// assert_eq!(9, (three_apples * 3).get());
    /// ```
    pub fn get(&self) -> Repr {
        self.0
    }
}

#[cfg_attr(feature = "unstable_generic_const_own_type", allow(deprecated))]
impl<const TF: TraitFlags, Unit, Repr> Instant<TF, Unit, Repr> {
    /// `new` is a synonym for `from` that can be evaluated in
    /// compile time. The main use-case of this functions is defining
    /// constants.
    pub const fn new(repr: Repr) -> Instant<TF, Unit, Repr> {
        Instant(repr, PhantomData)
    }
}

#[cfg_attr(feature = "unstable_generic_const_own_type", allow(deprecated))]
impl<const TF: TraitFlags, Unit: Default, Repr> Instant<TF, Unit, Repr> {
    /// Provides a useful shortcut to access units of an instant if
    /// they implement the `Default` trait:
    ///
    /// ```
    /// #![cfg_attr(
    ///     feature = "unstable_generic_const_own_type",
    ///     feature(generic_const_exprs)
    /// )]
    ///
    /// use phantom_newtype::Instant;
    ///
    /// #[derive(Debug, Default)]
    /// struct SecondsFromEpoch;
    /// let when = Instant::<SecondsFromEpoch, i64>::from(5);
    ///
    /// assert_eq!("5 SecondsFromEpoch", format!("{} {:?}", when, when.unit()));
    /// ```
    pub fn unit(&self) -> Unit {
        Default::default()
    }
}

#[cfg_attr(feature = "unstable_generic_const_own_type", allow(deprecated))]
impl<const TF: TraitFlags, Unit, Repr> Instant<TF, Unit, Repr>
where
    Unit: DisplayerOf<Instant<TF, Unit, Repr>>,
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
    /// struct YearUnit;
    /// // This causes ICE (with feature `unstable_generic_const_own_type`), see https://github.com/rust-lang/rust/issues/134044:
    /// #[cfg(not(feature = "unstable_generic_const_own_type"))]
    /// type YearAD = phantom_newtype::Instant<YearUnit, u64>;
    /// // No ICE:
    /// #[cfg(feature = "unstable_generic_const_own_type")]
    /// type YearAD = phantom_newtype::InstantForFlags<{phantom_newtype::trait_flag::TRAIT_FLAGS_IS_COPY_IS_DEFAULT}, YearUnit, u64>;
    ///
    /// impl DisplayerOf<YearAD> for YearUnit {
    ///   fn display(year: &YearAD, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    ///     write!(f, "{} AD", year.get())
    ///   }
    /// }
    ///
    /// assert_eq!(format!("{}", YearAD::from(1221).display()), "1221 AD");
    /// ```
    pub fn display(&self) -> DisplayProxy<'_, Self, Unit> {
        DisplayProxy::new(self)
    }
}

#[cfg_attr(feature = "unstable_generic_const_own_type", allow(deprecated))]
impl<const TF: TraitFlags, Unit, Repr> From<Repr> for Instant<TF, Unit, Repr> {
    fn from(repr: Repr) -> Self {
        Self::new(repr)
    }
}

#[cfg_attr(feature = "unstable_generic_const_own_type", allow(deprecated))]
impl<const TF: TraitFlags, Unit, Repr: Clone> Clone for Instant<TF, Unit, Repr> {
    fn clone(&self) -> Self {
        Instant(self.0.clone(), PhantomData)
    }
}

#[cfg_attr(feature = "unstable_generic_const_own_type", allow(deprecated))]
impl<Unit, Repr: Copy> Copy
    for Instant<{ trait_flag::TRAIT_FLAGS_IS_COPY_IS_DEFAULT }, Unit, Repr>
{
}
#[cfg_attr(feature = "unstable_generic_const_own_type", allow(deprecated))]
impl<Unit, Repr: Copy> Copy
    for Instant<{ trait_flag::TRAIT_FLAGS_IS_COPY_NO_DEFAULT }, Unit, Repr>
{
}

#[cfg_attr(feature = "unstable_generic_const_own_type", allow(deprecated))]
impl<Unit, Repr: Default> Default
    for Instant<{ trait_flag::TRAIT_FLAGS_IS_COPY_IS_DEFAULT }, Unit, Repr>
{
    fn default() -> Self {
        Self(Default::default(), PhantomData)
    }
}
#[cfg_attr(feature = "unstable_generic_const_own_type", allow(deprecated))]
impl<Unit, Repr: Default> Default
    for Instant<{ trait_flag::TRAIT_FLAGS_NO_COPY_IS_DEFAULT }, Unit, Repr>
{
    fn default() -> Self {
        Self(Default::default(), PhantomData)
    }
}

#[cfg_attr(feature = "unstable_generic_const_own_type", allow(deprecated))]
impl<const TF: TraitFlags, Unit, Repr: PartialEq> PartialEq for Instant<TF, Unit, Repr> {
    fn eq(&self, rhs: &Self) -> bool {
        self.0.eq(&rhs.0)
    }
}

#[cfg_attr(feature = "unstable_generic_const_own_type", allow(deprecated))]
impl<const TF: TraitFlags, Unit, Repr: Eq> Eq for Instant<TF, Unit, Repr> {}

#[cfg_attr(feature = "unstable_generic_const_own_type", allow(deprecated))]
impl<const TF: TraitFlags, Unit, Repr: PartialOrd> PartialOrd for Instant<TF, Unit, Repr> {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&rhs.0)
    }
}

#[cfg_attr(feature = "unstable_generic_const_own_type", allow(deprecated))]
impl<const TF: TraitFlags, Unit, Repr: Ord> Ord for Instant<TF, Unit, Repr> {
    fn cmp(&self, rhs: &Self) -> Ordering {
        self.0.cmp(&rhs.0)
    }
}

#[cfg_attr(feature = "unstable_generic_const_own_type", allow(deprecated))]
impl<const TF: TraitFlags, Unit, Repr: Hash> Hash for Instant<TF, Unit, Repr> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state)
    }
}

#[cfg_attr(feature = "unstable_generic_const_own_type", allow(deprecated))]
impl<const TF: TraitFlags, Unit, Repr, Repr2> Add<Amount<TF, Unit, Repr2>>
    for Instant<TF, Unit, Repr>
where
    Repr: AddAssign<Repr2> + Copy,
    Repr2: Copy,
{
    type Output = Self;
    fn add(mut self, rhs: Amount<TF, Unit, Repr2>) -> Self {
        self.add_assign(rhs);
        self
    }
}

#[cfg_attr(feature = "unstable_generic_const_own_type", allow(deprecated))]
impl<const TF: TraitFlags, Unit, Repr, Repr2> AddAssign<Amount<TF, Unit, Repr2>>
    for Instant<TF, Unit, Repr>
where
    Repr: AddAssign<Repr2> + Copy,
    Repr2: Copy,
{
    fn add_assign(&mut self, rhs: Amount<TF, Unit, Repr2>) {
        self.0 += rhs.get()
    }
}

#[cfg_attr(feature = "unstable_generic_const_own_type", allow(deprecated))]
impl<const TF: TraitFlags, Unit, Repr, Repr2> SubAssign<Amount<TF, Unit, Repr2>>
    for Instant<TF, Unit, Repr>
where
    Repr: SubAssign<Repr2> + Copy,
    Repr2: Copy,
{
    fn sub_assign(&mut self, rhs: Amount<TF, Unit, Repr2>) {
        self.0 -= rhs.get()
    }
}

#[cfg_attr(feature = "unstable_generic_const_own_type", allow(deprecated))]
impl<const TF: TraitFlags, Unit, Repr> Sub for Instant<TF, Unit, Repr>
where
    Repr: Sub + Copy,
{
    type Output = Amount<TF, Unit, <Repr as Sub>::Output>;

    fn sub(self, rhs: Self) -> Self::Output {
        Amount::<TF, Unit, <Repr as Sub>::Output>::new(self.0 - rhs.0)
    }
}

#[cfg_attr(feature = "unstable_generic_const_own_type", allow(deprecated))]
impl<const TF: TraitFlags, Unit, Repr, Repr2> Sub<Amount<TF, Unit, Repr2>>
    for Instant<TF, Unit, Repr>
where
    Repr: SubAssign<Repr2> + Copy,
    Repr2: Copy,
{
    type Output = Self;

    fn sub(mut self, rhs: Amount<TF, Unit, Repr2>) -> Self {
        self.sub_assign(rhs);
        self
    }
}

#[cfg_attr(feature = "unstable_generic_const_own_type", allow(deprecated))]
impl<const TF: TraitFlags, Unit, Repr> MulAssign<Repr> for Instant<TF, Unit, Repr>
where
    Repr: MulAssign + Copy,
{
    fn mul_assign(&mut self, rhs: Repr) {
        self.0 *= rhs;
    }
}

#[cfg_attr(feature = "unstable_generic_const_own_type", allow(deprecated))]
impl<const TF: TraitFlags, Unit, Repr> Mul<Repr> for Instant<TF, Unit, Repr>
where
    Repr: MulAssign + Copy,
{
    type Output = Self;

    fn mul(mut self, rhs: Repr) -> Self {
        self.mul_assign(rhs);
        self
    }
}

#[cfg_attr(feature = "unstable_generic_const_own_type", allow(deprecated))]
impl<const TF: TraitFlags, Unit, Repr> Div<Self> for Instant<TF, Unit, Repr>
where
    Repr: Div<Repr> + Copy,
{
    type Output = <Repr as Div>::Output;

    fn div(self, rhs: Self) -> Self::Output {
        self.0.div(rhs.0)
    }
}

#[cfg_attr(feature = "unstable_generic_const_own_type", allow(deprecated))]
impl<const TF: TraitFlags, Unit, Repr> fmt::Debug for Instant<TF, Unit, Repr>
where
    Repr: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[cfg_attr(feature = "unstable_generic_const_own_type", allow(deprecated))]
impl<const TF: TraitFlags, Unit, Repr> fmt::Display for Instant<TF, Unit, Repr>
where
    Repr: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[cfg(feature = "serde")]
#[cfg_attr(feature = "unstable_generic_const_own_type", allow(deprecated))]
impl<const TF: TraitFlags, Unit, Repr: Serialize> Serialize for Instant<TF, Unit, Repr> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.0.serialize(serializer)
    }
}

#[cfg(feature = "serde")]
#[cfg_attr(feature = "unstable_generic_const_own_type", allow(deprecated))]
impl<'de, const TF: TraitFlags, Unit, Repr> Deserialize<'de> for Instant<TF, Unit, Repr>
where
    Repr: Deserialize<'de>,
{
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Repr::deserialize(deserializer).map(Self::new)
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_complex_instant_arithmetics() {
        enum Seconds {}
        enum UTC {}

        type Timestamp = Instant<Seconds, i64>;
        type TsDiff = Amount<Seconds, i64>;
        type Date = Instant<UTC, Timestamp>;

        let epoch = Date::new(Timestamp::new(0));
        let date = Date::new(Timestamp::new(123456789));
        let span = Amount::<UTC, TsDiff>::new(TsDiff::from(123456789));

        assert_eq!(date - epoch, span);
        assert_eq!(date - span, epoch);
        assert_eq!(epoch + span, date);
    }
}
