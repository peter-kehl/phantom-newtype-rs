#[cfg(feature = "unstable_generic_const_own_type")]
use core::marker::ConstParamTy;

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
#[derive(Eq, PartialEq, PartialOrd, ConstParamTy, core::fmt::Debug)]
pub struct TraitFlags(u8);

const fn trait_flags_new(tf: u8) -> TraitFlags {
    #[cfg(not(feature = "unstable_generic_const_own_type"))]
    {
        tf
    }
    #[cfg(feature = "unstable_generic_const_own_type")]
    {
        const MAX_FLAG_BITS: u8 = 0b11;
        if tf > MAX_FLAG_BITS {
            panic!("The parameter is higher than MAX_FLAG_BITS.");
        }

        TraitFlags(tf)
    }
}

const TRAIT_FLAG_BIT_COPY: u8 = 0b1;
const TRAIT_FLAG_BIT_DEFAULT: u8 = 0b10;

pub const TRAIT_FLAGS_IS_COPY_IS_DEFAULT: TraitFlags =
    trait_flags_new(TRAIT_FLAG_BIT_COPY | TRAIT_FLAG_BIT_DEFAULT);
pub const TRAIT_FLAGS_IS_COPY_NO_DEFAULT: TraitFlags = trait_flags_new(TRAIT_FLAG_BIT_COPY);
pub const TRAIT_FLAGS_NO_COPY_IS_DEFAULT: TraitFlags = trait_flags_new(TRAIT_FLAG_BIT_DEFAULT);
pub const TRAIT_FLAGS_NO_COPY_NO_DEFAULT: TraitFlags = trait_flags_new(0);

const fn trait_flags_bits(tf: TraitFlags) -> u8 {
    #[cfg(not(feature = "unstable_generic_const_own_type"))]
    return tf;
    #[cfg(feature = "unstable_generic_const_own_type")]
    return tf.0;
}

const fn is_copy(flags: TraitFlags) -> bool {
    trait_flags_bits(flags) & TRAIT_FLAG_BIT_COPY != 0
}
const fn is_default(flags: TraitFlags) -> bool {
    trait_flags_bits(flags) & TRAIT_FLAG_BIT_DEFAULT != 0
}
#[cfg(test)]
mod test_flags {
    extern crate std;
    use super::*;

    #[test]
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
#[derive(Clone)]
struct S2<const TF: TraitFlags> {}

impl Copy for S2<TRAIT_FLAGS_IS_COPY_IS_DEFAULT> {}
impl Copy for S2<TRAIT_FLAGS_IS_COPY_NO_DEFAULT> {}

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

pub fn s2_default() -> S2<TRAIT_FLAGS_IS_COPY_IS_DEFAULT> {
    let outp = Default::default();
    outp
}

pub fn s2_no_copy(inp: S2<TRAIT_FLAGS_NO_COPY_IS_DEFAULT>) -> S2<TRAIT_FLAGS_NO_COPY_IS_DEFAULT> {
    let outp = inp;
    outp
}
