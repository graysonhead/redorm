use chrono::{NaiveDate, DateTime, Utc};
use redis;
use redorm::prelude::*;
use redorm::redis::HSetRedis;

#[derive(DeriveHashSet, Debug)]
#[redorm(prefix_name = "poets")]
pub struct Poet {
    #[redorm(key)]
    name: String,
    born: NaiveDate,
    died: NaiveDate,
    now: DateTime<Utc>,
    genre: String,
    nationality: String,
}

// For Redis Client support, Implement HSetRedis with the "redis" feature enabled
impl HSetRedis for Poet{}

fn main() {

    // Define an object. Anything that implements ToString should work, as well as most Date/Time types, integers, and floats
    let mut verlaine = Poet {
        name: "Verlaine".into(),
        born: NaiveDate::from_ymd_opt(1844, 3, 30).unwrap(),
        died: NaiveDate::from_ymd_opt(1896, 1, 8).unwrap(),
        now: Utc::now(),
        genre: "Decadent".into(),
        nationality: "French".into(),
    };
    // Create a redis client instance and get a connection
    let client = redis::Client::open("redis://127.0.0.1:6379").unwrap();
    let mut con = client.get_connection().unwrap();
    // Store the struct in Redis
    verlaine.hset(&mut con).unwrap();
    // Update a field and update in redis
    let new_time = Utc::now();
    verlaine.now = new_time;
    verlaine.hset(&mut con).unwrap();

    // Fetch the struct from redis
    let new_instance = Poet::hgetall(&verlaine.get_key(), &mut con).unwrap();
    assert_eq!(new_instance.now, new_time);
}
