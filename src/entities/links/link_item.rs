use serde::{Deserialize, Serialize};

use diesel::prelude::*;
use diesel::sqlite::Sqlite;

use super::link_section::LinkSection;
use crate::{error::EntityError, result::Result, schema};

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
    pub order_number: Option<i32>,
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
    pub order_number: Option<i32>,
    pub link_section_id: Option<i32>,
}

impl LinkItem {
    pub fn find_by_id(connection: &mut SqliteConnection, item_id: &i32) -> Result<Self> {
        let returned_data = schema::link_item::table
            .find(item_id)
            .first::<Self>(connection)?;

        Ok(returned_data)
    }

    fn max_possible_order_number(
        connection: &mut SqliteConnection,
        section_id: &i32,
    ) -> Result<i32> {
        use schema::link_item::dsl::*;

        let max_order_number = link_item
            .filter(link_section_id.eq(section_id))
            .select(diesel::dsl::max(order_number))
            .first::<Option<i32>>(connection)?
            .unwrap_or(0);

        Ok(max_order_number + 1)
    }

    fn validate_order_number(
        max_possible_order_number: &i32,
        order_number: &i32,
        current_position: Option<&i32>,
    ) -> Result<()> {
        let mut max_possible_order_number = *max_possible_order_number;
        if current_position
            .filter(|pos| **pos == max_possible_order_number - 1)
            .is_some()
        {
            max_possible_order_number -= 1;
        }
        if *order_number < 1 || *order_number > max_possible_order_number {
            return EntityError::BadData("order_number is out of bounds".to_owned()).into();
        }

        Ok(())
    }

    pub fn create(connection: &mut SqliteConnection, mut data: NewLinkItem) -> Result<Self> {
        use schema::link_item::dsl::*;
        connection.transaction(|connection: &mut SqliteConnection| {
            let max_possible_order_number =
                Self::max_possible_order_number(connection, &data.link_section_id)?;

            let order_num = data.order_number.unwrap_or(max_possible_order_number);
            data.order_number = data
                .order_number
                .filter(|on| *on == order_num)
                .or(Some(order_num));

            Self::validate_order_number(&max_possible_order_number, &order_num, None)?;

            if order_num < max_possible_order_number {
                diesel::update(link_item)
                    .filter(
                        link_section_id
                            .eq(data.link_section_id)
                            .and(order_number.ge(order_num)),
                    )
                    .set(order_number.eq(order_number + 1))
                    .execute(connection)?;
            }

            let returned_data = diesel::insert_into(link_item)
                .values(data)
                .returning(Self::as_returning())
                .get_result(connection)?;

            Ok(returned_data)
        })
    }

    fn secure_order_position(
        &self,
        connection: &mut SqliteConnection,
        new_position: &i32,
    ) -> Result<()> {
        use schema::link_item::dsl::*;

        if self.order_number > *new_position {
            diesel::update(link_item)
                .filter(
                    link_section_id
                        .eq(self.link_section_id)
                        .and(order_number.ge(*new_position))
                        .and(order_number.lt(self.order_number)),
                )
                .set(order_number.eq(order_number + 1))
                .execute(connection)?;
        } else {
            diesel::update(link_item)
                .filter(
                    link_section_id
                        .eq(self.link_section_id)
                        .and(order_number.gt(self.order_number))
                        .and(order_number.le(*new_position)),
                )
                .set(order_number.eq(order_number - 1))
                .execute(connection)?;
        }

        Ok(())
    }

    pub fn update(
        &self,
        connection: &mut SqliteConnection,
        mut data: UpdateLinkItem,
    ) -> Result<Self> {
        use schema::link_item::dsl::*;
        connection.transaction(|connection: &mut SqliteConnection| {
            let section_id = data.link_section_id.unwrap_or(self.link_section_id);
            let max_possible_order_number =
                Self::max_possible_order_number(connection, &section_id)?;

            let order_num = data.order_number.unwrap_or({
                if self.order_number > max_possible_order_number {
                    max_possible_order_number
                } else {
                    self.order_number
                }
            });

            if section_id != self.link_section_id {
                if data.order_number.is_some() {
                    Self::validate_order_number(&max_possible_order_number, &order_num, None)?;
                }
                diesel::update(link_item)
                    .filter(
                        link_section_id
                            .eq(section_id)
                            .and(order_number.ge(data.order_number.unwrap())),
                    )
                    .set(order_number.eq(order_number + 1))
                    .execute(connection)?;

                diesel::update(link_item)
                    .filter(
                        link_section_id
                            .eq(self.link_section_id)
                            .and(order_number.gt(self.order_number)),
                    )
                    .set(order_number.eq(order_number - 1))
                    .execute(connection)?;
            } else if order_num != self.order_number {
                Self::validate_order_number(
                    &max_possible_order_number,
                    &order_num,
                    Some(&self.order_number),
                )?;
                self.secure_order_position(connection, &order_num)?;
            }

            data.order_number = data
                .order_number
                .filter(|on| *on == order_num)
                .or(Some(order_num));
            let returned_data = diesel::update(&self)
                .set(data)
                .returning(Self::as_returning())
                .get_result(connection)?;

            Ok(returned_data)
        })
    }

    pub fn delete(self, connection: &mut SqliteConnection) -> Result<()> {
        use schema::link_item::dsl::*;
        connection.transaction(|connection: &mut SqliteConnection| {
            diesel::update(link_item)
                .filter(
                    link_section_id
                        .eq(self.link_section_id)
                        .and(order_number.gt(self.order_number)),
                )
                .set(order_number.eq(order_number - 1))
                .execute(connection)?;
            diesel::delete(&self).execute(connection)?;

            Ok(())
        })
    }
}
