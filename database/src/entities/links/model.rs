use serde::{Deserialize, Serialize};

use diesel::prelude::*;
use diesel::sqlite::Sqlite;

use crate::schema;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable, Selectable, PartialEq)]
#[diesel(table_name = schema::link_section)]
#[diesel(check_for_backend(Sqlite))]
pub struct LinkSection {
    pub id: i32,
    pub title: String,
    pub order_number: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = schema::link_section)]
#[diesel(check_for_backend(Sqlite))]
pub struct NewLinkSection {
    pub title: String,
    pub order_number: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, AsChangeset, PartialEq)]
#[diesel(table_name = schema::link_section)]
#[diesel(check_for_backend(Sqlite))]
pub struct UpdateLinkSection {
    pub title: Option<String>,
    pub order_number: Option<i32>,
}

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    Queryable,
    Selectable,
    Identifiable,
    Associations,
    PartialEq,
)]
#[diesel(belongs_to(LinkSection))]
#[diesel(table_name = schema::link_item)]
#[diesel(check_for_backend(Sqlite))]
pub struct LinkItem {
    pub id: i32,
    pub title: String,
    pub link: String,
    pub order_number: i32,
    pub link_section_id: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[diesel(table_name = schema::link_item)]
#[diesel(check_for_backend(Sqlite))]
pub struct NewLinkItem {
    pub title: String,
    pub link: String,
    pub order_number: i32,
    pub link_section_id: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, AsChangeset, PartialEq)]
#[diesel(belongs_to(LinkSection))]
#[diesel(table_name = schema::link_item)]
#[diesel(check_for_backend(Sqlite))]
pub struct UpdateLinkItem {
    pub title: Option<String>,
    pub link: Option<String>,
    pub order_number: Option<i32>,
    pub link_section_id: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkSectionWithItems {
    #[serde(flatten)]
    pub section: LinkSection,
    pub items: Vec<LinkItem>,
}
