extern crate rust_test;
extern crate dotenv;

use dotenv::dotenv;
use rust_test::auth::{ get_access_token };

fn main(){
    dotenv().ok();

    //let url = get_base_url();
    //获取access token
    get_access_token()
}
