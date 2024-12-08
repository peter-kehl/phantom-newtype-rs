// Copyright 2019 DFINITY
// Copyright 2024 Peter Lyons Kehl
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

#![no_std]
#![cfg_attr(
    any(
        feature = "unstable_generic_const_own_type",
        feature = "unstable_transmute_unchecked"
    ),
    allow(incomplete_features)
)]
#![cfg_attr(
    feature = "unstable_generic_const_own_type",
    feature(adt_const_params),
    feature(generic_const_exprs)
)]
#![cfg_attr(feature = "unstable_transmute_unchecked", feature(core_intrinsics))]

//#![feature(unsized_const_params)] // https://github.com/rust-lang/rust/issues/95174

mod amount;
mod displayer;
mod id;
mod instant;
//pub mod prelude;
//pub mod prelude_full;
//mod to;
//mod trait_flag;

pub use amount::Amount;
pub use displayer::{DisplayProxy, DisplayerOf};
pub use id::Id;
pub use instant::Instant;
