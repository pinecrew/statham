extern crate statham_macro;
extern crate statham;
use statham_macro::json;
use statham::*;

fn main() {
    let mut a = Json::new()
        .insert("a", Null)
        .insert("b", 32)
        .insert("c", 3.14)
        .insert("d", vec![1, 2, 3, 4])
        .insert("e", vec![3.14, 4.15, 5.16])
        .insert("f", vec![from!("123"), from!(32)])
        .insert("g", "Hello, world!")
        .insert_iter(vec![("A1", 5), ("A2", 6)])
        .insert_iter(vec![
            ("v1", from!(vec![1, 2, 3])),
            ("v2", from!(32)),
            ("v3", from!("hello")),
            ("v4", Null),
            ("v5", from!(true)),
        ]);

    if let Some(value) = a.get_mut("b") {
        *value = from!("123");
    }

    let b = Json::default().insert("a", a).insert("bool", true);

    // TODO
    let c = json!{
        "a": "hello",
        "b": 42,
        "c": 3.14,
        "d": false,
        "e": {}
    };

    println!("b: {}", b);
    println!("c: {}", c);
}
