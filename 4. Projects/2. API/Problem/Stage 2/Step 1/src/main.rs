#[macro_use]
extern crate rocket;

// TODO: import log, pretty_env_logger, dotenv, and PgPoolOptions

mod cors;
mod handlers;
mod models;

use cors::*;
use handlers::*;

#[launch]
async fn rocket() -> _ {
    // TODO: Initialize pretty_env_logger
    // TODO: Initialize dotenv

    // Create a new PgPoolOptions instance with a maximum of 5 connections.
    // Use dotenv to get the database url. 
    // Use the `unwrap` or `expect` method instead of handling errors. If an
    // error occurs at this stage the server should be terminated. 
    // See examples on GitHub page: https://github.com/launchbadge/sqlx
    // let pool = todo!();

    // Using slqx, execute a SQL query that selects all questions from the questions table.
    // Use the `unwrap` or `expect` method to handle errors. This is just some test code to
    // make sure we can connect to the database.  
    // let recs = todo!();

    info!("********* Question Records *********");
    // TODO: Log recs with debug formatting using the info! macro

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
