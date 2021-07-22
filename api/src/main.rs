use actix_web::{HttpServer, HttpResponse, web, App, Responder, get, post};
use std::io;
use database;
use database::models::game_state::GameState as DBGameState;
use domain::GameState as DomainGameState;

extern crate serde_json;

#[actix_web::main]
async fn main() -> io::Result<()>{
    println!("Starting server at http://127.0.0.1:8080/");

    HttpServer::new(|| {
        App::new()
            .service(get_animal)
            .service(save_animal)
            .service(tick_forward)
            .service(feed_animal)
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

#[post("/tick_forward")]
async fn tick_forward(json: web::Json<DomainGameState>) -> impl Responder {
    let mut game_state = json.into_inner();
    game_state.tick_forward();
    println!("{:?}", game_state);
    let message = serde_json::to_string(&game_state).unwrap();

    HttpResponse::Ok().content_type("application/json").body(message)
}

#[post("/feed_animal")]
async fn feed_animal(json: web::Json<DomainGameState>) -> impl Responder {
    let mut game_state = json.into_inner();
    game_state.feed_animal();
    let message = serde_json::to_string(&game_state).unwrap();

    HttpResponse::Ok().content_type("application/json").body(message)
}