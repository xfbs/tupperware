use super::Storage;
use std::sync::Arc;

#[derive(Clone, Debug)]
struct MyData<S: Storage> {
    string: S::Type<str>,
    list: S::Type<[i64]>,
    number: usize,
}

#[test]
fn can_use_boxed() {
    let data = MyData::<super::Box> {
        string: "abc".into(),
        list: Box::new([15]),
        number: 5,
    };
    assert_eq!(&*data.string, "abc");
    assert_eq!(&*data.list, &[15]);
    assert_eq!(data.number, 5);
}

#[test]
fn can_use_arc() {
    let data = MyData::<super::Arc> {
        string: "abc".into(),
        list: Arc::new([15]),
        number: 5,
    };

    assert_eq!(&*data.string, "abc");
    assert_eq!(&*data.list, &[15]);
    assert_eq!(data.number, 5);
}

#[test]
fn can_use_ref() {
    let data = MyData::<super::Ref<'_>> {
        string: "abc",
        list: &[15],
        number: 5,
    };

    assert_eq!(&*data.string, "abc");
    assert_eq!(&*data.list, &[15]);
    assert_eq!(data.number, 5);
}
