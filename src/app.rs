use fetch_access_token;
use get_env_url;

pub struct App{
    pub base_url: Option<String>,
    pub access_token: Option<String>,
}

impl App {
    pub fn new() -> App {
        let mut app = App{
            base_url: None,
            access_token: None,
        };
        app.init();
        app
    }
    pub fn init(&mut self) {
        let base_url = self.get_base_url().unwrap();
    }
    pub fn get_base_url(&mut self) -> Result<&str, ()> {
        if self.base_url.is_none() {
            self.base_url = Some(get_env_url());
        }
        Ok(self.base_url.as_ref().unwrap())
    }
    pub fn get_access_token(&mut self) -> Result<&str, ()> {
        if self.access_token.is_none() {
            let base_url = self.base_url.as_ref().unwrap();
            self.access_token = Some(fetch_access_token(base_url));
        }
        Ok(self.access_token.as_ref().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::App;

    lazy_static! {
        static ref APP: App = App::new();
    }

    #[test]
    fn app_get_access_token_is_ok() {
        let token = APP.get_access_token();
        assert!(token.is_ok());
    }
}
