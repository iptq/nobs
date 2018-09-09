mod mem;
// mod redis;

pub use mem::MemCache;
// pub use redis::RedisCache;

pub trait Cache {
    fn set(&mut self, &str, impl AsRef<str>);
    fn get(&self, &str) -> &str;
}
