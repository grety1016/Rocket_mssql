//rocket use modules:
#[allow(unused_imports)]
use rocket::{self,build,get,post,launch,routes};
#[allow(unused_imports)]
use rocket::tokio::{task,time};

//std use modules:
use std::env;

//database config module
pub mod db_config;

//crypto modules:
use crypto::{digest::Digest, sha2::Sha256};
 

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
    crypto_hash();
    
}

async fn get_dbhost() -> String {
    let host = std::env::var("DB_HOST").unwrap();
    host
}

fn crypto_hash() {
    let mut hasher = Sha256::new();
    hasher.input_str("helloword");
    let hex = hasher.result_str();
    println!("pwd into hash: {}", hex);
     
}
//f0da559ea59ced68b4d657496bee9753c0447d70702af1a351c7577226d97723
//0b322d15ea034793a8646baa1744ffacbdf7f959b66c68970f032c4ac8b9c8cb