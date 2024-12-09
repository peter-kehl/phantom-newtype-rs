use core::fmt;
use core::marker::PhantomData;

pub trait DisplayerOf<T> {
    fn display(value: &T, f: &mut fmt::Formatter<'_>) -> fmt::Result;
}

pub struct DisplayProxy<'a, T, Displayer>
where
    Displayer: DisplayerOf<T>,
{
    value: &'a T,
    displayer_tag: PhantomData<Displayer>,
}

impl<'a, T, Displayer> DisplayProxy<'a, T, Displayer>
where
    Displayer: DisplayerOf<T>,
{
    pub fn new(value: &'a T) -> Self {
        Self {
            value,
            displayer_tag: PhantomData,
        }
    }
}

impl<'a, T, Displayer> fmt::Display for DisplayProxy<'a, T, Displayer>
where
    Displayer: DisplayerOf<T>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Displayer::display(&self.value, f)
    }
}
