use diesel::prelude::*;
use database::schema::*;
use database::models::*;

fn main() -> QueryResult<()> {
    let mut connection = database::establish_connection();

    let items = link_item::table
        .inner_join(link_section::table)
        .select((LinkItem::as_select(), LinkSection::as_select()))
        .load::<(LinkItem, LinkSection)>(&mut connection)?;

    println!("Items: \n {items:?}\n");

    Ok(())
}