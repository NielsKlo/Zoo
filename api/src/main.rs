use actix_web::{HttpServer, HttpResponse, App, Responder, get, post};
use actix_session::{CookieSession, Session};
use std::io;
use database;
use database::models::game_state::GameState as DBGameState;
use domain::GameState as DomainGameState;
use serde_json::from_str;

extern crate serde_json;

#[actix_web::main]
async fn main() -> io::Result<()>{
    println!("Starting server at http://127.0.0.1:8080/");

    HttpServer::new(|| {
        App::new()
            .wrap(CookieSession::signed(&[0; 32]).secure(false))
            .service(get_animals)
            .service(save_animals)
            .service(tick_forward)
            .service(feed_animal)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

#[post("/get_animals")]
async fn get_animals(session: Session, name: String) -> impl Responder {
    let message = database::get_animals(name);
    let deserialized: DomainGameState = serde_json::from_str(&message).unwrap();
    session.set("game", deserialized).expect("Couldn't store game state in session");

    HttpResponse::Ok().content_type("application/json").body(message)
}

#[get("/save_animals")]
async fn save_animals(session: Session) -> impl Responder {
    let game_state = session.get::<DBGameState>("game").unwrap().unwrap();
    database::save_animal(game_state);

    HttpResponse::Ok()
}

#[get("/tick_forward")]
async fn tick_forward(session: Session) -> impl Responder {
    let mut game_state = session.get::<DomainGameState>("game").unwrap().unwrap();
    game_state.tick_forward();
    session.set("game", &game_state).expect("Couldn't replace game state");

    let message = serde_json::to_string(&game_state).unwrap();
    HttpResponse::Ok().content_type("application/json").body(message)
}

#[post("/feed_animal")]
async fn feed_animal(session: Session, id: String) -> impl Responder {
    let mut game_state = session.get::<DomainGameState>("game").unwrap().unwrap();
    game_state.feed_animal(from_str::<usize>(&id).unwrap());
    session.set("game", &game_state).expect("Couldn't replace game state");

    let message = serde_json::to_string(&game_state).unwrap();
    HttpResponse::Ok().content_type("application/json").body(message)
}