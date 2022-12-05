use redis::{self, ConnectionLike, RedisResult, aio::ConnectionLike as AsyncConnectionLike};
use crate::traits;
use std::collections::HashMap;
use async_trait::async_trait;
use futures_util::StreamExt;
#[async_trait]
pub trait HSetRedis: traits::HashSet {
    fn hset<C>(&self, con: &mut C) -> Result<i32, redis::RedisError> 
    where C: ConnectionLike
    {
        let mut query = redis::cmd("HSET");
        for arg in &self.gen_hset_args() {
            query.arg(arg);
        }
        let retval = query.query::<i32>(con)?;
        Ok(retval)
    }

    async fn hset_async<C>(&self, con: &mut C) -> RedisResult<i32>
    where C: AsyncConnectionLike + std::marker::Send
    {
        let mut query = redis::cmd("HSET");
        for arg in &self.gen_hset_args() {
            query.arg(arg);
        }
        Ok(query.query_async::<_, i32>(con)
            .await?)
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

    async fn hgetall_async<T, C>(key: &T, con: &mut C) -> Result<Self, redis::RedisError>
    where C: AsyncConnectionLike + std::marker::Send,
            T: ToString + std::marker::Sync, 
            Self: Sized
    {
        let mut res: HashMap<String, String> = redis::cmd("HGETALL")
            .arg(format!("{}:{}", Self::get_prefix(), &key.to_string()))
            .query_async(con)
            .await?;
        res.insert(Self::key_name(), format!("{}:{}", Self::get_prefix(), key.to_string()));
        Ok(Self::get_hset_from_args(&res))
    }

    async fn hscan_keys_async<C>(con: &mut C) -> Result<Vec<String>, redis::RedisError>
    where Self: Sized, C: AsyncConnectionLike + std::marker::Send
    {
        let mut res: Vec<String> = Vec::new();
        let mut iterator: redis::AsyncIter<String> = redis::cmd("SCAN")
            .cursor_arg(0)
            .arg("MATCH")
            .arg(format!("{}:*", Self::get_prefix()))
            .arg("TYPE")
            .arg("hash")
            .clone()
            .iter_async(con)
            .await?;
        while let Some(key) = iterator.next_item().await {
            res.push(key)
        }
        Ok(res)
    }

    async fn hgetall_all_async<C>(con: &mut C) -> Result<Vec<Self>, redis::RedisError>
    where Self: Sized, C: AsyncConnectionLike + std::marker::Send
    {
        let mut res: Vec<Self> = Vec::new();
        let keys = Self::hscan_keys_async(con).await?;
        for key in keys {
            let key_no_prefix = key.to_string().replace(&format!("{}:", &Self::get_prefix()), "");
            let item: Self = Self::hgetall_async(&key_no_prefix, con).await?;
            res.push(item);
        }
        Ok(res)
    }
}

    