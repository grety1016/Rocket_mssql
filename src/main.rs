//rocket use modules:
//Rocket启动，路由注册，路由分配等
#[allow(unused_imports)]
use rocket::{self,build,get,post,launch,routes};
//Rocket内嵌的tokio异步运行时
#[allow(unused_imports)]
use rocket::tokio::{task,time};

//std use modules:
use std::env;

//Deserialize Serialize
use serde::{Serialize,Deserialize};

//local lib module:
pub mod db_config;
pub mod testmod;


//local lib use:
use testmod:: {crypto_hash,get_dbhost,uuid_fn,serialize_fn};

 
 
 

//extern use modules:
use lazy_static::lazy_static;

lazy_static! {
    static ref NUMBERS:u32 = 0;
}

#[get("/<name>")]
fn index(name:& str) -> String{
   let return_str = format!( "Hello, world,{}!",name);
   return_str
}


// #[launch]
// async fn rocket() -> _ {
//     rocket::build()
//     .mount("/",routes![index])
// }

#[tokio::main]
async fn main() {
    //println!("host:{}",get_dbhost()).await;

    //crypto_hash();

    //uuid_fn();

    //serialize_fn();

    ///测试内容是否会保存到之前的分支
    
}