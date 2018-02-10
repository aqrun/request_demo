
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;
extern crate curl;
extern crate hyper;
extern crate futures;
extern crate tokio_core;
extern crate dotenv;
#[macro_use]
extern crate lazy_static;

use std::env;

mod beans;
mod auth;
mod app;

pub use self::beans::{ResultBean, AccessTokenBean};
pub use self::app::App;
pub use self::auth::fetch_access_token;

//get env base url
pub fn get_env_url() -> String {
    env::var("BASE_URL").expect("BASE_URL not exist in .env")
}
