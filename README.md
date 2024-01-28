# Tupperware

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
different data storage variations.

For example:

```rust
use tupperware::{Storage, Inline};

#[derive(Debug, Clone)]
struct MyData<S: Storage = Inline> {
    name: S::Value<str>,
    orders: S::Value<i64>,
}
```

With this code, you can now decide between storing the data inline (which is
the default) or storing it in an Arc. For example, `MyData<Inline>` expands to:

```rust
struct MyData {
    name: String,
    orders: Vec<i64>
}
```

However, `MyData<Arc>` expands to:

```rust
struct MyData {
    name: Arc<str>,
    orders: Arc<[i64]>
}
```

Using this trait gives you polymorphism over how your data is stored. You can
also defined your own custom storage strategies.


## License

MIT, see [LICENSE.md][].
