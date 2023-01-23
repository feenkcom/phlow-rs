use crate::AnyValue;
use std::fmt::{Debug, Display, Formatter, Result};
use std::rc::Rc;

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
pub fn get_debug_fmt_fn<T>() -> Option<DebugFmtFn> {
    trait Detect {
        fn fmt_fn() -> Option<DebugFmtFn>;
    }
    impl<T> Detect for T {
        default fn fmt_fn() -> Option<DebugFmtFn> {
            None
        }
    }
    impl<T> Detect for T
    where
        T: Debug + 'static,
    {
        fn fmt_fn() -> Option<DebugFmtFn> {
            Some(Rc::new(|value: &AnyValue, f: &mut Formatter<'_>| {
                if let Some(reference) = value.as_ref_safe::<T>() {
                    <Self as Debug>::fmt(reference, f)
                } else {
                    Ok(())
                }
            }))
        }
    }
    <T as Detect>::fmt_fn()
}
#[cfg(not(feature = "printing"))]
pub fn get_debug_fmt_fn<T>() -> Option<DebugFmtFn> {
    None
}

pub(crate) type DisplayFmtFn = Rc<dyn Fn(&AnyValue, &mut Formatter) -> Result>;
#[cfg(feature = "printing")]
pub fn get_display_fmt_fn<T>() -> Option<DisplayFmtFn> {
    trait Detect {
        fn fmt_fn() -> Option<DisplayFmtFn>;
    }
    impl<T> Detect for T {
        default fn fmt_fn() -> Option<DisplayFmtFn> {
            None
        }
    }
    impl<T> Detect for T
    where
        T: Display + 'static,
    {
        fn fmt_fn() -> Option<DisplayFmtFn> {
            Some(Rc::new(|value: &AnyValue, f: &mut Formatter<'_>| {
                if let Some(reference) = value.as_ref_safe::<T>() {
                    <Self as Display>::fmt(reference, f)
                } else {
                    Ok(())
                }
            }))
        }
    }
    <T as Detect>::fmt_fn()
}

#[cfg(not(feature = "printing"))]
pub fn get_display_fmt_fn<T>() -> Option<DisplayFmtFn> {
    None
}
