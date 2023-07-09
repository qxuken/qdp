use serde::{Deserialize, Serialize};

use diesel::prelude::*;
use diesel::sqlite::Sqlite;

use crate::schema;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable, Selectable)]
#[serde(rename_all = "camelCase")]
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
}

#[derive(Debug, Clone, Serialize, Deserialize, AsChangeset, PartialEq)]
#[serde(rename_all = "camelCase")]
#[diesel(table_name = schema::link_section)]
#[diesel(check_for_backend(Sqlite))]
pub struct UpdateLinkSection {
    pub title: Option<String>,
    pub order_number: i32,
}

#[derive(
    Debug, Clone, Serialize, Deserialize, Queryable, Selectable, Identifiable, Associations,
)]
#[serde(rename_all = "camelCase")]
#[diesel(belongs_to(LinkSection))]
#[diesel(table_name = schema::link_item)]
#[diesel(check_for_backend(Sqlite))]
pub struct LinkItem {
    pub id: i32,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub link: String,
    pub order_number: i32,
    #[serde(skip_serializing)]
    pub link_section_id: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[serde(rename_all = "camelCase")]
#[diesel(table_name = schema::link_item)]
#[diesel(check_for_backend(Sqlite))]
pub struct NewLinkItem {
    pub title: String,
    pub description: Option<String>,
    pub link: String,
    pub link_section_id: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, AsChangeset, PartialEq)]
#[serde(rename_all = "camelCase")]
#[diesel(belongs_to(LinkSection))]
#[diesel(table_name = schema::link_item)]
#[diesel(check_for_backend(Sqlite))]
pub struct UpdateLinkItem {
    pub title: Option<String>,
    pub description: Option<String>,
    pub link: Option<String>,
    pub order_number: i32,
    pub link_section_id: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkSectionWithItems {
    #[serde(flatten)]
    pub section: LinkSection,
    pub items: Vec<LinkItem>,
}
