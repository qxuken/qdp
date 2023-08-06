use serde::{Deserialize, Serialize};

use super::{link_item::LinkItem, link_section::LinkSection};
use crate::{result::Result, schema};
use diesel::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinksView {
    #[serde(flatten)]
    pub section: LinkSection,
    pub items: Vec<LinkItem>,
}

impl LinksView {
    pub fn query(connection: &mut SqliteConnection) -> Result<Vec<Self>> {
        let link_sections = schema::link_section::table
            .order(schema::link_section::order_number.asc())
            .select(LinkSection::as_select())
            .load(connection)?;

        let link_items = LinkItem::belonging_to(&link_sections)
            .order(schema::link_item::order_number.asc())
            .select(LinkItem::as_select())
            .load(connection)?;

        let returned_data = link_items
            .grouped_by(&link_sections)
            .into_iter()
            .zip(link_sections)
            .map(|(items, section)| Self { section, items })
            .collect::<Vec<Self>>();

        Ok(returned_data)
    }
}
