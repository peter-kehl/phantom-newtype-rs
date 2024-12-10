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

use core::marker::PhantomData;

/// Flag-gated:
/// - Default
/// - Copy
/// - Display, ToString

/// Indicator trait that activates a blanket `impl` of [To].
///
/// This can't activate any blanket `impl` of [core::ops::Deref], because anything like the
/// following fails to compile:
/// ```compile_fail
/// impl<T, Repr, O> Deref for Amm<T, Repr>
/// where
/// Self: As<O> {
/// /// ...
/// }
/// ```
pub trait As<T> {}
pub trait AsMut<T> {}
pub trait AsFrom<T> {}
pub trait AsFromMut<T> {}

#[derive(Copy, Clone)]
pub struct Amm<T: Copy, Repr: Copy>(PhantomData<core::sync::atomic::AtomicPtr<T>>, Repr);

#[cfg(feature = "unstable_transmute_unchecked")]
pub const unsafe fn transmute_unchecked<T, U>(x: T) -> U {
    core::intrinsics::transmute_unchecked(x)
}

#[cfg(not(feature = "unstable_transmute_unchecked"))]
/// Thanks to Helix (noop_noob).
pub const unsafe fn transmute_unchecked<T, U>(x: T) -> U {
    use core::mem::ManuallyDrop;

    union Transmuter<T, U> {
        val: ManuallyDrop<T>,
        result: ManuallyDrop<U>,
    }
    ManuallyDrop::into_inner(
        Transmuter {
            val: ManuallyDrop::new(x),
        }
        .result,
    )
}

pub trait To<O: Copy, Repr: Copy> {
    fn to(self) -> Amm<O, Repr>;
    fn to_ref(&self) -> &Amm<O, Repr>;
}
pub trait ToMut<O: Copy, Repr: Copy> {
    fn to_mut(&mut self) -> &mut Amm<O, Repr>;
}
impl<T: Copy, Repr: Copy, O: Copy> To<O, Repr> for Amm<T, Repr>
where
    Self: As<O>, // this doesn't help: ,Repr: Sized
{
    fn to(self) -> Amm<O, Repr> {
        unsafe { transmute_unchecked(self) }
    }
    fn to_ref(&self) -> &Amm<O, Repr> {
        unsafe { transmute_unchecked(self) }
    }
}
impl<T: Copy, Repr: Copy, O: Copy> ToMut<O, Repr> for Amm<T, Repr>
where
    Self: AsMut<O>, // this doesn't help: ,Repr: Sized
{
    fn to_mut(&mut self) -> &mut Amm<O, Repr> {
        unsafe { transmute_unchecked(self) }
    }
}

/// This trait doesn't have a generic parameter indicating the type we're transforming from.
/// However, it has "From" in its name, because it's related to [AsFrom].
pub trait ToFrom<O: Copy, Repr: Copy> {
    fn to(self) -> Amm<O, Repr>;
    fn to_ref(&self) -> &Amm<O, Repr>;
}
pub trait ToFromMut<O: Copy, Repr: Copy> {
    fn to_mut(&mut self) -> &mut Amm<O, Repr>;
}
impl<T: Copy, Repr: Copy, O: Copy> ToFrom<O, Repr> for Amm<T, Repr>
where
    Amm<O, Repr>: AsFrom<T>,
{
    fn to(self) -> Amm<O, Repr> {
        unsafe { transmute_unchecked(self) }
    }
    fn to_ref(&self) -> &Amm<O, Repr> {
        unsafe { transmute_unchecked(self) }
    }
}
impl<T: Copy, Repr: Copy, O: Copy> ToFromMut<O, Repr> for Amm<T, Repr>
where
    Amm<O, Repr>: AsFromMut<T>,
{
    fn to_mut(&mut self) -> &mut Amm<O, Repr> {
        unsafe { transmute_unchecked(self) }
    }
}

// USERLAND:
#[derive(Clone, Copy)]
pub struct In;
#[derive(Clone, Copy)]
pub struct Out;
#[derive(Clone, Copy)]
pub struct Out2;

/// Indicate/activate the blanket impl.
impl<UNIT: Copy> As<Out> for Amm<In, UNIT> {}
impl<UNIT: Copy> As<Out2> for Amm<In, UNIT> {}

pub fn in_to_out_f32(inp: Amm<In, f32>) -> Amm<Out, f32> {
    let inp2 = inp;
    let imp2: Amm<Out2, _> = inp2.to();
    let inp3 = inp2.to() as Amm<Out, _>;
    // @TODO consider:
    //
    // let inp3 =  Amm<Out, _>::fr(inp2);

    /// the above `impl` automatically enables this:
    inp.to()
}

/// Indicate/activate the blanket impl.
impl<PROPERTY: Copy, UNIT: Copy> AsFrom<(In, PROPERTY)> for Amm<(Out, PROPERTY), UNIT> {}

pub fn in_to_out_f64<PROPERTY: Copy>(inp: Amm<(In, PROPERTY), f64>) -> Amm<(Out, PROPERTY), f64> {
    /// the above `impl` automatically enables this:
    inp.to()
}
pub fn in_to_out<PROPERTY: Copy, UNIT: Copy>(
    inp: Amm<(In, PROPERTY), UNIT>,
) -> Amm<(Out, PROPERTY), UNIT> {
    /// the above `impl` automatically enables this:
    inp.to()
}
