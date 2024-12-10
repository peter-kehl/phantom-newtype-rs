//! Unstable.

#[derive(Eq, PartialEq, PartialOrd, core::fmt::Debug)]
#[cfg_attr(
    feature = "unstable_generic_const_own_type",
    derive(core::marker::ConstParamTy)
)]
pub enum TraitFlagsValues {
    TRAIT_FLAGS_NO_COPY_NO_DEFAULT,
    TRAIT_FLAGS_IS_COPY_NO_DEFAULT,
    TRAIT_FLAGS_NO_COPY_IS_DEFAULT,
    TRAIT_FLAGS_IS_COPY_IS_DEFAULT,
}

/// Use for a const generic `TRAIT_FLAGS` parameter to indicate some optional functionality of
/// [Amount], [Id] or [Instant].
///
/// Do not hard code any values. Instead, use `TRAIT_FLAGS_*` constants (like
/// [TRAIT_FLAGS_IS_COPY_IS_DEFAULT]). Even better, use the type aliases like [Amount],
/// [AmountNoCopy], [AmountNoCopyNoDefault].
///
/// Subject to change. Once `#![feature(adt_const_params)]` becomes stable`:
/// [rust-lang/rust/issues/95174](https://github.com/rust-lang/rust/issues/95174), we switch to
/// using a proper struct here (which will derive [core::marker::ConstParamTy], derive/implement
/// [PartialEq] and [Eq]).
#[cfg(not(feature = "unstable_generic_const_own_type"))]
pub type TraitFlags = u8;
#[cfg(feature = "unstable_generic_const_own_type")]
/// `nightly` warning: Direct use of [IdForFlags] (and hence direct use of TraitFlags) is unstable!
#[deprecated(
    note = "`nightly` warning: Direct use of IdForFlags (and hence direct use of TraitFlags) is unstable!"
)]
pub type TraitFlags = TraitFlagsValues;

#[cfg_attr(feature = "unstable_generic_const_own_type", allow(deprecated))]
const fn trait_flags_new(tfv: TraitFlagsValues) -> TraitFlags {
    #[cfg(not(feature = "unstable_generic_const_own_type"))]
    {
        tfv as u8
    }
    #[cfg(feature = "unstable_generic_const_own_type")]
    {
        // See also https://doc.rust-lang.org/nightly/reference/items/enumerations.html#implicit-discriminants and https://doc.rust-lang.org/nightly/core/mem/fn.discriminant.html#accessing-the-numeric-value-of-the-discriminant
        tfv
    }
}

const TRAIT_FLAG_BIT_COPY: u8 = 0b1;
const TRAIT_FLAG_BIT_DEFAULT: u8 = 0b10;

#[cfg_attr(
    feature = "unstable_generic_const_own_type",
    deprecated(note = "`nightly` warning: Direct use is unstable!")
)]
#[cfg_attr(feature = "unstable_generic_const_own_type", allow(deprecated))]
pub const TRAIT_FLAGS_IS_COPY_IS_DEFAULT: TraitFlags =
    trait_flags_new(TraitFlagsValues::TRAIT_FLAGS_IS_COPY_IS_DEFAULT);
#[cfg_attr(
    feature = "unstable_generic_const_own_type",
    deprecated(note = "`nightly` warning: Direct use is unstable!")
)]
#[cfg_attr(feature = "unstable_generic_const_own_type", allow(deprecated))]
pub const TRAIT_FLAGS_IS_COPY_NO_DEFAULT: TraitFlags =
    trait_flags_new(TraitFlagsValues::TRAIT_FLAGS_IS_COPY_NO_DEFAULT);
#[cfg_attr(
    feature = "unstable_generic_const_own_type",
    deprecated(note = "`nightly` warning: Direct use is unstable!")
)]
#[cfg_attr(feature = "unstable_generic_const_own_type", allow(deprecated))]
pub const TRAIT_FLAGS_NO_COPY_IS_DEFAULT: TraitFlags =
    trait_flags_new(TraitFlagsValues::TRAIT_FLAGS_NO_COPY_IS_DEFAULT);
#[cfg_attr(
    feature = "unstable_generic_const_own_type",
    deprecated(note = "`nightly` warning: Direct use is unstable!")
)]
#[cfg_attr(feature = "unstable_generic_const_own_type", allow(deprecated))]
pub const TRAIT_FLAGS_NO_COPY_NO_DEFAULT: TraitFlags =
    trait_flags_new(TraitFlagsValues::TRAIT_FLAGS_NO_COPY_NO_DEFAULT);

#[cfg_attr(feature = "unstable_generic_const_own_type", allow(deprecated))]
const fn trait_flags_bits(tf: TraitFlags) -> u8 {
    #[cfg(not(feature = "unstable_generic_const_own_type"))]
    return tf;
    #[cfg(feature = "unstable_generic_const_own_type")]
    return tf as u8;
}

#[cfg_attr(feature = "unstable_generic_const_own_type", allow(deprecated))]
const fn is_copy(flags: TraitFlags) -> bool {
    trait_flags_bits(flags) & TRAIT_FLAG_BIT_COPY != 0
}
#[cfg_attr(feature = "unstable_generic_const_own_type", allow(deprecated))]
const fn is_default(flags: TraitFlags) -> bool {
    trait_flags_bits(flags) & TRAIT_FLAG_BIT_DEFAULT != 0
}
#[cfg(test)]
mod test_flags {
    extern crate std;
    use super::*;

    #[test]
    #[allow(deprecated)]
    fn all() {
        assert_eq!(is_copy(TRAIT_FLAGS_IS_COPY_IS_DEFAULT), true);
        assert_eq!(is_copy(TRAIT_FLAGS_IS_COPY_NO_DEFAULT), true);
        assert_eq!(is_copy(TRAIT_FLAGS_NO_COPY_IS_DEFAULT), false);
        assert_eq!(is_copy(TRAIT_FLAGS_NO_COPY_NO_DEFAULT), false);

        assert_eq!(is_default(TRAIT_FLAGS_IS_COPY_IS_DEFAULT), true);
        assert_eq!(is_default(TRAIT_FLAGS_NO_COPY_IS_DEFAULT), true);
        assert_eq!(is_default(TRAIT_FLAGS_IS_COPY_NO_DEFAULT), false);
        assert_eq!(is_default(TRAIT_FLAGS_NO_COPY_NO_DEFAULT), false);
    }
}

// Move to Amount, Instant:
// TODO
/*
/// Internal indicator trait. It signals that we implement [Default] for this type. That prevents
/// repetition of `impl Default` for various const generics.
trait ImplementDefault {}

/// Blanked implementation of [Default], indicated by [ImplementDefault].
impl<const TF: TraitFlags> Default for S2<TF>
where
    S2<TF>: ImplementDefault,
{
    fn default() -> Self {
        Self {}
    }
}
impl ImplementDefault for S2<TRAIT_FLAGS_IS_COPY_IS_DEFAULT> {}
impl ImplementDefault for S2<TRAIT_FLAGS_NO_COPY_IS_DEFAULT> {}
*/
