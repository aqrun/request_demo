use fetch_access_token;
use get_env_url;

// #[derive(Default)]
// struct BaseUrl(Option<String>);

// impl BaseUrl {
//     fn get(&mut self) -> &str {
//         self.0.get_or_insert_with(|| get_env_url())
//     }
// }

// #[derive(Default)]
// struct AccessToken(Option<String>);

// impl AccessToken {
//     fn get(&mut self, base_url: &str) -> &str {
//         self.0.get_or_insert_with(|| fetch_access_token(base_url))
//     }
// }

#[derive(Default)]
struct Cache<T>(Option<T>);

impl<T> Cache<T> {
    fn get<F>(&mut self, f: F) -> &T
    where 
        F: FnOnce() -> T,
    {
        self.0.get_or_insert_with(f)
    }
}

#[derive(Default)]
pub struct App{
    base_url: Cache<String>,
    access_token: Cache<String>,
}

impl App {
    pub fn new() -> App {
        App::default()
    }
    pub fn get_access_token(&mut self) -> Result<&str, ()> {
        let base_url = self.base_url.get(get_env_url);
        let token = self.access_token.get(||fetch_access_token(base_url));
        Ok(token)
    }
}

#[cfg(test)]
mod tests {
    use super::App;
    use dotenv::dotenv;

    // lazy_static! {
    //     static ref APP: App = App::new();
    // }

    #[test]
    fn app_get_access_token_is_ok() {
        dotenv().ok();

        let mut app = App::new();
        let token = app.get_access_token();
        //println!("=========== {}",token.unwrap());
        assert!(token.is_ok());
    }
}
