//Rocket启动，路由注册，路由分配等
#[allow(unused_imports)]
use rocket::{self, build, get, launch, post, routes,fairing::AdHoc};
//Rocket内嵌的tokio异步运行时
#[allow(unused_imports)]
use rocket::tokio::{task, time};
//标准库:
use std::{
    env,
    sync::{Arc, Mutex},
};
//系列与反系列化宏
use serde::{Deserialize, Serialize};
//本地定义的模块:
pub mod db_config;
pub mod testmod;

///事件驱动库:
use eventful::*;

//本地对象引入:
use testmod::*;
use testmod::{crypto_hash, eventful_fn, get_dbhost, serialize_fn, uuid_fn};

//声明静态变量库:
use lazy_static::lazy_static;

#[derive(Deserialize)]
struct Appconfig{
    address:String,
    port:u16
}

impl Appconfig  {
    pub fn new(address:String, port:u16) -> Self {
        Appconfig{address, port}
    }
}

lazy_static! {
    ///一个数字全局变量
    static ref NUMBERS:u32 = 0;
    ///一个用于全局多线程的事件分发器
    static ref EVENT_PUBLISH:Arc<Mutex<Option<Eventful>>> = Arc::new(Mutex::new(Some(Eventful::new())));
}

#[get("/")]
fn index() -> String {
    let return_str = format!("Hello, world,grety!",);
    return_str
}

#[launch]
async fn rocket() -> _ { 
    rocket::build() 
    .mount("/", routes![index])
}

//#[tokio::main]
// async fn main() {
//     // println!("host:{}",get_dbhost()).await;

//     // crypto_hash();

//     // uuid_fn();

//     // serialize_fn();

//     eventful_fn().await;
// }
