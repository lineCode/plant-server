use actix_web::{web, App, HttpServer};
use mongodb::{options::ClientOptions, Client};
use std::env;
use std::sync::*;

mod logs_handlers;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug");
    let mongo_url = env::var("CONNECTION_STRING_LOGS").unwrap();
    println!("Using connection string: {}", mongo_url);
    let mut client_options = ClientOptions::parse(&mongo_url).await.unwrap();
    client_options.app_name = Some("PlantApi".to_string());
    let client = web::Data::new(Mutex::new(Client::with_options(client_options).unwrap()));

    HttpServer::new(move || {
        App::new()
            .app_data(client.clone())
            .service(web::scope("/api").configure(logs_handlers::scoped_config))
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
