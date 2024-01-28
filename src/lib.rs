//! # Tupperware
//!
//! This crate allows you to express polymorphism over how your data is stored.
//!
//!
use std::{
    marker::PhantomData,
    borrow::Borrow,
    path::{Path, PathBuf},
    ffi::{OsStr, OsString},
};

#[cfg(test)]
mod tests;

/// Maps a (potentially unsized) type to its sized representation.
///
/// This trait is similar to the [`ToOwned`] trait except that it does not require the value to be
/// [`Clone`]. It only requires that the sized type implements `Borrow<Self>`.
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
///
/// This trait is used to express polymorphism over how data is stored, using an associated generic
/// type.
///
/// # Example
///
/// If you have a struct, in which you want to conditionally store values in a different way, you
/// can use this trait to express that. For example, you can write a struct like this:
///
/// ```rust
/// # use tupperware::{Storage, Inline};
/// type GroupId = i64;
///
/// pub struct User<S: Storage = Inline> {
///     name: S::Type<str>,
///     groups: S::Type<[GroupId]>,
/// }
/// ```
///
/// Now, depending on the storage parameter, the values are stored differently. If storage is
/// not specified, it defaults to [`Inline`], which means it is stored to the equivalent of:
///
/// ```rust
/// # type GroupId = i64;
/// pub struct User {
///     name: String,
///     groups: Vec<GroupId>,
/// }
/// ```
///
/// However, if you want to make this cheap to clone, for example because you store this value in a
/// cache, you can use `User<Arc>` instead, which will use atomic reference counting:
///
/// ```
/// # use std::sync::Arc;
/// # type GroupId = i64;
/// pub struct User {
///     name: Arc<str>,
///     groups: Arc<[GroupId]>,
/// }
/// ```
pub trait Storage {
    /// Type used to store a particular value.
    type Type<T: 'static + SizedRepresentation + ?Sized>: Borrow<T>;
}

/// Store the value inline.
///
/// This is intended to be the default [`Storage`] strategy. It stores all types as their
/// [`SizedRepresentation`], meaning that it maps every type T to T, with a few exceptions:
///
/// - `str` is mapped to `String`,
/// - `[T]` is mapped to `Vec<T>`,
/// - `Path` is mapped to `PathBuf`,
/// - `OsStr` is mapped to `OsString`.
pub struct Inline;

/// Store the value in a heap allocation using  `Box`.
pub struct Box;

/// Store the value in an atomic reference-counted heap allocation [`Arc`][std::sync::Arc].
pub struct Arc;

/// Store the value in a reference-counted heap allocation [`Rc`](std::sync::Rc].
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
