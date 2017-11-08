extern crate hyper;
extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_json;

use serde_json::to_string;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum TestEnum {
    A(Test),
    B(String),
    C(String, String),
    D(Test, String),
    E,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Test {
    foo: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TestTop {
    xyzzy: TestEnum,
}

fn main() {
    let ts = Test { foo: "bar".to_string() };

    let t1 = TestEnum::A(ts.clone());
    let t2 = TestEnum::B("baz".to_string());
    let t3 = TestEnum::C("baz".to_string(), "bamf".to_string());
    let t4 = TestEnum::D(ts, "baz".to_string());
    let t5 = TestTop { xyzzy: TestEnum::B("baz".to_string()) };
    let t6 = TestTop { xyzzy: TestEnum::E };
    
    println!("T1 = {}", to_string(&t1).unwrap());
    println!("T2 = {}", to_string(&t2).unwrap());
    println!("T3 = {}", to_string(&t3).unwrap());
    println!("T4 = {}", to_string(&t4).unwrap());
    println!("T5 = {}", to_string(&t5).unwrap());
    println!("T6 = {}", to_string(&t6).unwrap());
}