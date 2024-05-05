//rocket use modules:
#[allow(unused_imports)]
use rocket::{self,build,get,post,launch,routes};
#[allow(unused_imports)]
use rocket::tokio::{task,time};

//extern use modules:
use lazy_static::lazy_static;

lazy_static! {
    static ref NUMBERS:u32 = 0;
}

#[get("/")]
fn index() -> &'static str{
    "Hello, world!"
}


#[launch]
async fn rocket() -> _ {
    rocket::build()
    .mount("/",routes![index])
}