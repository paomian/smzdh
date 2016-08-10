#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate nom;

extern crate log4rs;
extern crate iron;
extern crate router;
extern crate serde_json;
extern crate serde;
extern crate smzdh_commons;
extern crate rand;
extern crate redis;

mod handlers;
mod handler;


fn main() {
    handler::run();
}
