use diesel::prelude::*;

use crate::entities::links::{
    LinkItem, LinkSection, LinkSectionWithItems, NewLinkItem, NewLinkSection, UpdateLinkItem,
    UpdateLinkSection,
};
use crate::{error::DatabaseError, schema};

pub fn find_all_links(
    connection: &mut SqliteConnection,
) -> Result<Vec<LinkSectionWithItems>, DatabaseError> {
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

    let max_order_number = link_section
        .select(diesel::dsl::max(order_number))
        .first::<Option<i32>>(connection)?
        .unwrap_or(0);

    let returned_data = diesel::insert_into(link_section)
        .values((data, order_number.eq(max_order_number + 1)))
        .returning(LinkSection::as_returning())
        .get_result(connection)?;

    Ok(returned_data)
}

pub fn reorder_link_section(
    connection: &mut SqliteConnection,
    section_id: &i32,
    new_position: &i32,
) -> Result<(), DatabaseError> {
    use schema::link_section::dsl::*;

    let section: LinkSection = link_section.find(section_id).first(connection)?;

    let max_order_number = schema::link_section::table
        .select(diesel::dsl::max(order_number))
        .first::<Option<i32>>(connection)?
        .unwrap_or(0);

    let mut new_position = *new_position;

    if new_position < 1 {
        new_position = 1;
    }

    if new_position > max_order_number {
        new_position = max_order_number;
    }

    if section.order_number > new_position {
        diesel::update(link_section)
            .filter(
                order_number
                    .ge(new_position)
                    .and(order_number.lt(section.order_number)),
            )
            .set(order_number.eq(order_number + 1))
            .execute(connection)?;
    } else {
        diesel::update(link_section)
            .filter(
                order_number
                    .gt(section.order_number)
                    .and(order_number.le(new_position)),
            )
            .set(order_number.eq(order_number - 1))
            .execute(connection)?;
    }

    Ok(())
}

pub fn update_link_section(
    connection: &mut SqliteConnection,
    section_id: &i32,
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
    use schema::link_section::dsl::*;

    diesel::delete(link_item.filter(link_section_id.eq(&section_id))).execute(connection)?;
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

    let max_order_number = link_item
        .select(diesel::dsl::max(order_number))
        .first::<Option<i32>>(connection)?
        .unwrap_or(0);

    let returned_data = diesel::insert_into(link_item)
        .values((data, order_number.eq(max_order_number + 1)))
        .returning(LinkItem::as_returning())
        .get_result(connection)?;

    Ok(returned_data)
}

pub fn update_link_item(
    connection: &mut SqliteConnection,
    item_id: &i32,
    data: UpdateLinkItem,
) -> Result<LinkItem, DatabaseError> {
    use schema::link_item::dsl::*;

    let returned_data = diesel::update(link_item.find(&item_id))
        .set(&data)
        .returning(LinkItem::as_returning())
        .get_result(connection)?;

    if data.link_section_id.is_some() {
        let link_sections = schema::link_section::table
            .find(&returned_data.link_section_id)
            .first::<LinkSection>(connection)?;

        diesel::update(LinkItem::belonging_to(&link_sections))
            .filter(
                order_number
                    .ge(&returned_data.order_number)
                    .and(id.ne(&returned_data.id)),
            )
            .set(order_number.eq(order_number + 1))
            .execute(connection)?;
    }

    Ok(returned_data)
}

pub fn reorder_link_item(
    connection: &mut SqliteConnection,
    item_id: &i32,
    new_position: &i32,
) -> Result<(), DatabaseError> {
    use schema::link_item::dsl::*;

    let item: LinkItem = link_item.find(item_id).first(connection)?;

    let link_sections = schema::link_section::table
        .find(item.link_section_id)
        .first::<LinkSection>(connection)?;

    let max_order_number = LinkItem::belonging_to(&link_sections)
        .select(diesel::dsl::max(order_number))
        .first::<Option<i32>>(connection)?
        .unwrap_or(0);

    let mut new_position = *new_position;

    if new_position < 1 {
        new_position = 1;
    }

    if new_position > max_order_number {
        new_position = max_order_number;
    }

    if item.order_number > new_position {
        diesel::update(link_item)
            .filter(
                order_number
                    .ge(new_position)
                    .and(order_number.lt(item.order_number)),
            )
            .set(order_number.eq(order_number + 1))
            .execute(connection)?;
    } else {
        diesel::update(link_item)
            .filter(
                order_number
                    .gt(item.order_number)
                    .and(order_number.le(new_position)),
            )
            .set(order_number.eq(order_number - 1))
            .execute(connection)?;
    }

    diesel::update(&item)
        .set(order_number.eq(new_position))
        .execute(connection)?;

    Ok(())
}

pub fn delete_link_item(
    connection: &mut SqliteConnection,
    item_id: i32,
) -> Result<(), DatabaseError> {
    use schema::link_item::dsl::*;

    diesel::delete(link_item.find(&item_id)).execute(connection)?;

    Ok(())
}
