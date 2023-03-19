use injectify::Injectify;
use std::fmt::Debug;

pub trait Trait1: Debug {}
pub trait Trait2: Debug {}

#[derive(Debug)]
struct T1(i64);
impl Trait1 for T1 {}

#[derive(Debug)]
struct T2(String);
impl Trait2 for T2 {}

#[derive(Debug)]
struct T3 {
    _x: f32,
}
impl Trait2 for T3 {}

#[Injectify]
#[derive(Debug)]
pub struct InjectionStruct<T: Debug> {
    x: i64,
    y: impl Trait1,
    z: impl Trait2,
    z2: impl Trait2,
    u: T,
}

#[test]
fn test() {
    let t = InjectionStruct {
        x: 0,
        y: T1(123),
        z: T2(String::from("123")),
        z2: T3 { _x: 12.3 },
        u: 9.2,
    };

    let t2 = InjectionStruct {
        x: 0,
        y: T1(123),
        z: T3 { _x: 0.1 },
        z2: T3 { _x: 2.3 },
        u: "3".to_string(),
    };

    println!("{:#?}", t);
    println!("{:#?}", t2);
}
