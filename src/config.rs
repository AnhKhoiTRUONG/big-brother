use std::env;

pub struct Config {
    identifier: String,
    secret: String,
}

impl Config {
    pub fn init() -> Self {
        dotenvy::dotenv().ok();
        Config {
            identifier: env::var("identifier")
                .expect("Environment variable 'identifier' must be set"),
            secret: env::var("secret").expect("Environment variable 'secret' must be set"),
        }
    }

    pub fn identifier(&self) -> &String {
        &self.identifier
    }

    pub fn secret(&self) -> &String {
        &self.secret
    }
}
