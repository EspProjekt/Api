use r2d2::Pool;
use r2d2::PooledConnection;
use r2d2_redis::RedisConnectionManager;

pub type RedisPool = Pool<RedisConnectionManager>;
pub type RedisConn = PooledConnection<RedisConnectionManager>;
