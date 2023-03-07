use crate::error_handler::CustomError;
use lazy_static::lazy_static;
use r2d2;
use openssl::ssl::{SslConnector, SslMethod};
use postgres_openssl::MakeTlsConnector;
use r2d2_postgres::{PostgresConnectionManager};

type Pool = r2d2::Pool<PostgresConnectionManager<MakeTlsConnector>>;
pub type DbConnection = r2d2::PooledConnection<PostgresConnectionManager<MakeTlsConnector>>;

lazy_static! {
    static ref POOL : Pool = {
        let builder = SslConnector::builder(SslMethod::tls()).unwrap();
        let connector = MakeTlsConnector::new(builder.build());
        /*let config = Config::new()
        .host("ep-orange-flower-377957.eu-central-1.aws.neon.tech")
        .user("pierrelestunff")
        .port(5432)
        .password("c0pjCDfAnI2N")
        .ssl_mode(SslMode.Require)
        .dbname("neondb");*/
        let manager = PostgresConnectionManager::new("host=ep-orange-flower-377957.eu-central-1.aws.neon.tech user=pierrelestunff password=c0pjCDfAnI2N dbname=neondb".parse().unwrap(), connector);
        Pool::new(manager).unwrap()
        // Pool::new(manager).expect("Failed to create db pool")
    };
}

pub fn init() {
    lazy_static::initialize(&POOL);
    connection();//.expect("Failed to get db connection");
    //embedded_migrations::run(&conn).unwrap();
}

pub fn connection() -> Result<DbConnection, CustomError> {
    POOL.get().map_err(|e| CustomError::new(500, format!("Failed getting db connection: {}", e)))
}
