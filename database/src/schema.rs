// @generated automatically by Diesel CLI.

diesel::table! {
    link_item (id) {
        id -> Integer,
        title -> Text,
        link -> Text,
        order_number -> Integer,
        link_section_id -> Integer,
    }
}

diesel::table! {
    link_section (id) {
        id -> Integer,
        title -> Text,
        order_number -> Integer,
    }
}

diesel::joinable!(link_item -> link_section (link_section_id));

diesel::allow_tables_to_appear_in_same_query!(
    link_item,
    link_section,
);
