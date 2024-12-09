#[derive(Eq, PartialEq, PartialOrd, core::fmt::Debug)]
#[derive(core::marker::ConstParamTy)]
pub enum TraitFlags {
    TRAIT_FLAGS_NO_COPY_NO_DEFAULT,
    TRAIT_FLAGS_IS_COPY_NO_DEFAULT,
    TRAIT_FLAGS_NO_COPY_IS_DEFAULT,
    TRAIT_FLAGS_IS_COPY_IS_DEFAULT,
}


const TRAIT_FLAG_BIT_COPY: u8 = 0b1;
const TRAIT_FLAG_BIT_DEFAULT: u8 = 0b10;


const fn trait_flags_bits(tf: TraitFlags) -> u8 {
    tf as u8
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
        assert_eq!(is_copy(TraitFlags::TRAIT_FLAGS_IS_COPY_IS_DEFAULT), true);
        assert_eq!(is_copy(TraitFlags::TRAIT_FLAGS_IS_COPY_NO_DEFAULT), true);
        assert_eq!(is_copy(TraitFlags::TRAIT_FLAGS_NO_COPY_IS_DEFAULT), false);
        assert_eq!(is_copy(TraitFlags::TRAIT_FLAGS_NO_COPY_NO_DEFAULT), false);

        assert_eq!(is_default(TraitFlags::TRAIT_FLAGS_IS_COPY_IS_DEFAULT), true);
        assert_eq!(is_default(TraitFlags::TRAIT_FLAGS_NO_COPY_IS_DEFAULT), true);
        assert_eq!(is_default(TraitFlags::TRAIT_FLAGS_IS_COPY_NO_DEFAULT), false);
        assert_eq!(is_default(TraitFlags::TRAIT_FLAGS_NO_COPY_NO_DEFAULT), false);
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
