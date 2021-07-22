use std::collections::HashMap;
use to_hashmap::{ToHashMap, Wrapper};
use to_hashmap_derive::*;

#[derive(ToHashMap, Debug)]
pub struct Foo {
    bar: i32,
    baz: i32,
}

fn main() {
    let foo = Foo { bar: 0, baz: 0 };

    let map = foo.to_hashmap();

    println!("{:?}", map);
}
