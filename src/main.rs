//rocket use modules:
//Rocket启动，路由注册，路由分配等
#[allow(unused_imports)]
use rocket::{self, build, get, launch, post, routes};
//Rocket内嵌的tokio异步运行时
#[allow(unused_imports)]
use rocket::tokio::{task, time};

//std use modules:
use std::{
    env,
    sync::{Arc, Mutex},
};

//Deserialize Serialize
use serde::{Deserialize, Serialize};

//local lib module:
pub mod db_config;
pub mod testmod;

///eventful module:
use eventful::*;

//local lib use:
use testmod::*;
use testmod::{crypto_hash, eventful_fn, get_dbhost, serialize_fn, uuid_fn};

//extern use modules:
use lazy_static::lazy_static;

lazy_static! {
    ///一个数字全局变量
    static ref NUMBERS:u32 = 0;
    ///一个用于全局多线程的事件分发器
    static ref EVENT_PUBLISH:Arc<Mutex<Eventful>> = Arc::new(Mutex::new(Eventful::new()));
}

#[get("/<name>")]
fn index(name: &str) -> String {
    let return_str = format!("Hello, world,{}!", name);
    return_str
}

// #[launch]
// async fn rocket() -> _ {
//     rocket::build()
//     .mount("/",routes![index])
// }

#[tokio::main]
async fn main() {
    // println!("host:{}",get_dbhost()).await;

    // crypto_hash();

    // uuid_fn();

    // serialize_fn();

    eventful_fn().await;
}
