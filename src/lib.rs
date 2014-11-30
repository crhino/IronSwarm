#![crate_name = "swarm"]
#![crate_type = "rlib"]
#![crate_type = "dylib"]
pub mod agent;
pub mod byteid;
pub mod artifact;
pub mod swarm;

pub trait Location {
    fn distance(&Self, other: &Self) -> uint;
}

#[test]
fn it_works() {
}
