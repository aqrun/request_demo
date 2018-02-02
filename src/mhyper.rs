use std::io::{self, Write};
use futures::{Future, Stream};
use futures::future;
use hyper::{Client, Error};
use tokio_core::reactor::Core;
use serde_json::{self, Value};

pub fn run(){

    let mut core = Core::new().unwrap();
    let client = Client::new(&core.handle());

    let uri = "http://localhost:99/v3/geo/info".parse().unwrap();
    let work = client.get(uri).and_then(|res| {
        println!("Response: {}", res.status());

        res.body().fold(Vec::new(), |mut v, chunk| {
            v.extend(&chunk[..]);
            future::ok::<_, Error>(v)
        }).and_then(|chunks| {
            let s = String::from_utf8(chunks).unwrap();
            future::ok::<_, Error>(s)
        })
    });
    let data = core.run(work).unwrap();

    let data_json: Value = serde_json::from_str(&data).unwrap();
    println!("11 {:?}", data_json);
    println!("code: {}", data_json["code"]);
    println!("base_url: {}", data_json["result"]["base_url"]);
}