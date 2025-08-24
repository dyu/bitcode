use std::borrow::Cow;
use bitcode::{Encode, Decode};

#[derive(Encode, Decode, PartialEq, Debug)]
struct Foo<'a> {
    x: u32,
    y: &'a str,
    z: Cow<'a, str>,
    z_list: Vec<Cow<'a, str>>,
}

fn main() {
    let original = Foo {
        x: 10,
        y: "abc",
        z: "gg".into(),
        z_list: vec![
            "abc".into(),
            "gg".into(),
        ],
    };
    let encoded: Vec<u8> = bitcode::encode(&original); // No error
    let decoded: Foo<'_> = bitcode::decode(&encoded).unwrap();
    assert_eq!(original, decoded);
    println!("{:?}", decoded);
    /*
    println!(
        "{{ x: {}, y: \"{}\", z: \"{}\" }}",
        original.x,
        original.y,
        original.z,
    );
    */
}
