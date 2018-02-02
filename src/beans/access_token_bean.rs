

// access token API数据结果
#[derive(Serialize, Deserialize, Debug)]
pub struct AccessTokenBean{
    // token
    pub access_token: String,
    // 过期时间
    pub expires_in: i32,
}