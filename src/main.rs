extern crate futures;
extern crate hyper;
extern crate tokio_core;
extern crate serde_json;


mod easy;
mod mhyper;


fn main() {
    mhyper::run();
}
