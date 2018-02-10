extern crate request_demo;
extern crate dotenv;

use dotenv::dotenv;
use request_demo::{App};

fn main(){
    dotenv().ok();

    //let mut app = App::new();
    //println!("access_token: {}", app.get_access_token().expect("token empty"))
}
