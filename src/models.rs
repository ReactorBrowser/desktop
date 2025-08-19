use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::history)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct History {
    pub id: i32,
    pub url: String,
    pub title: Option<String>,
    pub visited_at: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::history)]
pub struct NewHistory<'a> {
    pub url: &'a str,
    pub title: Option<&'a str>,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::tabs)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Tab {
    pub id: i32,
    pub url: String,
    pub title: Option<String>,
    pub loaded: bool,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::tabs)]
pub struct NewTab<'a> {
    pub url: &'a str,
    pub title: Option<&'a str>,
}
