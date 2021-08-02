use std::{
    fmt,
    marker::PhantomData,
    num::NonZeroUsize,
    ops::Deref,
    rc::Rc,
};

// incr, decr must not cause memory problems, like dangling pointers, use after free
unsafe trait RefCounted<T>
where
    T: ?Sized,
{
    type Pointer: Copy;

    fn increment(pointer: Self::Pointer);
    fn decrement(pointer: Self::Pointer);

    fn count(pointer: Self::Pointer) -> Option<NonZeroUsize> {
        None
    }

    fn deref<'s>(pointer: Self::Pointer) -> &'s T;
}

struct Intrusive<T, R>
where
    R: RefCounted<T>,
    T: ?Sized,
{
    pointer: R::Pointer,
    marker: PhantomData<Rc<T>>,
}

impl<T, R> Intrusive<T, R>
where
    R: RefCounted<T>,
    T: ?Sized,
{
    pub unsafe fn adopt(pointer: R::Pointer) -> Self {
        Self {
            pointer,
            marker: PhantomData,
        }
    }

    pub unsafe fn retain(pointer: R::Pointer) -> Self {
        R::increment(pointer);
        Self::adopt(pointer)
    }
}

impl<T, R> Clone for Intrusive<T, R>
where
    R: RefCounted<T>,
    T: ?Sized,
{
    fn clone(&self) -> Self {
        unsafe { Self::retain(self.pointer) }
    }
}

impl<T, R> Deref for Intrusive<T, R>
where
    R: RefCounted<T>,
    T: ?Sized,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        R::deref(self.pointer)
    }
}

impl<T, R> fmt::Display for Intrusive<T, R>
where
    R: RefCounted<T>,
    T: ?Sized + fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&**self, f)
    }
}

impl<T, R> fmt::Debug for Intrusive<T, R>
where
    R: RefCounted<T>,
    T: ?Sized + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&**self, f)
    }
}

impl<T, R> fmt::Pointer for Intrusive<T, R>
where
    R: RefCounted<T>,
    T: ?Sized,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Pointer::fmt(&(&**self as *const T), f)
    }
}
