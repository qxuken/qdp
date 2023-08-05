use serde::{Deserialize, Serialize};

use diesel::prelude::*;
use diesel::sqlite::Sqlite;

use crate::{error::DatabaseError, schema};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Identifiable, Selectable)]
#[serde(rename_all = "camelCase")]
#[diesel(table_name = schema::link_section)]
#[diesel(check_for_backend(Sqlite))]
pub struct LinkSection {
    pub id: i32,
    pub title: String,
    pub order_number: i32,
}

impl LinkSection {
    pub fn find_by_id(
        connection: &mut SqliteConnection,
        id: &i32,
    ) -> Result<LinkSection, DatabaseError> {
        let returned_data = schema::link_section::table
            .find(id)
            .first::<Self>(connection)?;

        Ok(returned_data)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[serde(rename_all = "camelCase")]
#[diesel(table_name = schema::link_section)]
#[diesel(check_for_backend(Sqlite))]
pub struct NewLinkSection {
    pub title: String,
    pub order_number: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, AsChangeset, PartialEq)]
#[serde(rename_all = "camelCase")]
#[diesel(table_name = schema::link_section)]
#[diesel(check_for_backend(Sqlite))]
pub struct UpdateLinkSection {
    pub title: Option<String>,
    pub order_number: Option<i32>,
}
