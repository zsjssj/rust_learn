use std::collections::HashSet;

// 需要的相关库：redis、bb8【异步连接池】
use bb8::Pool;
use redis::{AsyncCommands, Client, Script};
type RedisPool = Pool<Client>;

#[tokio::main]
pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    // 通过连接池连接redis
    let client = Client::open("redis://127.0.0.1:6379/")?; // 1. 创建 Redis 客户端
    let pool = RedisPool::builder().max_size(20).build(client).await?; // 2. 通过 bb8 创建连接池
    let mut conn = pool.get().await?; // 3. 从连接池获取连接

    /* String数据 */
    // 1. 写入数据
    let _: () = conn.set("s:user:1", "Alice").await?;
    let _: () = conn.set_ex("s:user:2", "Alice", 20).await?; //写入数据，并设置过期时间
    let success: bool = conn.expire("s:user:1", 20).await?; //对已有的数据，设置过期时间
    match success {
        true => println!("过期时间设置成功"),
        false => println!("键不存在，设置失败"),
    }
    // 2. 读取数据
    let value: String = conn.get("s:user:1").await?;
    println!("Redis value: {value}");
    // 3. 自增/自减 n，并返回结果
    let count: i64 = conn.incr("s:count", 5).await?;
    let count: i64 = conn.decr("s:count", 2).await?;
    // 4. 删除数据
    let _: () = conn.del("s:count").await?;

    /* List数据 */
    // 1.从左向右推数据、从左边抛出数据
    let _: () = conn.lpush("list:age", 18).await?;
    let a: i64 = conn.lpop("list:age", None).await?; //一次弹出一个元素
    let tasks: Vec<String> = conn
        .lpop("tasks", Some(std::num::NonZeroUsize::new(3).unwrap()))
        .await?; //一次弹出多个元素【3个】，NonZeroUsize：绝对非零的 usize类型
    // 2.从右向左推数据、从右边抛出数据
    let _: () = conn.rpush("list:name", "ssje").await?;
    let a: String = conn.rpop("list:age", None).await?; //一次弹出一个元素
    let tasks: Vec<String> = conn
        .rpop("tasks", Some(std::num::NonZeroUsize::new(3).unwrap()))
        .await?; //一次弹出多个元素【3个】
    // 3. 读取List的长度
    let len: usize = conn.llen("list:name").await?;
    //  4.查看列表元素
    let list_name: Vec<String> = conn.lrange("list:name", 0, -1).await?;

    /* Hash数据 */
    // 1.写入
    let _: () = conn.hset("hash:person", "ssje", 20).await?; //hash对象名称,hash:key[filed],hash:value
    // let _: () = conn.hset_ex("hash:person", [("zsj", 20)]).await?;
    let _: () = conn.hset_multiple("hash:person", &[("zsj", 20), ("ssje2", 10)]).await?; //批量写入
    // 2.读取
    let age: u16 = conn.hget("hash:person", "ssje").await?;
    let age_list: Vec<u16> = conn.hgetall("hash:person").await?; //获取"hash:person"下所有值
    // 3.获取hsah数据【hash:person】下的keys
    let keys: Vec<String> = conn.hkeys("hash:person").await?;
    // 4.删除
    let _: () = conn.hdel("hash:person", "ssje").await?;
    // 5.获取has数据中filed数量
    let count: usize = conn.hlen("hash:person").await?;

    /* Set数据 【无序】*/
    // 1. 写入
    let _: () = conn.sadd("set:user_ids", "1001").await?; //添加一个成员
    let _: () = conn.sadd("set:user_ids", ("1001", "1002", "1003")).await?; //添加多个成员
    let _: () = conn.sadd("set:group_ids", &["1001", "1002", "1003"]).await?; //添加多个成员
    // 2. 检查是否包含
    let is_has: bool = conn.sismember("set:user_ids", "1001").await?;
    // 3. 查看
    let members: Vec<String> = conn.smembers("set:user_ids").await?;
    // 4. 差集
    let data_diff: HashSet<String> = conn.sdiff(("set:user_ids", "set:group_ids")).await?;
    // 5. 交集
    let data_inter: HashSet<String> = conn.sinter(("set:user_ids", "set:group_ids")).await?;
    // 6. 并集
    let data_inter: HashSet<String> = conn.sunion(("set:user_ids", "set:group_ids")).await?;
    // 7. 获取set长度
    let slen: usize = conn.scard("set:user_ids").await?;
    // 8. 删除set中元素
    let _: () = conn.srem("set:user_ids", "1001").await?;

    /* zSet数据 【有序】*/
    // 1. 添加
    let _: () = conn.zadd("zset:class", "ssje", 80).await?;
    let _: () = conn
        .zadd_multiple("zset:class", &[("zsj", 96), ("ww", 78), ("zyp", 83)])
        .await?; //批量添加
    // 2. 查询
    // 按分数升序查询: ["ww", "ssje", "zyp", "zsj"]
    let result: Vec<String> = conn.zrange("zset:class", 0, -1).await?;
    // 按分数降序查询: ["zsj", "zyp", "ssje", "ww"]
    let result: Vec<String> = conn.zrevrange("zset:class", 0, -1).await?;
    // 查询指定分数范围内的成员，升序 ["ssje", "zyp", "zsj"]
    let result: Vec<String> = conn.zrangebyscore("class", 80, 100).await?;
    // 查询指定分数范围内的成员，降序 ["zyp", "ssje"]
    let result: Vec<String> = conn.zrevrangebyscore("class", 80, 90).await?;
    // 获取所有成员及对应分数  [("ww", 78), ("ssje", 80), ("zyp", 83), ("zsj", 96)]
    let result: Vec<(String, f64)> = conn.zrange_withscores("class", 0, -1).await?;
    // 获取指定成员的分数  Some(90.0)
    let score: Option<f64> = conn.zscore("class", "zsj").await?;

    /* redis事务 */
    // .atomic() 会让管道以 MULTI / EXEC 事务方式执行。
    let result: (String, String) = redis::pipe()
        .atomic()
        .set("price", 100)
        .ignore()
        .set("status", "active")
        .ignore()
        .get("price")
        .get("status")
        .query_async(&mut *conn)
        .await?;
    println!("price = {},status = {}", result.0, result.1);

    /* redis 锁 */
    let key = "lock:order:1001";
    // 尝试获取锁，过期时间 10 秒
    // 1.基础分布式互斥锁
    let lock = RedisLock::new(pool.clone());
    match lock.try_acquire(key, 10_000).await? {
        Some(token) => {
            println!("获取锁成功");
            // 在这里执行业务逻辑
            // 例如处理订单
            let released = lock.release(key, &token).await?;
            println!("释放锁结果：{}", released);
        }
        None => {
            println!("获取锁失败，其他任务正在处理");
        }
    }
    // 1.1,测试多任务竞争同一把锁
    use tokio::join;
    let key = "test:lock:concurrency";
    let (a, b) = join!(lock.try_acquire(key, 10_000), lock.try_acquire(key, 10_000),);
    let a = a?;
    let b = b?;
    assert_ne!(a.is_some(), b.is_some());
    // 释放成功获得的锁
    if let Some(token) = a {
        println!("a 释放锁");
        lock.release(key, &token).await?;
    }
    if let Some(token) = b {
        println!("b 释放锁");
        lock.release(key, &token).await?;
    }
    // 2.可重入锁

    // conn 离开作用域时，连接归还连接池
    Ok(())
}

// 基础分布式互斥锁，适用：多个 Axum 实例不能同时处理同一个订单
struct RedisLock {
    pool: RedisPool,
}
impl RedisLock {
    fn new(pool: RedisPool) -> Self {
        Self { pool }
    }
    ///尝试获取锁
    async fn try_acquire(&self, key: &str, ttl_ms: u64) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let token = uuid::Uuid::new_v4().to_string();
        let mut conn = self.pool.get().await?;
        let result: Option<String> = redis::cmd("SET")
            .arg(key)
            .arg(&token)
            .arg("NX")
            .arg("PX")
            .arg(ttl_ms)
            .query_async(&mut *conn)
            .await?;
        // if result.is_some() { Ok(Some(token)) } else { Ok(None) }
        Ok(result.map(|_| token))
    }
    ///仅允许锁的持有者释放锁
    async fn release(&self, key: &str, token: &str) -> Result<bool, Box<dyn std::error::Error>> {
        let mut conn = self.pool.get().await?;
        let script = Script::new(
            // 获取获取锁的值并比较
            // KEYS[1]：传入的第一个 Redis 键，也就是锁的 key，例如 lock:order:1001。
            // ARGV[1]：传入的第一个参数，也就是获取锁时生成的唯一 token。
            // GET：读取 Redis 中这个 key 对应的值。
            // 第二行：如果 token 匹配，就删除锁 redis.call("DEL", KEYS[1])成功返回1，若key不存在，DEL返回0。
            // 第四行：如果 token 不匹配，不执行删除。
            r#"
        if redis.call("GET", KEYS[1]) == ARGV[1] then
            return redis.call("DEL", KEYS[1])
        else
            return 0
        end
        "#,
        );
        let result: i32 = script.key(key).arg(token).invoke_async(&mut *conn).await?;
        Ok(result == 1)
    }
}

// 可重入锁，适用：同一业务调用链需要重复获取同一把锁
struct LockReentrant {}

//公平锁 / 排队锁，适用：多个请求必须按顺序获得锁

//读写锁（Read-Write Lock），适用：多个读者共享访问、写者独占

//Redlock 多节点分布式锁，适用：需要多 Redis 节点协调获取锁
