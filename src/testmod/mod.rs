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

///use std::sync::atomic::{AtomicI16, Ordering},Arc;
use std::{
    any::Any,
    clone,
    fmt::Debug,
    sync::{
        atomic::{AtomicI16, Ordering},
        Arc,
    },
};

///用于钉住一个动态类型在Future异步时的内存位置
use std::{future::Future, pin::Pin};

///创建事件主题
define_topic! {
    /// 主题User
    ["topic EventPack"]
    TopicEventPack: EventPack;
}

///声明一个标记Trait
pub trait MarkTrait: Send + Debug  + Clone + 'static {}
///为不同类型实现标记Trait
impl MarkTrait for User {}
impl MarkTrait for Person {}

//枚举用于存储事件所针对的不同类型
pub enum MarkTraitEnum {
    User,
    Person,
}


///用于保存事件传递时需要的变量类型 
pub struct EventPack {
    name: String,
    object: MarkTraitEnum,
    eventfn: Box<dyn FnMut(Message<TopicEventPack>) + Send + 'static>,
}
impl EventPack {
    pub fn new<T, F>(name: String, object: MarkTraitEnum, eventfn: Box<F>) -> Self
    where
        T: MarkTrait + 'static,
        F: FnMut(Message<TopicEventPack>) + Send + 'static,
    {
        EventPack {
            name,
            object,
            eventfn,
        }
    }
}
///实现事件包类型初始化方法
// impl EventPack {
//         pub fn new(name: String, object: Box<dyn MarkTrait>,eventfn:Box<dyn FnMut(Message<TopicEventPack>) + Send + 'static>) -> Self
//         {
//             EventPack { name, object,eventfn }
//         }

//     }

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
    pub fn create_fn_mut(&self) -> Box<dyn FnMut(Message<TopicEventPack>) + Send + 'static> {
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
    pub fn create_fn_mut2(
        &self,
    ) -> Box<dyn FnMut(Message<TopicEventPack>) -> Pin<Box<dyn Future<Output = ()>>> + Send + 'static>
    {
        Box::new(move |msg| {
            Box::pin(async move {
                tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
                println!("user name2: {},开车行驶中", msg.name);
            })
        })
    }
}

///人类类型
#[derive(Debug, Serialize, Deserialize, Clone)]
struct Person {
    name: String,
}
///实现人类初始化
impl Person {
    pub fn new(name: String) -> Self {
        Person { name }
    }
    ////
    ///这是一个返回闭包的函数
    pub fn create_fn_mut(&self) -> Box<dyn FnMut(Message<TopicEventPack>) + Send + 'static> {
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
    pub fn create_fn_mut2(
        &self,
    ) -> Box<dyn FnMut(Message<TopicEventPack>) -> Pin<Box<dyn Future<Output = ()>>> + Send + 'static>
    {
        Box::new(move |msg| {
            Box::pin(async move {
                tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
                println!("user name2: {},开车行驶中", msg.name);
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
    let mut uuid = Uuid::new_v4();
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
    let mut vec_object: Vec<EventPack> = Vec::new();
    let user = User::new("grety".to_string());
    let person: Person = Person::new("human".to_string());
    let eventpack_user = EventPack::new(
        user.get_name().to_string(),
        Box::new(user.clone()),
        Box::new(user.create_fn_mut()),
    );

    let eventpack_person = EventPack::new(
        person.get_name().to_string(),
        Box::new(person.clone()),
        Box::new(person.create_fn_mut()),
    );

    //vec_object.push(eventpack_user);
    //vec_object.push(eventpack_person);
    
    let eventful = Eventful::new();
    eventful.subscribe(TopicEventPack, eventpack_user.eventfn);
    //eventful.publish(TopicEventPack, eventpack_user);




    //print_simple_type_of(&user1);

    // for item in vec_object.iter() {

    //     info!("{:#?}",item.object);
    // }

    // let user = User::new("joke".to_string());
    // let eventful = Eventful::new();

    // eventful.subscribe(TopicUser, user.create_fn_mut());

    // let user_clone = user.clone();
    // eventful.publish(TopicUser, user_clone);

    // eventful.shutdown();

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
