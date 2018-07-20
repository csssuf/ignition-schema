extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod schema {
    include!(concat!(env!("OUT_DIR"), "/schema.rs"));
}

pub use schema::*;
