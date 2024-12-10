//! Unstable.

#[derive(Eq, PartialEq, PartialOrd, core::marker::ConstParamTy)]
pub enum TraitFlags {
    TRAIT_FLAGS_NO_COPY_NO_DEFAULT,
}

pub const TRAIT_FLAGS_NO_COPY_NO_DEFAULT: TraitFlags =
    TraitFlags::TRAIT_FLAGS_NO_COPY_NO_DEFAULT;
