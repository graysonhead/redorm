use redis;
use crate::traits;
use std::collections::HashMap;

pub trait HSetRedis: traits::HashSet {
    fn hset(&self, con: &mut redis::Connection) -> Result<i32, redis::RedisError> {
        let mut query = redis::cmd("HSET");
        for arg in &self.gen_hset_args() {
            query.arg(arg);
        }
        let retval = query.query::<i32>(con)?;
        Ok(retval)
    }

    fn hgetall<T>(key: &T, con: &mut redis::Connection) -> Result<Self, redis::RedisError>
    where T: ToString, Self: Sized
    {
        let mut res: HashMap<String, String> = redis::cmd("HGETALL")
            .arg(format!("{}:{}", Self::get_prefix(), &key.to_string()))
            .query(con)?;
        res.insert(Self::key_name(), format!("{}:{}", Self::get_prefix(), key.to_string()));
        Ok(Self::get_hset_from_args(&res))
    }
}