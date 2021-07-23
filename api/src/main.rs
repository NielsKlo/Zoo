use actix_web::{HttpServer, HttpResponse, App, Responder, get, post};
use actix_session::{CookieSession, Session};
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
            .wrap(CookieSession::signed(&[0; 32]).secure(false))
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
async fn get_animal(session: Session) -> impl Responder {
    let message = database::get_animal();
    let deserialized: DomainGameState = serde_json::from_str(&message).unwrap();
    session.set("animal", deserialized).expect("couldn't store game state in session");

    HttpResponse::Ok().content_type("application/json").body(message)
}

#[get("/save_animal")]
async fn save_animal(session: Session) -> impl Responder {
    let game_state = session.get::<DBGameState>("animal").unwrap().unwrap();
    database::save_animal(game_state);

    HttpResponse::Ok()
}

#[get("/tick_forward")]
async fn tick_forward(session: Session) -> impl Responder {
    let mut game_state = session.get::<DomainGameState>("animal").unwrap().unwrap();
    game_state.tick_forward();
    session.set("animal", &game_state).expect("Couldn't replace game state");
    let message = serde_json::to_string(&game_state).unwrap();

    HttpResponse::Ok().content_type("application/json").body(message)
}

#[post("/feed_animal")]
async fn feed_animal(session: Session) -> impl Responder {
    let mut game_state = session.get::<DomainGameState>("animal").unwrap().unwrap();
    game_state.feed_animal();
    session.set("animal", &game_state).expect("Couldn't replace game state");
    let message = serde_json::to_string(&game_state).unwrap();

    HttpResponse::Ok().content_type("application/json").body(message)
}