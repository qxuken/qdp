use serde::{Deserialize, Serialize};

use diesel::prelude::*;
use diesel::sqlite::Sqlite;

use crate::{error::EntityError, result::Result, schema};

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

impl LinkSection {
    pub fn find_by_id(connection: &mut SqliteConnection, id: &i32) -> Result<Self> {
        let returned_data = schema::link_section::table
            .find(id)
            .first::<Self>(connection)?;

        Ok(returned_data)
    }

    fn max_possible_order_number(connection: &mut SqliteConnection) -> Result<i32> {
        use schema::link_section::dsl::*;

        let max_order_number = schema::link_section::table
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

    pub fn create(connection: &mut SqliteConnection, mut data: NewLinkSection) -> Result<Self> {
        use schema::link_section::dsl::*;
        connection.transaction(|connection: &mut SqliteConnection| {
            let max_possible_order_number = Self::max_possible_order_number(connection)?;

            let order_num = data.order_number.unwrap_or(max_possible_order_number);
            data.order_number = data
                .order_number
                .filter(|on| *on == order_num)
                .or(Some(order_num));

            Self::validate_order_number(&max_possible_order_number, &order_num, None)?;

            if order_num < max_possible_order_number {
                diesel::update(link_section)
                    .filter(order_number.ge(order_num))
                    .set(order_number.eq(order_number + 1))
                    .execute(connection)?;
            }

            let returned_data = diesel::insert_into(link_section)
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
        use schema::link_section::dsl::*;

        if self.order_number > *new_position {
            diesel::update(link_section)
                .filter(
                    order_number
                        .ge(*new_position)
                        .and(order_number.lt(self.order_number)),
                )
                .set(order_number.eq(order_number + 1))
                .execute(connection)?;
        } else {
            diesel::update(link_section)
                .filter(
                    order_number
                        .gt(self.order_number)
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
        data: &UpdateLinkSection,
    ) -> Result<Self> {
        connection.transaction(|connection: &mut SqliteConnection| {
            if let Some(new_position) = data.order_number.filter(|on| *on != self.order_number) {
                let max_possible_order_number = Self::max_possible_order_number(connection)?;
                Self::validate_order_number(
                    &max_possible_order_number,
                    &new_position,
                    Some(&self.order_number),
                )?;
                self.secure_order_position(connection, &new_position)?;
            }

            let returned_data = diesel::update(self)
                .set(data)
                .returning(Self::as_returning())
                .get_result(connection)?;

            Ok(returned_data)
        })
    }

    pub fn delete(self, connection: &mut SqliteConnection) -> Result<()> {
        use schema::link_section::dsl::*;

        connection.transaction(|connection: &mut SqliteConnection| {
            diesel::delete(
                schema::link_item::table
                    .filter(schema::link_item::dsl::link_section_id.eq(&self.id)),
            )
            .execute(connection)?;

            diesel::update(link_section)
                .filter(order_number.gt(self.order_number))
                .set(order_number.eq(order_number - 1))
                .execute(connection)?;
            diesel::delete(&self).execute(connection)?;

            Ok(())
        })
    }
}
