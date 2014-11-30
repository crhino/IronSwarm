#![crate_name = "swarm"]
#![crate_type = "rlib"]
#![crate_type = "dylib"]
mod agent;

pub trait Location {
    fn distance(&Self, other: &Self) -> uint;
}

#[test]
fn it_works() {
}
