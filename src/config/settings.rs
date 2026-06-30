use anyhow::Result;

#[derive(Clone)]
pub struct Settings {
    pub database_url: String,
    pub host: String,
    pub port: u16,
    pub jwt_secret: String,
}

impl Settings {
    pub fn new() -> Result<Self> {
        dotenvy::dotenv().ok();

        Ok(Self {
            database_url: std::env::var("DATABASE_URL")?,
            host: std::env::var("HOST")?,
            port: std::env::var("PORT")?.parse()?,
            jwt_secret: std::env::var("JWT_SECRET")?,
        })
    }
}