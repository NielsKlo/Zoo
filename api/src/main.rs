use actix_web::{HttpServer, HttpResponse, App, Responder, get};
use std::io;
use domain;
use domain::{Animal, GameState};
extern crate serde_json;

#[actix_web::main]
async fn main() -> io::Result<()>{

    println!("Starting server at http://127.0.0.1:8080/");

    HttpServer::new(|| {
        App::new()
            .service(get_animal)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

#[get("/animal")]
async fn get_animal() -> impl Responder {
    let animal = Animal { species: "Penguin".to_string(), age: 10, hunger: 50};
    let game_state = GameState {animal};
    let message = serde_json::to_string(&game_state).unwrap();
    println!("{}", &message);
    HttpResponse::Ok().content_type("application/json").body(message)
}
