# Tupperware

[![crate version](https://img.shields.io/crates/v/tupperware.svg)](https://crates.io/crates/tupperware)
[![docs](https://img.shields.io/badge/docs-latest-blue)](https://docs.rs/tupperware)

Crate that lets you decide if you want to put your types into a `Box`. It allows
you to express polymorphism over how your data is stored.

## Explanation

Sometimes you have a struct, in which you would like to store the data
differently depending on circumstance. Some examples are:

- You want to be able to switch between using a `Rc` and an `Arc`, depending on
  the presence of a feature that enables support for multithreading (while
  paying the cost of using an atomic reference counter).
- You want to be able to store references to data in circumstances.

Tupperware can help here, by allowing you to use the type system to switch between
different data storage variations. You can write your struct definitions semantially,
capturing what data each field should hold, and then switch between different storage
representations of your data.

## Usage

For example, you can define a struct such as this:

```rust
use tupperware::{Storage, Inline};

type OrderId = i64;

#[derive(Debug, Clone)]
struct MyData<S: Storage = Inline> {
    name: S::Value<str>,
    orders: S::Value<[OrderId]>,
}
```

Now, depending on the `Storage` type parameter, your values will be stored differently.
Here is how the struct would be stored for each of the default storage strategies:

<details>
<summary>Inline</summary>

```rust
#[derive(Debug, Clone)]
struct MyData {
    name: String,
    orders: Vec<OrderId>,
}
```

</details>
<details>
<summary>Box</summary>

```rust
#[derive(Debug, Clone)]
struct MyData {
    name: Box<str>,
    orders: Box<[OrderId]>,
}
```

</details>
<details>
<summary>Arc</summary>

```rust
#[derive(Debug, Clone)]
struct MyData {
    name: Arc<str>,
    orders: Arc<[OrderId]>,
}
```

</details>
<details>
<summary>Rc</summary>

```rust
#[derive(Debug, Clone)]
struct MyData {
    name: Rc<str>,
    orders: Rc<[OrderId]>,
}
```

</details>
<details>
<summary>Ref<'a></summary>

```rust
#[derive(Debug, Clone)]
struct MyData {
    name: &'a str,
    orders: &'a [OrderId]>,
}
```

</details>

This example shows how the storage strategy now lets you control how your type
is stored. It allows you to specialize the storage mechanism easily, for example
swapping out `Rc` for `Arc` if a given feature is enabled:

```rust
#[cfg(feature = "sync")]
pub type MyData = types::MyData<Arc>;
#[cfg(not(feature = "sync"))]
pub type MyData = types::MyData<Rc>;
```


With this code, you can now use the type parameter to control how your types are stored.
This is set to default to storing the type inline, however you have a couple of built-in
strategies. Here is an example of how `str` maps using different strategies:

You can also define your own storage mechanisms by implementing the `Storage` trait.

## License

MIT, see [LICENSE.md][].
