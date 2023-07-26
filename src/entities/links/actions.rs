use diesel::prelude::*;

use super::model::{
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
    mut data: NewLinkSection,
) -> Result<LinkSection, DatabaseError> {
    use schema::link_section::dsl::*;
    connection.transaction(|connection: &mut SqliteConnection| {
        let max_order_number = link_section
            .select(diesel::dsl::max(order_number))
            .first::<Option<i32>>(connection)?
            .unwrap_or(0);

        let max_possible_order_number = max_order_number + 1;

        data.order_number = data
            .order_number
            .filter(|pos| *pos > 1 || *pos <= max_possible_order_number)
            .or(Some(max_possible_order_number));

        if let Some(new_position) = data
            .order_number
            .filter(|pos| *pos < max_possible_order_number)
        {
            diesel::update(link_section)
                .filter(order_number.ge(new_position))
                .set(order_number.eq(order_number + 1))
                .execute(connection)?;
        }

        let returned_data = diesel::insert_into(link_section)
            .values(data)
            .returning(LinkSection::as_returning())
            .get_result(connection)?;

        Ok(returned_data)
    })
}

fn reorder_link_section(
    connection: &mut SqliteConnection,
    section_id: &i32,
    new_position: &mut i32,
) -> Result<(), DatabaseError> {
    use schema::link_section::dsl::*;

    let section: LinkSection = link_section.find(section_id).first(connection)?;

    if *new_position == section.order_number {
        return Ok(());
    }

    let max_order_number = schema::link_section::table
        .select(diesel::dsl::max(order_number))
        .first::<Option<i32>>(connection)?
        .unwrap_or(0);
    let max_possible_order_number = max_order_number + 1;

    if *new_position < 1 || *new_position > max_possible_order_number {
        *new_position = max_possible_order_number;
    }

    if section.order_number > *new_position {
        diesel::update(link_section)
            .filter(
                order_number
                    .ge(*new_position)
                    .and(order_number.lt(section.order_number)),
            )
            .set(order_number.eq(order_number + 1))
            .execute(connection)?;
    } else {
        diesel::update(link_section)
            .filter(
                order_number
                    .gt(section.order_number)
                    .and(order_number.le(*new_position)),
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
    connection.transaction(|connection: &mut SqliteConnection| {
        if let Some(mut new_position) = data.order_number {
            reorder_link_section(connection, section_id, &mut new_position)?;
        }

        let returned_data = diesel::update(link_section.find(section_id))
            .set(&data)
            .returning(LinkSection::as_returning())
            .get_result(connection)?;

        Ok(returned_data)
    })
}

pub fn delete_link_section(
    connection: &mut SqliteConnection,
    section_id: i32,
) -> Result<(), DatabaseError> {
    use schema::link_section::dsl::*;
    connection.transaction(|connection: &mut SqliteConnection| {
        diesel::delete(
            schema::link_item::table
                .filter(schema::link_item::dsl::link_section_id.eq(&section_id)),
        )
        .execute(connection)?;

        let section = link_section
            .find(section_id)
            .first::<LinkSection>(connection)?;

        diesel::update(link_section)
            .filter(order_number.gt(section.order_number))
            .set(order_number.eq(order_number - 1))
            .execute(connection)?;
        diesel::delete(&section).execute(connection)?;

        Ok(())
    })
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
    mut data: NewLinkItem,
) -> Result<LinkItem, DatabaseError> {
    use schema::link_item::dsl::*;
    connection.transaction(|connection: &mut SqliteConnection| {
        let max_order_number = link_item
            .filter(link_section_id.eq(data.link_section_id))
            .select(diesel::dsl::max(order_number))
            .first::<Option<i32>>(connection)?
            .unwrap_or(0);

        let max_possible_order_number = max_order_number + 1;

        data.order_number = data
            .order_number
            .filter(|pos| *pos > 1 || *pos <= max_possible_order_number)
            .or(Some(max_possible_order_number));

        if let Some(new_position) = data
            .order_number
            .filter(|pos| *pos < max_possible_order_number)
        {
            diesel::update(link_item)
                .filter(
                    link_section_id
                        .eq(data.link_section_id)
                        .and(order_number.ge(new_position)),
                )
                .set(order_number.eq(order_number + 1))
                .execute(connection)?;
        }

        let returned_data = diesel::insert_into(link_item)
            .values(data)
            .returning(LinkItem::as_returning())
            .get_result(connection)?;

        Ok(returned_data)
    })
}

fn reorder_link_item(
    connection: &mut SqliteConnection,
    item: &LinkItem,
    new_position: &mut i32,
) -> Result<(), DatabaseError> {
    use schema::link_item::dsl::*;

    let max_order_number = link_item
        .filter(link_section_id.eq(item.link_section_id))
        .select(diesel::dsl::max(order_number))
        .first::<Option<i32>>(connection)?
        .unwrap_or(0);
    let max_possible_order_number = max_order_number + 1;

    if *new_position < 1 || *new_position > max_possible_order_number {
        *new_position = max_possible_order_number;
    }

    if item.order_number > *new_position {
        diesel::update(link_item)
            .filter(
                link_section_id
                    .eq(item.link_section_id)
                    .and(order_number.ge(*new_position))
                    .and(order_number.lt(item.order_number)),
            )
            .set(order_number.eq(order_number + 1))
            .execute(connection)?;
    } else {
        diesel::update(link_item)
            .filter(
                link_section_id
                    .eq(item.link_section_id)
                    .and(order_number.gt(item.order_number))
                    .and(order_number.le(*new_position)),
            )
            .set(order_number.eq(order_number - 1))
            .execute(connection)?;
    }

    Ok(())
}

pub fn update_link_item(
    connection: &mut SqliteConnection,
    item_id: &i32,
    mut data: UpdateLinkItem,
) -> Result<LinkItem, DatabaseError> {
    use schema::link_item::dsl::*;
    connection.transaction(|connection: &mut SqliteConnection| {
        let item = link_item.find(item_id).first::<LinkItem>(connection)?;
        let max_order_number = link_item
            .filter(link_section_id.eq(data.link_section_id.unwrap_or(item.link_section_id)))
            .select(diesel::dsl::max(order_number))
            .first::<Option<i32>>(connection)?
            .unwrap_or(0);
        let max_possible_order_number = max_order_number + 1;
        data.order_number = data
            .order_number
            .filter(|pos| *pos > 1 || *pos <= max_possible_order_number)
            .or(Some(item.order_number))
            .filter(|pos| *pos <= max_possible_order_number)
            .or(Some(max_possible_order_number));

        if let Some(new_section_id) = data
            .link_section_id
            .filter(|lsid| *lsid != item.link_section_id)
        {
            diesel::update(link_item)
                .filter(
                    link_section_id
                        .eq(new_section_id)
                        .and(order_number.ge(data.order_number.unwrap()))
                        .and(id.ne(&item.id)),
                )
                .set(order_number.eq(order_number + 1))
                .execute(connection)?;

            diesel::update(link_item)
                .filter(
                    link_section_id
                        .eq(item.link_section_id)
                        .and(order_number.gt(item.order_number))
                        .and(id.ne(&item.id)),
                )
                .set(order_number.eq(order_number - 1))
                .execute(connection)?;
        } else if let Some(mut new_position) =
            data.order_number.filter(|pos| *pos != item.order_number)
        {
            reorder_link_item(connection, &item, &mut new_position)?;
        }

        let returned_data = diesel::update(link_item.find(&item_id))
            .set(&data)
            .returning(LinkItem::as_returning())
            .get_result(connection)?;

        Ok(returned_data)
    })
}

pub fn delete_link_item(
    connection: &mut SqliteConnection,
    item_id: i32,
) -> Result<(), DatabaseError> {
    use schema::link_item::dsl::*;
    connection.transaction(|connection: &mut SqliteConnection| {
        let item = link_item.find(item_id).first::<LinkItem>(connection)?;
        diesel::update(link_item)
            .filter(
                link_section_id
                    .eq(item.link_section_id)
                    .and(order_number.gt(item.order_number)),
            )
            .set(order_number.eq(order_number - 1))
            .execute(connection)?;
        diesel::delete(&item).execute(connection)?;

        Ok(())
    })
}
