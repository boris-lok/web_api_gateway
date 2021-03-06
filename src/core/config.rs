#[derive(Debug, Clone)]
pub struct Config {
    pub debug: bool,
    pub secret_key: String,
    pub postgres_host: String,
    pub postgres_database: String,
    pub postgres_username: String,
    pub postgres_password: String,
    pub postgres_port: u16,
    pub postgres_max_connections: u32,
    pub redis_host: String,
    pub redis_username: Option<String>,
    pub redis_password: String,
    pub redis_port: u16,
}

impl Config {
    pub fn new() -> Self {
        let debug = dotenv::var("DEBUG")
            .map(|x| x.parse::<bool>().unwrap_or(true))
            .unwrap_or_else(|_| true);

        if debug {
            let _ = dotenv::from_path("./dev.env").expect("Can't find the dev.env.");
        } else {
            let _ = dotenv::from_path("./prod.env").expect("Can't find the prod.env.");
        }

        let secret_key = dotenv::var("SECRET_KEY").expect("Can't read secret_key from env.");

        let postgres_host =
            dotenv::var("POSTGRES_HOST").expect("Can't read postgres_host from env.");
        let postgres_username =
            dotenv::var("POSTGRES_USER").expect("Can't read postgres_user from env.");
        let postgres_password =
            dotenv::var("POSTGRES_PASSWORD").expect("Can't read postgres_password from env.");
        let postgres_port = dotenv::var("POSTGRES_PORT")
            .expect("Can't read postgres_port from env.")
            .parse::<u16>()
            .expect("Can't parse the port to u16 type.");
        let postgres_database =
            dotenv::var("POSTGRES_DB").expect("Can't read postgres_db from env.");

        let redis_host = dotenv::var("REDIS_HOST").expect("Can't read redis host from env.");
        let redis_username = dotenv::var("REDIS_USERNAME").ok();
        let redis_password =
            dotenv::var("REDIS_PASSWORD").expect("Can't read redis password from env.");
        let redis_port = dotenv::var("REDIS_PORT")
            .expect("Can't read redis port from env.")
            .parse::<u16>()
            .unwrap_or(6379);

        Self {
            debug,
            secret_key,
            postgres_host,
            postgres_database,
            postgres_username,
            postgres_password,
            postgres_port,
            postgres_max_connections: 10,
            redis_host,
            redis_username,
            redis_password,
            redis_port
        }
    }
}
