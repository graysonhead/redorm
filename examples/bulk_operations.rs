use deadpool_redis::{
    redis::{cmd, FromRedisValue},
    Config, Runtime,
};
use tokio;

use redorm::prelude::*;
use redorm::redis::HSetRedis;

#[derive(DeriveHashSet, Debug)]
#[redorm(prefix_name = "example_data")]
pub struct Data {
    #[redorm(key)]
    data_key: String,
    data2: usize,
}

// For Redis Client support, Implement HSetRedis with the "redis" feature enabled
impl HSetRedis for Data {}

#[tokio::main]
async fn main() {
    let client = redis::Client::open("redis://127.0.0.1:6379").unwrap();
    let mut con = client.get_async_connection().await.unwrap();
    //Define some objects.
    for i in 0..1000 {
        let data = Data {
            data_key: i.to_string(),
            data2: i,
        };
        data.hset_async(&mut con).await;
    }
    let data = Data::hgetall_all_async(&mut con).await.unwrap();
    println!("{:#?}", data);
}
