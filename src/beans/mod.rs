
mod access_token_bean;

pub use self::access_token_bean::AccessTokenBean;

#[derive(Serialize, Deserialize, Debug)]
pub struct ResultBean<T>{
    pub code: i32,
    pub msg: String,
    pub result: T
}