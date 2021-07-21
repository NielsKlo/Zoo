use actix_web::{HttpServer, HttpResponse, web, App, Responder, get, post};
use std::io;
use database;
use database::models::game_state::GameState as DBGameState;

extern crate serde_json;

#[actix_web::main]
async fn main() -> io::Result<()>{
    println!("Starting server at http://127.0.0.1:8080/");

    HttpServer::new(|| {
        App::new()
            .service(get_animal)
            .service(save_animal)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

#[get("/animal")]
async fn get_animal() -> impl Responder {
    let message = database::get_animal();

    HttpResponse::Ok().content_type("application/json").body(message)
}

#[post("/save_animal")]
async fn save_animal(json: web::Json<DBGameState>) -> impl Responder {
    let game_state = json.into_inner();
    database::save_animal(game_state);

    HttpResponse::Ok()
}