use diesel::prelude::*;

use crate::entities::links::{
    LinkItem, LinkSection, LinkSectionWithItems, NewLinkItem, NewLinkSection, UpdateLinkItem,
    UpdateLinkSection,
};
use crate::{error::DatabaseError, schema};

pub fn find_all_links(
    connection: &mut SqliteConnection,
) -> Result<Vec<LinkSectionWithItems>, DatabaseError> {
    use crate::schema::link_item::order_number;

    let link_sections = schema::link_section::table
        .select(LinkSection::as_select())
        .load(connection)?;

    let link_items = LinkItem::belonging_to(&link_sections)
        .order(order_number.asc())
        .select(LinkItem::as_select())
        .load(connection)?;

    let returned_data = link_items
        .grouped_by(&link_sections)
        .into_iter()
        .zip(link_sections)
        .map(|(items, section)| LinkSectionWithItems { section, items })
        .collect::<Vec<LinkSectionWithItems>>();

    Ok(returned_data)
}

pub fn find_link_section(
    connection: &mut SqliteConnection,
    section_id: &i32,
) -> Result<LinkSection, DatabaseError> {
    use schema::link_section::dsl::*;

    let returned_data = link_section
        .find(section_id)
        .first::<LinkSection>(connection)?;

    Ok(returned_data)
}

pub fn insert_link_section(
    connection: &mut SqliteConnection,
    data: NewLinkSection,
) -> Result<LinkSection, DatabaseError> {
    use schema::link_section::dsl::*;

    let returned_data = diesel::insert_into(link_section)
        .values(data)
        .returning(LinkSection::as_returning())
        .get_result(connection)?;

    Ok(returned_data)
}

pub fn update_link_section(
    connection: &mut SqliteConnection,
    section_id: i32,
    data: UpdateLinkSection,
) -> Result<LinkSection, DatabaseError> {
    use schema::link_section::dsl::*;

    let returned_data = diesel::update(link_section.find(section_id))
        .set(data)
        .returning(LinkSection::as_returning())
        .get_result(connection)?;

    Ok(returned_data)
}

pub fn delete_link_section(
    connection: &mut SqliteConnection,
    section_id: i32,
) -> Result<(), DatabaseError> {
    use schema::link_item::dsl::*;

    diesel::delete(link_item.filter(link_section_id.eq(&section_id))).execute(connection)?;

    use schema::link_section::dsl::*;

    diesel::delete(link_section.find(&section_id)).execute(connection)?;

    Ok(())
}

pub fn find_link_item(
    connection: &mut SqliteConnection,
    item_id: &i32,
) -> Result<LinkItem, DatabaseError> {
    use schema::link_item::dsl::*;

    let returned_data = link_item.find(item_id).first::<LinkItem>(connection)?;

    Ok(returned_data)
}

pub fn insert_link_item(
    connection: &mut SqliteConnection,
    data: NewLinkItem,
) -> Result<LinkItem, DatabaseError> {
    use schema::link_item::dsl::*;

    let returned_data = diesel::insert_into(link_item)
        .values(data)
        .returning(LinkItem::as_returning())
        .get_result(connection)?;

    Ok(returned_data)
}

pub fn update_link_item(
    connection: &mut SqliteConnection,
    item_id: i32,
    data: UpdateLinkItem,
) -> Result<LinkItem, DatabaseError> {
    use schema::link_item::dsl::*;

    let returned_data = diesel::update(link_item.find(&item_id))
        .set(data)
        .returning(LinkItem::as_returning())
        .get_result(connection)?;

    Ok(returned_data)
}

pub fn delete_link_item(
    connection: &mut SqliteConnection,
    item_id: i32,
) -> Result<(), DatabaseError> {
    use schema::link_item::dsl::*;

    diesel::delete(link_item.find(&item_id)).execute(connection)?;

    Ok(())
}
