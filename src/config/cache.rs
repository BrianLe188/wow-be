use bb8::{Pool, PooledConnection};
use bb8_redis::RedisConnectionManager;

pub type CachePool = Pool<RedisConnectionManager>;

pub type CacheConn<'a> = PooledConnection<'a, RedisConnectionManager>;

pub async fn init_cache_pool(cache_url: &str) -> Result<CachePool, String> {
    let config = RedisConnectionManager::new(cache_url).map_err(|err| format!("{:?}", err))?;

    Pool::builder()
        .build(config)
        .await
        .map_err(|err| format!("{:?}", err))
}

pub async fn get_cache_conn(pool: &CachePool) -> Result<CacheConn<'_>, String> {
    pool.get().await.map_err(|err| err.to_string())
}
