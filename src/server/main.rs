#![feature(proc_macro_hygiene, decl_macro)]

use rocket::State;
use std::fs;
use std::process::Command;

mod config;
use config::Config;

#[macro_use]
extern crate rocket;

#[get("/meeting?<url>")]
fn meeting(config: State<Config>, url: Option<String>) -> &'static str {
    println!("{}", url.clone().unwrap());

    Command::new(config.zoom_bin_path.to_string())
        .arg(format!("{}={}", "--url", url.unwrap().to_string()))
        .spawn()
        .expect("failed to execute process");

    "launched zoom meeting!"
}

fn main() {
    let config_file = fs::read_to_string("./config.yml".to_string()).unwrap();
    let config: Config = serde_yaml::from_str(&config_file).unwrap();

    println!("using zoom binary path: {}", config.zoom_bin_path);

    rocket::ignite()
        .manage(config)
        .mount("/", routes![meeting])
        .launch();
}
