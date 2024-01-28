use std::{
    marker::PhantomData,
    borrow::Borrow,
    path::{Path, PathBuf},
    ffi::{OsStr, OsString},
};

/// Maps a (potentially unsized) type to it's sized representation.
///
/// This trait is similar to the [`ToOwned`] trait except that it does not require the value to be
/// [`Clone`].
pub trait SizedRepresentation {
    /// Sized representation of the value.
    type Sized: Borrow<Self>;
}

impl<T> SizedRepresentation for T {
    type Sized = T;
}

impl SizedRepresentation for str {
    type Sized = String;
}

impl<T> SizedRepresentation for [T] {
    type Sized = Vec<T>;
}

impl SizedRepresentation for Path {
    type Sized = PathBuf;
}

impl SizedRepresentation for OsStr {
    type Sized = OsString;
}

/// Storage kind for values.
pub trait Storage {
    /// Type used to store a particular value.
    type Type<T: 'static + SizedRepresentation + ?Sized>: Borrow<T>;
}

/// Store the value inline.
pub struct Inline;

/// Store the value in a heap allocation.
pub struct Box;

/// Store the value in an atomic reference-counted heap allocation [`Arc`][std::sync::Arc].
pub struct Arc;

/// Store the value in a reference-counted heap allocation.
pub struct Rc;

/// Store the value as a reference.
pub struct Ref<'a>(PhantomData<&'a ()>);

impl Storage for Inline {
    type Type<T: 'static + SizedRepresentation + ?Sized> = T::Sized;
}

impl Storage for Box {
    type Type<T: 'static + SizedRepresentation + ?Sized> = std::boxed::Box<T>;
}

impl Storage for Arc {
    type Type<T: 'static + SizedRepresentation + ?Sized> = std::sync::Arc<T>;
}

impl Storage for Rc {
    type Type<T: 'static + SizedRepresentation + ?Sized> = std::rc::Rc<T>;
}

impl<'b> Storage for Ref<'b> {
    type Type<T: 'static + SizedRepresentation + ?Sized> = &'b T;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Debug)]
    struct MyData<S: Storage> {
        string: S::Type<str>,
        set: std::collections::BTreeSet<S::Type<u64>>,
    }

    type MyDataBoxed = MyData<Box>;
    type MyDataArc = MyData<Arc>;
    type MyDataRef<'a> = MyData<Ref<'a>>;
}
