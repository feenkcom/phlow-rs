use std::fmt::{Debug, Display, Formatter, Result};
use std::rc::Rc;
use crate::AnyValue;

pub struct Fmt<F>(pub F)
    where
        F: Fn(&mut Formatter) -> Result;

impl<F> Debug for Fmt<F>
    where
        F: Fn(&mut Formatter) -> Result,
{
    fn fmt(&self, f: &mut Formatter) -> Result {
        (self.0)(f)
    }
}

impl<F> Display for Fmt<F>
    where
        F: Fn(&mut Formatter) -> Result,
{
    fn fmt(&self, f: &mut Formatter) -> Result {
        (self.0)(f)
    }
}

pub(crate) type DebugFmtFn = Rc<dyn Fn(&AnyValue, &mut Formatter) -> Result>;

#[cfg(feature = "printing")]
pub fn get_debug_fmt_fn<T>(param: &T) -> Option<DebugFmtFn> {
    trait Detect {
        fn fmt_fn(&self) -> Option<DebugFmtFn>;
    }
    impl<T> Detect for T {
        default fn fmt_fn(&self) -> Option<DebugFmtFn> {
            None
        }
    }
    impl<T> Detect for T
        where
            T: Debug + 'static,
    {
        fn fmt_fn(&self) -> Option<DebugFmtFn> {
            Some(Rc::new(|value: &AnyValue, f: &mut Formatter<'_>| {
                <Self as Debug>::fmt(value.as_ref::<T>(), f)
            }))
        }
    }
    param.fmt_fn()
}
#[cfg(not(feature = "printing"))]
pub fn get_debug_fmt_fn<T>(param: &T) -> Option<DebugFmtFn> {
    None
}

pub(crate) type DisplayFmtFn = Rc<dyn Fn(&AnyValue, &mut Formatter) -> Result>;
#[cfg(feature = "printing")]
pub fn get_display_fmt_fn<T>(param: &T) -> Option<DisplayFmtFn> {
    trait Detect {
        fn fmt_fn(&self) -> Option<DisplayFmtFn>;
    }
    impl<T> Detect for T {
        default fn fmt_fn(&self) -> Option<DisplayFmtFn> {
            None
        }
    }
    impl<T> Detect for T
        where
            T: Display + 'static,
    {
        fn fmt_fn(&self) -> Option<DisplayFmtFn> {
            Some(Rc::new(|value: &AnyValue, f: &mut Formatter<'_>| {
                <Self as Display>::fmt(value.as_ref::<T>(), f)
            }))
        }
    }
    param.fmt_fn()
}

#[cfg(not(feature = "printing"))]
pub fn get_display_fmt_fn<T>(param: &T) -> Option<DisplayFmtFn> {
    None
}
