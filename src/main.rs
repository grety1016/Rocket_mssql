//Rocket启动，路由注册，路由分配等
#[allow(unused_imports)]
use rocket::{self, build, get, launch, post, routes,fairing::AdHoc,http::Method};
//Rocket内嵌的tokio异步运行时
#[allow(unused_imports)]
use rocket::tokio::{task, time};
//rocket_cors跨域同源策略
use rocket_cors::{AllowedOrigins,CorsOptions};
//Tera模板
use tera::Template;
//标准库:
use std::{
    env,
    sync::{Arc, Mutex},
};
//系列与反系列化宏
use serde::{Deserialize, Serialize};
use serde_json::{json,Value};
//本地定义的模块:
pub mod db_config;
pub mod testmod;
///日志追踪库:
pub use tracing::info;

///事件驱动库:
use eventful::*;

//本地对象引入:
use testmod::*;
use testmod::{crypto_hash, eventful_fn, get_dbhost, serialize_fn, uuid_fn,init};

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

///下列引用用于websocket
use rocket::http::Status;
use rocket::response::status;
use rocket::tokio::sync::broadcast;
use rocket::State;
use rocket_ws::{WebSocket, stream::DuplexStream}; 
use rocket::futures::{SinkExt, StreamExt};


// #[get("/ws?<name>&<age>")]
// async fn ws<'r>(ws:WebSocket,name:&'r str,age:u8) -> rocket_ws::Channel<'r> {
//     ws.channel(move |mut stream| Box::pin(async move {        
//         let message = format!("Hello!{:#?},{}",name,age); 
//             let object = json!({
//                 "name":name,
//                 "age":age,
//             }); 
            
//             let object = serde_json::to_string(&object).unwrap();      
         
//             let message_clone = message.clone();
//             tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
//             let _ = stream.send(object.into()).await;            
         
//         Ok(())
//     }))
// }


#[get("/ws2")]
async fn ws2(ws:WebSocket) -> rocket_ws::Channel<'static> {
    ws.channel(move |mut stream| Box::pin(async move {     
            //tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            let _ = stream.send("hello".into()).await;            
         
        Ok(())
    }))
}





#[get("/?<name>&<age>&<pwd>")]
fn index<'r>(name:&'r str,age:u8,pwd:String) -> String {    

    info!("name: {:?},age:{:?},pwd:{:?}",name,age,pwd);

    format!("name: {:?},age:{:?},pwd:{:?}",name,age,pwd)
}

#[launch]
async fn rocket() -> _ {
    //使用rocket_cors处理跨域同源策略问题：
    let allowed_origins = AllowedOrigins::all();
    //cors请求处理配置
    let cors = CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post].into_iter().map(From::from).collect(),
        allowed_headers: rocket_cors::AllowedHeaders::all(),
        allow_credentials: true,
        ..Default::default()
    }.to_cors()
    .expect("CORS configuration failed");

    //初始化trancing日志追踪
    init(); 

    //rocket服务器启动
    rocket::build() 
    .attach(cors)
    .mount("/", routes![index,ws2])
}

//#[tokio::main]
// async fn main() {
//     // println!("host:{}",get_dbhost()).await;

//     // crypto_hash();

//     // uuid_fn();

//     // serialize_fn();

//     eventful_fn().await;
// }
