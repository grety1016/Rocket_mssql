//crypto modules:
pub use crypto::{digest::Digest, sha2::Sha256};
//rocket crate
pub use rocket::serde::{json::Json,Serialize,Deserialize};
//random number modules: 
use rand::Rng;

//uuid module:
use uuid::Uuid;

//serde_json module:
use serde_json::json;

//use eventful crate:
use eventful::*;

//tracing module:
use tracing::info;

//use std::sync::atomic::{AtomicI16, Ordering},Arc;
use std::sync::{atomic::{AtomicI16, Ordering}, Arc};

use std::{future::Future, pin::Pin};




////事件User相关的定义
        //创建事件主题
        define_topic! {
            /// 主题User
            ["topic User"]
            TopicUser: User;    
        }

        #[derive(Clone)]
        pub struct User {
            name: String,
        }

        impl User {
            pub fn new(name: String) -> Self {
                User { name }
            }
            ////
            ///这是一个返回闭包的函数
            pub fn create_fn_mut(&self) -> Box<dyn FnMut(Message<TopicUser>) + Send + 'static> {
                Box::new(move |msg| {
                   
                        println!("user name1: {},开车行驶中", msg.name);
                    })
            }

            pub fn get_name(&self) -> &str {
                &self.name
            }

            pub fn set_name(&mut self, name: &str) {
                self.name = name.to_string();
            }


            ///这是一个闭包中返回一个Future的
            pub fn create_fn_mut2(&self) -> Box<dyn FnMut(Message<TopicUser>) -> Pin<Box<dyn Future<Output = ()>>> + Send + 'static> {
                Box::new(move |msg| {
                    Box::pin(async move {
                        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
                        println!("user name2: {},开车行驶中", msg.name);
                    })
                })
            }

            

        }


 



#[derive(Debug, Serialize, Deserialize)]
struct Person {
    name: String,
    age:u8
}
#[derive(Debug, Serialize, Deserialize)]
struct User2 {
    name: String,
    age:u8,
    phones: Vec<String>
}

pub async fn get_dbhost() -> String {
    let host = std::env::var("DB_HOST").unwrap();
    host
}

pub fn crypto_hash() {
    let mut hasher = Sha256::new();
    
    // //以下两行仅用于需要增加随机数进行hash时使用，使用时请保存生成的随机数，否则无法得到最初的hash值。
    // let mut rng = rand::thread_rng();
    // let salt: [u8; 8] = rng.gen();
    // println!("salt is {:?}",salt);

    hasher.input_str("kephi520.");
    let hex = hasher.result_str();
    println!("pwd into hash: {}", hex);     
}

pub fn uuid_fn() {
    let mut uuid = Uuid::new_v4();
    println!("UUID: {}", uuid);
    let my_uuid = Uuid::parse_str(uuid.to_string().as_str()).unwrap();
    println!("uuid: {:#?}", my_uuid);
}

pub fn serialize_fn() {
    let person = Person {
        name: "John".to_string(),
        age: 28,
    };

    let user = r#"{"name": "John Doe",
    "age": 43,
    "phones": [
        "+44 1234567",
        "+44 2345678"
    ]}"#;

    let user_json : User2 = serde_json::from_str(&user).unwrap();

    println!("Serialized: {:#?}", Json(user_json));

    println!("Serialized: {:#?}", Json(person));
}

fn init() {
    let subscriber =
        tracing_subscriber::FmtSubscriber::builder().with_max_level(tracing::Level::INFO).finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();
}

pub async fn eventful_fn() {
    init();

        // tokio::task::spawn(
        //     async move{ 
        //         loop  {
        //             let user_event = USER_EVENT.lock().unwrap();
    
        //             user_event.publish(TopicUser,user_clone);
        //         }
        //     }
        // );
    
        // let user_event = USER_EVENT.lock().unwrap();
        // user_event.subscribe(TopicUser,user.create_fn_mut());
        // user_event.subscribe_async(TopicUser,user.create_fn_mut2());

}