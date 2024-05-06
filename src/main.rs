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
    let host = std::env::var("DB_HOST").unwrap();
    println!("db_host: {:?}",host);
    
}