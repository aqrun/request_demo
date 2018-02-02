
mod access_token_bean;

pub use self::access_token_bean::AccessTokenBean;

// 定义API返回结果结构体
// code msg 固定 具体值在result
#[derive(Serialize, Deserialize, Debug)]
pub struct ResultBean<T>{
    // code 0 成功 1失败
    pub code: i32,
    // 服务器返回错误相关信息
    pub msg: String,
    // 结果JSON对象
    pub result: T
}