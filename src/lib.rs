pub mod models;
pub mod schema;

use std::env;

use diesel::prelude::*;
use dotenvy::dotenv;

use crate::models::*;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connection to: {}", database_url))
}

pub fn create_history(
    conn: &mut SqliteConnection,
    url: &str,
    title: Option<&str>,
) -> Result<(), diesel::result::Error> {
    use crate::schema::history;

    let new_history = NewHistory { url, title };

    diesel::insert_into(history::table)
        .values(&new_history)
        .execute(conn)?;

    Ok(())
}

pub fn show_history(conn: &mut SqliteConnection) -> Result<Vec<History>, diesel::result::Error> {
    use crate::schema::history::dsl::*;

    Ok(history.select(History::as_select()).load(conn)?)
}

pub fn delete_history(
    conn: &mut SqliteConnection,
    history_id: i32,
) -> Result<(), diesel::result::Error> {
    use crate::schema::history::dsl::*;

    diesel::delete(history.filter(id.eq(history_id))).execute(conn)?;

    Ok(())
}
