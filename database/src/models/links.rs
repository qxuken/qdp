use diesel::prelude::*;
use diesel::sqlite::Sqlite;
use crate::schema;

#[derive(Queryable, Identifiable, Selectable, Debug, PartialEq)]
#[diesel(table_name = schema::link_section)]
#[diesel(check_for_backend(Sqlite))]
pub struct LinkSection {
    pub id: i32,
    pub title: String,
}

#[derive(Queryable, Selectable, Identifiable, Associations, Debug, PartialEq)]
#[diesel(belongs_to(LinkSection))]
#[diesel(table_name = schema::link_item)]
#[diesel(check_for_backend(Sqlite))]
pub struct LinkItem {
    pub id: i32,
    pub title: String,
    pub link: String,
    pub link_section_id: i32,
}
