extern crate statham;
extern crate statham_macro;
use statham::*;
use statham_macro::json;

fn main() {
    let mut a = Json::new()
        .item("a", Null)
        .item("b", 32)
        // same as Null
        .item("c", None)
        // auto unpack Option
        .item("d", Some(3.14))
        // vectors
        .item("e", vec![1, 2, 3, 4])
        .item("f", vec![3.14, 4.15, 5.16])
        // slice support (enable unstable feature)
        // .item("f", &[3.14, 4.15, 5.16])
        // automatic primitive vector casting
        .item("g", array!["123", 32])
        .item("h", "Hello, world!")
        .items(vec![("A1", 5), ("A2", 6)])
        .items(vec![
            ("v1", from!(vec![1, 2, 3])),
            ("v2", from!(32)),
            ("v3", from!("hello")),
            ("v4", None),
            ("v5", from!(true)),
        ]);

    if let Some(value) = a.get_mut("b") {
        *value = from!("123");
    }

    let b = Json::default().item("a", a).item("bool", true);

    // TODO
    let c = json! {
        "a": "hello",
        "b": 42,
        "c": 3.14,
        "d": false,
        "e": {}
    };

    println!("b: {}", b);
    println!("c: {}", c);
}
