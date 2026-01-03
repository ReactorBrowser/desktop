pub mod models;
pub mod schema;

use std::{env, error::Error};

use diesel::prelude::*;
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
use dotenvy::dotenv;

use crate::models::*;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

pub fn run_migrations(
    conn: &mut SqliteConnection,
) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    conn.run_pending_migrations(MIGRATIONS)?;

    Ok(())
}

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

    diesel::delete(history.find(history_id)).execute(conn)?;

    Ok(())
}

pub fn create_tab(
    conn: &mut SqliteConnection,
    url: &str,
    title: Option<&str>,
) -> Result<Tab, diesel::result::Error> {
    use crate::schema::tabs;

    let new_tab = NewTab { url, title };

    let tab = diesel::insert_into(tabs::table)
        .values(&new_tab)
        .returning(Tab::as_returning())
        .get_result(conn)?;
    Ok(tab)
}

pub fn show_tabs(conn: &mut SqliteConnection) -> Result<Vec<Tab>, diesel::result::Error> {
    use crate::schema::tabs::dsl::*;

    Ok(tabs.select(Tab::as_select()).load(conn)?)
}

pub fn load_tab(conn: &mut SqliteConnection, tab_id: i32) -> Result<(), diesel::result::Error> {
    use crate::schema::tabs::dsl::*;

    diesel::update(tabs.find(tab_id))
        .set(loaded.eq(true))
        .execute(conn)?;

    Ok(())
}

pub fn unload_tab(conn: &mut SqliteConnection, tab_id: i32) -> Result<(), diesel::result::Error> {
    use crate::schema::tabs::dsl::*;

    diesel::update(tabs.find(tab_id))
        .set(loaded.eq(false))
        .execute(conn)?;

    Ok(())
}

pub fn delete_tab(conn: &mut SqliteConnection, tab_id: i32) -> Result<(), diesel::result::Error> {
    use crate::schema::tabs::dsl::*;

    diesel::delete(tabs.find(tab_id)).execute(conn)?;

    Ok(())
}
