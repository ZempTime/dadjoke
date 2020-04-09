use diesel::prelude::*;
use uuid::Uuid;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

use crate::models::{NewDadjoke, Dadjoke};

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn get_all_dadjokes() -> QueryResult<Vec<Dadjoke>> {
    let conn = establish_connection();
    Dadjoke::all(&conn)
}

pub fn find_dadjoke(id: String) -> QueryResult<Dadjoke> {
    let conn = establish_connection();
    Dadjoke::find_with_id(id, &conn)
}

pub fn create_dadjoke(joketext: &str, conn: &PgConnection) -> QueryResult<Dadjoke> {
    let conn = establish_connection();

    let new_dadjoke = NewDadjoke {
        id: Uuid::new_v4().to_string(),
        text: joketext.to_owned(),
    };

    Dadjoke::insert(new_dadjoke, &conn)
}

pub fn update_dadjoke(dadjoke: Dadjoke, conn: &PgConnection) -> QueryResult<Dadjoke> {
    let conn = establish_connection();
    Dadjoke::update(dadjoke, conn)
}

pub fn delete_dadjoke(id: String, conn: &PgConnection) -> QueryResult<usize> {
    let conn = establish_connection();
    Dadjoke::delete_with_id(id, &conn)
}