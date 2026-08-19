pub mod models;
pub mod schema;

use anyhow::{Context, Result};
use std::env;

use diesel::prelude::*;
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
use dotenvy::dotenv;

use crate::models::*;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

pub fn run_migrations(conn: &mut SqliteConnection) -> Result<()> {
    conn.run_pending_migrations(MIGRATIONS)
        .map_err(|e| anyhow::anyhow!("Migration failed: {}", e))?;
    Ok(())
}

pub fn establish_connection() -> Result<SqliteConnection> {
    dotenv().ok();

    let database_url = if let Ok(url) = env::var("DATABASE_URL") {
        url
    } else {
        let project_dirs = directories::ProjectDirs::from("com", "Reactor", "ReactorBrowser")
            .context("Could not determine local application data directory")?;

        let data_dir = project_dirs.data_local_dir();
        std::fs::create_dir_all(data_dir).context("Failed to create application data directory")?;

        let path = data_dir.join("reactor.db");
        path.to_str()
            .context("Database path is invalid UTF-8")?
            .to_string()
    };

    SqliteConnection::establish(&database_url)
        .with_context(|| format!("Error connecting to {database_url}"))
}

pub fn create_history(conn: &mut SqliteConnection, url: &str, title: Option<&str>) -> Result<()> {
    use crate::schema::history;

    let new_history = NewHistory { url, title };

    diesel::insert_into(history::table)
        .values(&new_history)
        .execute(conn)?;

    Ok(())
}

pub fn show_history(conn: &mut SqliteConnection) -> Result<Vec<History>> {
    use crate::schema::history::dsl::*;

    Ok(history.select(History::as_select()).load(conn)?)
}

pub fn delete_history(conn: &mut SqliteConnection, history_id: i32) -> Result<()> {
    use crate::schema::history::dsl::*;

    diesel::delete(history.find(history_id)).execute(conn)?;

    Ok(())
}

pub fn create_tab(conn: &mut SqliteConnection, url: &str, title: Option<&str>) -> Result<Tab> {
    use crate::schema::tabs;

    let new_tab = NewTab { url, title };

    let tab = diesel::insert_into(tabs::table)
        .values(&new_tab)
        .returning(Tab::as_returning())
        .get_result(conn)?;
    Ok(tab)
}

pub fn show_tabs(conn: &mut SqliteConnection) -> Result<Vec<Tab>> {
    use crate::schema::tabs::dsl::*;

    Ok(tabs.select(Tab::as_select()).load(conn)?)
}

pub fn update_tab(
    conn: &mut SqliteConnection,
    tab_id: i32,
    new_url: &str,
    new_title: Option<&str>,
) -> Result<()> {
    use crate::schema::tabs::dsl::*;

    diesel::update(tabs.find(tab_id))
        .set((url.eq(new_url), title.eq(new_title)))
        .execute(conn)?;

    Ok(())
}

pub fn delete_tab(conn: &mut SqliteConnection, tab_id: i32) -> Result<()> {
    use crate::schema::tabs::dsl::*;

    diesel::delete(tabs.find(tab_id)).execute(conn)?;

    Ok(())
}
