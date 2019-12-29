use actix::{Actor, SyncContext};
use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use dotenv::dotenv;
use r2d2::Pool;
use std::env;

pub type PostgresPool = Pool<ConnectionManager<PgConnection>>;

pub struct DbExecutor(pub PostgresPool);

impl DbExecutor {
    pub fn new() -> Self {
        let pool = get_pool();

        Self(pool)
    }
}

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

pub fn get_pool() -> PostgresPool {
    dotenv().ok();
    let url = env::var("DATABASE_URL").expect("no DB URL");
    let mgr = ConnectionManager::<PgConnection>::new(url);
    r2d2::Pool::builder()
        .build(mgr)
        .expect("could not build connection pool")
}
