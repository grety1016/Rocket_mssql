//crypto modules:
pub use crypto::{digest::Digest, sha2::Sha256};
//rocket crate
pub use rocket::serde::{json::Json, Deserialize, Serialize};
//random number modules:
use rand::Rng;

//uuid module:
use uuid::Uuid;

//serde_json module:
use serde_json::json;

///use eventful crate:
use eventful::*;

///tracing module:
use tracing::info;

//use std::sync::atomic::{AtomicI16, Ordering},Arc;
use std::{
    any::Any,
    clone,
    fmt::Debug,
    sync::{
        atomic::{AtomicI16, Ordering},
        Arc, Mutex,
    },
};

///用于钉住一个动态类型在Future异步时的内存位置
use std::{future::Future, pin::Pin};

///引入全局事件分发器
use crate::EVENT_PUBLISH;

//创建事件主题
define_topic! {
    /// 主题User
    ["topic UserEvent"]
    TopicUser: User;
    ["topic PersonEvent"]
    TopicPerson: Person;
}

///用户类型
#[derive(Clone, Debug)]
pub struct User {
    name: String,
}
///用户实现
impl User {
    pub fn new(name: String) -> Self {
        User { name }
    }
    ////
    ///这是一个返回闭包的函数
    pub fn create_fn_mut(&self) -> Box<dyn FnMut(Message<TopicUser>) + Send + 'static> {
        Box::new(move |msg| {
            println!("user name: {},开车行驶中", msg.name);
        })
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    ///这是一个闭包中返回一个Future的
    pub fn create_fn_mut2(
        &self,
    ) -> Box<dyn FnMut(Message<TopicUser>) -> Pin<Box<dyn Future<Output = ()>>> + Send + 'static>
    {
        Box::new(move |msg| {
            Box::pin(async move {
                tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
                println!("user name: {},开车行驶中", msg.name);
            })
        })
    }
}

///人类类型
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Person {
    name: String,
}
///实现人类初始化
impl Person {
    pub fn new(name: String) -> Self {
        Person { name }
    }
    ////
    ///这是一个返回闭包的函数
    pub fn create_fn_mut(&self) -> Box<dyn FnMut(Message<TopicPerson>) + Send + 'static> {
        Box::new(move |msg| {
            println!("person name: {},开车行驶中", msg.name);
        })
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }

    ///这是一个闭包中返回一个Future的
    pub fn create_fn_mut2(
        &self,
    ) -> Box<dyn FnMut(Message<TopicPerson>) -> Pin<Box<dyn Future<Output = ()>>> + Send + 'static>
    {
        Box::new(move |msg| {
            Box::pin(async move {
                tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
                println!("person name: {},开车行驶中", msg.name);
            })
        })
    }
}
#[derive(Debug, Serialize, Deserialize)]
struct User2 {
    name: String,
    age: u8,
    phones: Vec<String>,
}
///获取Host配置
pub async fn get_dbhost() -> String {
    let host = std::env::var("DB_HOST").unwrap();
    host
}
///Hash加密
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
//生成UUID方法
pub fn uuid_fn() {
    let uuid = Uuid::new_v4();
    println!("UUID: {}", uuid);
    let my_uuid = Uuid::parse_str(uuid.to_string().as_str()).unwrap();
    println!("uuid: {:#?}", my_uuid);
}
///系列化操作
pub fn serialize_fn() {
    let person = Person {
        name: "John".to_string(),
    };

    let user = r#"{"name": "John Doe",
    "age": 43,
    "phones": [
        "+44 1234567",
        "+44 2345678"
    ]}"#;

    let user_json: User2 = serde_json::from_str(&user).unwrap();

    println!("Serialized: {:#?}", Json(user_json));

    println!("Serialized: {:#?}", Json(person));
}

///获取类型名称方法
pub fn print_simple_type_of<T>(_: &T) {
    let full_name = std::any::type_name::<T>();
    let name_parts: Vec<&str> = full_name.rsplit("::").collect();
    let mut simple_name = name_parts[0];
    if simple_name.ends_with('>') {
        simple_name = &simple_name[..simple_name.len() - 1];
    }
    println!("{}", simple_name);
}

///日志追踪
fn init() {
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(tracing::Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();
}
///事件机制方法
pub async fn eventful_fn() {
    init();

    let user = User::new("grety".to_string());
    let person: Person = Person::new("human".to_string());

    //多线程从全局事件分发器共享获取资源

    // {
    //     let mut eventful_opt  = EVENT_PUBLISH.lock().unwrap();
    //     if let Some(eventful) = eventful_opt.as_ref() {
    //         eventful.subscribe_async(TopicUser,user.create_fn_mut2());
    //     }
    // }

    // {
    //     let mut eventful_opt  = EVENT_PUBLISH.lock().unwrap();
    //     if let Some(eventful) = eventful_opt.as_ref() {
    //         eventful.publish(TopicUser,user);
    //     }
    // }
    // {
    //     let mut eventful_opt  = EVENT_PUBLISH.lock().unwrap();
    //     if let Some(eventful) = eventful_opt.take() {
    //         eventful.shutdown();

    //     }
    // }

    //非全局变量使用
    let user_clone = user.clone();
    let person_clone = person.clone();

    info!("testing……");

    {
        let eventful_opt = EVENT_PUBLISH.lock().unwrap();
        if let Some(eventful) = eventful_opt.as_ref() {
            eventful.subscribe_async(TopicUser, user_clone.create_fn_mut2());
        }
    }
    //tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    {
        let eventful_opt = EVENT_PUBLISH.lock().unwrap();
        if let Some(eventful) = eventful_opt.as_ref() {
            eventful.subscribe_async(TopicPerson, person_clone.create_fn_mut2());
        }
    }
    //tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    {
        let eventful_opt = EVENT_PUBLISH.lock().unwrap();
        if let Some(eventful) = eventful_opt.as_ref() {
            eventful.publish(TopicUser, user);
        }
    }
    {
        let eventful_opt = EVENT_PUBLISH.lock().unwrap();
        if let Some(eventful) = eventful_opt.as_ref() {
            eventful.publish(TopicPerson, person);
        }
    }

    {
        let mut eventful_opt = EVENT_PUBLISH.lock().unwrap();
        if let Some(eventful) = eventful_opt.take() {
            eventful.shutdown();
        }
    }
}

//下列函数来源于rust_new
