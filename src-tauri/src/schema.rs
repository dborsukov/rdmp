// @generated automatically by Diesel CLI.

diesel::table! {
    maps (uuid) {
        uuid -> Text,
        title -> Text,
        description -> Text,
    }
}

diesel::table! {
    nodes (uuid) {
        uuid -> Text,
        title -> Text,
        description -> Text,
        is_main_node -> Bool,
        node_order -> Integer,
        parent_node -> Nullable<Text>,
        roadmap_uuid -> Text,
    }
}

diesel::joinable!(nodes -> maps (roadmap_uuid));

diesel::allow_tables_to_appear_in_same_query!(
    maps,
    nodes,
);
