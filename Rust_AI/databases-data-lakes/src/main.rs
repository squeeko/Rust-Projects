extern crate diesel;
use diesel::pg::PgConnection;
//use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set!");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url));
}

fn main() {
    let connection = establish_connection();
    // Database interactions go here....
    println!("");
}
