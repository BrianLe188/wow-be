use diesel_async::{
    AsyncPgConnection,
    pooled_connection::{
        AsyncDieselConnectionManager,
        deadpool::{Object, Pool},
    },
};

pub type DbPool = Pool<AsyncPgConnection>;

pub type DbConn = Object<AsyncPgConnection>;

pub fn init_pool(db_url: &str) -> Result<DbPool, String> {
    let config = AsyncDieselConnectionManager::<AsyncPgConnection>::new(db_url);

    Pool::builder(config)
        .build()
        .map_err(|err| format!("{:?}", err))
}

pub async fn get_conn(pool: &DbPool) -> Result<DbConn, String> {
    pool.get().await.map_err(|err| err.to_string())
}
