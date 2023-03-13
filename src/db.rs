use crate::error_handler::CustomError;
use lazy_static::lazy_static;
use r2d2;
use openssl::ssl::{SslConnector, SslMethod, SslVerifyMode};
use postgres_openssl::MakeTlsConnector;
use r2d2_postgres::{PostgresConnectionManager};
use serde::Deserialize;

type Pool = r2d2::Pool<PostgresConnectionManager<MakeTlsConnector>>;
pub type DbConnection = r2d2::PooledConnection<PostgresConnectionManager<MakeTlsConnector>>;

use std::fmt;

#[derive(Deserialize, Debug)]
struct Config {
    db_host: String,
    db_user: String,
    db_password: String,
    db_name: String
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "host={} user={} password={} dbname={}", self.db_host, self.db_user, self.db_password, self.db_name)
    }
}

lazy_static! {
    static ref POOL : Pool = {
        let c = envy::from_env::<Config>().expect("Please provide DB_HOST, DB_USER, DB_PASSWORD and DB_NAME env var");
        let mut builder = SslConnector::builder(SslMethod::tls()).unwrap();
        builder.set_verify(SslVerifyMode::NONE);
        let connector = MakeTlsConnector::new(builder.build());
        let manager = PostgresConnectionManager::new(c.to_string().parse().unwrap(), connector);
        Pool::new(manager).unwrap()
    };
}

pub async fn init() {
    lazy_static::initialize(&POOL);
    //connection();//.expect("Failed to get db connection");
    //embedded_migrations::run(&conn).unwrap();
}

pub fn connection() -> Result<DbConnection, CustomError> {
    POOL.get().map_err(|e| CustomError::new(500, format!("Failed getting db connection: {}", e)))
}
