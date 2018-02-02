
#[derive(Serialize, Deserialize, Debug)]
pub struct AccessTokenBean{
    pub access_token: String,
    pub expires_in: i32,
}