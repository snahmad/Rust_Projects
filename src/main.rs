#[macro_use] extern crate rocket;


use rocket::{State, Config};
use rocket::fairing::AdHoc;
use rocket::serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(crate = "rocket::serde")]
struct AppConfig {
    key: String,
    port: u16
}

#[get("/")]
fn read_config(rocket_config: &Config, app_config: &State<AppConfig>) -> String {
    format!("{:#?}\n{:#?}", app_config, rocket_config)
}

#[get("/<name>/<age>")]
fn hello(name: &str, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}



#[launch]
fn rocket() -> _ {
    
 

    rocket::build()
    .mount("/", routes![read_config])
    .attach(AdHoc::config::<AppConfig>())

    //rocket::build().mount("/hello", routes![hello])
}