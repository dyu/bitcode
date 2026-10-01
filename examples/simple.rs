use std::borrow::Cow;
use bitcode::{Encode, Decode};

#[derive(Encode, Decode, PartialEq, Debug)]
#[repr(u8)]
pub enum Size {
    SMALL,
    MEDIUM,
    LARGE,
}

#[derive(Encode, Decode, PartialEq, Debug)]
struct Bar<'a> {
  f: f32,
  d: f64,
  s: &'a str,
}

#[derive(Encode, Decode, PartialEq, Debug)]
struct Foo<'a> {
    a: u8,
    b: Bar<'a>,
    s: Size,
    u: &'a [u8],
    u_list: Vec<&'a [u8]>,
    v: Cow<'a, [u8]>,
    v_list: Vec<Cow<'a, [u8]>>,
    x: u32,
    y: &'a str,
    y_list: Vec<&'a str>,
    z: Cow<'a, str>,
    z_list: Vec<Cow<'a, str>>,
}

fn main() {
    let original = Foo {
        a: 1,
        b: Bar { f: 1.5, d: 10.05, s: "bar" },
        s: Size::MEDIUM,
        u: b"\x01\x02hello",
        u_list: vec![
            b"\x01uno",
            b"\x02dos",
        ],
        v: b"\x03\x04world".into(),
        v_list: vec![
            b"\x01one".into(),
            b"\x02two".into(),
        ],
        x: 10,
        y: "abc",
        y_list: vec![
            "hello",
            "world",
        ],
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
