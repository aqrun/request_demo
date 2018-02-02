use tokio_core::reactor::Core;
use hyper::{Client, Error, Request, Post};
use hyper::header::{ContentLength, ContentType};
use serde_json::{self, Value};
use futures::{future, Future, Stream};
use std::env;
use super::{get_env_url, ResultBean, AccessTokenBean};

// base url
pub fn get_base_url() -> String {

    let mut core = Core::new().unwrap();
    let client = Client::new(&core.handle());

    let uri = format!("{}/v3/geo/info", get_env_url()).parse().unwrap();
    let request = Request::new(Post, uri);

    let work = client.request(request).and_then(|res| {
//        println!("Response: {}", res.status());
//        println!("============\n {:?} \n============\n", res);

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
//    println!("11 {:?}", data_json);
//    println!("code: {}", data_json["code"]);
  //  println!("base_url: {}", data_json["result"]["base_url"]);
    let base_url: &str = data_json["result"]["base_url"].as_str().unwrap_or("base_url error");
    String::from(base_url)
}

// get access token
pub fn get_access_token() -> String {
    let mut core = Core::new().unwrap();
    let client = Client::new(&core.handle());

    // 请求链接
    let url = format!("{}/v3/auth/token-request", get_env_url()).parse().unwrap();

    let mut request = Request::new(Post, url);
    //需要的参数
    let username = env::var("USERNAME").expect("USERNAME not exist");
    let password = env::var("PASSWORD").expect("PASSWORD not exist");

    let json = json!({
            "username": username,
            "password": password
        });
    let body = json.to_string();
    
    // 设置内 content-type application/json
    request.headers_mut().set(ContentType::json());
    // 设置 content-length
    request.headers_mut().set(ContentLength(body.len() as u64));
    // post 内容
    request.set_body(body);

    let work = client.request(request).and_then(|res| {
        res.body().fold(Vec::new(), |mut v, chunk| {
            v.extend(&chunk[..]);
            future::ok::<_, Error>(v)
        }).and_then(|chunks| {
            let s = String::from_utf8(chunks).unwrap();
            future::ok::<_, Error>(s)
        })
    });

    let data = core.run(work).unwrap();

    // 转为结果结构体
    let result_bean: ResultBean<AccessTokenBean> = serde_json::from_str(&data).unwrap();

//    println!("{:?}", result_bean);
//    println!("{}", result_bean.result.access_token);
    //返回 token
    result_bean.result.access_token
}



