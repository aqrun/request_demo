extern crate request_demo;
extern crate dotenv;

use dotenv::dotenv;
use request_demo::auth::{ get_access_token };

fn main(){
    dotenv().ok();

    //let url = get_base_url();
    //获取access token
    get_access_token();
}
