#[macro_use]
extern crate rocket;

#[macro_use]
extern crate log;

extern crate pretty_env_logger;

use dotenvy::dotenv;

use sqlx::postgres::PgPoolOptions;

mod cors;
mod handlers;
mod models;
mod persistance;

use cors::*;
use handlers::*;

#[launch]
async fn rocket() -> _ {
    pretty_env_logger::init();
    dotenv().ok();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&std::env::var("DATABASE_URL").expect("DATABASE_URL must be set."))
        .await
        .expect("Failed to create Postgres connection pool!");
    
    // TODO: Delete this query
    let recs = sqlx::query!("SELECT * FROM questions")
            .fetch_all(&pool)
            .await
            .unwrap();

    // TODO: Delete these log statements
    info!("********* Question Records *********");
    info!("{:?}", recs);

    rocket::build()
        .mount(
            "/",
            routes![
                create_question,
                read_questions,
                delete_question,
                create_answer,
                read_answers,
                delete_answer
            ],
        )
        .attach(CORS)
}
