# Tupperware

[![docs](https://img.shields.io/badge/docs-latest-blue)](https://docs.rs/tupperware)
[![crate version](https://img.shields.io/crates/v/tupperware.svg)](https://crates.io/crates/tupperware)

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

With this code, you can now use the type parameter to control how your types are stored.
This is set to default to storing the type inline, however you have a couple of built-in
strategies. Here is an example of how `str` maps using different strategies:

| Strategy | Type |
| --- | --- |
| Inline | `String` |
| Arc | `Arc<str>` |
| Box | `Box<str>` |
| Rc | `Rc<str>` |
| Ref<'a> | `&'a str` |

You can also define your own storage mechanisms by implementing the `Storage` trait.

## License

MIT, see [LICENSE.md][].
