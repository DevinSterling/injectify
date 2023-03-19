# Injectify
Utility library to use `impl Trait` in structs.

## Example

```rust
use injectify::Injectify;
use std::fmt::Debug;

// Allow `MyStruct` to contain implementations 
// of `Trait1` and `Trait2` using `impl`:
#[Injectify]
#[derive(Debug)]
struct MyStruct<T> {
    x: i32,
    t1: impl Trait1,
    t2: impl Trait2,
    t2_2: impl Trait2,
    t: T,
}

trait Trait1: Debug {}
trait Trait2: Debug {}

#[derive(Debug)]
struct T1(i64);
impl Trait1 for T1 {}

#[derive(Debug)]
struct T2(String);
impl Trait2 for T2 {}

#[derive(Debug)]
struct T3 { y: bool }
impl Trait2 for T3 {}

fn main() {
    let my_struct = MyStruct {
        x: 1337,
        t1: T1(123),
        t2: T2(String::from("abc")),
        t2_2: T3 { y: true },
        t: 20.23,
    };

    println!("{my_struct:#?}");
}
```

## License
License
Licensed under either of Apache License, Version 2.0 or MIT license at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for 
inclusion in this crate by you, as defined in the Apache-2.0 license, shall be dual 
licensed as above, without any additional terms or conditions.