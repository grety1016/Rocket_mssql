//crypto modules:
pub use crypto::{digest::Digest, sha2::Sha256};
//rocket crate
pub use rocket::serde::{json::Json,Serialize,Deserialize};
//random number modules: 
use rand::Rng;

//uuid module:
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
struct Person {
    name: String,
    age:u8
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
    println!("Serialized: {:#?}", Json(person));
}