use std::{
    marker::PhantomData,
    borrow::Borrow,
};

/// Value that has a sized representation.
pub trait Value {
    /// Sized representation of the value.
    type Sized: Borrow<Self>;
}

impl<T> Value for T {
    type Sized = T;
}

impl Value for str {
    type Sized = String;
}

impl<T> Value for [T] {
    type Sized = Vec<T>;
}

/// Storage kind for values.
pub trait Storage {
    /// Type used to store a particular value.
    type Type<T: 'static + Value + ?Sized>: Borrow<T>;
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

/// Store the value as a mutable reference.
pub struct RefMut<'a>(PhantomData<&'a ()>);

impl Storage for Inline {
    type Type<T: 'static + Value + ?Sized> = T::Sized;
}

impl Storage for Box {
    type Type<T: 'static + Value + ?Sized> = std::boxed::Box<T>;
}

impl Storage for Arc {
    type Type<T: 'static + Value + ?Sized> = std::sync::Arc<T>;
}

impl Storage for Rc {
    type Type<T: 'static + Value + ?Sized> = std::rc::Rc<T>;
}

impl<'b> Storage for Ref<'b> {
    type Type<T: 'static + Value + ?Sized> = &'b T;
}

impl<'b> Storage for RefMut<'b> {
    type Type<T: 'static + Value + ?Sized> = &'b mut T;
}

#[derive(Clone, Debug)]
struct MyData<S: Storage> {
    string: S::Type<str>,
    set: std::collections::BTreeSet<S::Type<u64>>,
}

type MyDataBoxed = MyData<Box>;
type MyDataArc = MyData<Arc>;
type MyDataRef<'a> = MyData<Ref<'a>>;
