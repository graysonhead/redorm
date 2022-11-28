use deadpool_redis::{redis::{cmd, FromRedisValue}, Config, Runtime};
use tokio;

use redorm::prelude::*;
use redorm::redis::HSetRedis;

#[derive(DeriveHashSet, Debug)]
#[redorm(prefix_name = "poets")]
pub struct Poet {
    #[redorm(key)]
    name: String,
    genre: String,
    nationality: String,
}

// For Redis Client support, Implement HSetRedis with the "redis" feature enabled
impl HSetRedis for Poet{}

#[tokio::main]
async fn main() {

    let verlaine = Poet {
        name: "Verlaine2".into(),
        genre: "Decadent".into(),
        nationality: "French".into(),
    };

    let cfg = Config::from_url("redis://127.0.0.1/");
    let pool = cfg.create_pool(Some(Runtime::Tokio1)).unwrap();

    let mut conn = pool.get().await.unwrap();
    verlaine.hset_async(&mut conn).await.unwrap();

    let new_result = Poet::hgetall_async(&verlaine.name, &mut conn).await.unwrap();
    println!("{:#?}", new_result);
}