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
        node_type -> Text,
        node_order -> Integer,
        done -> Bool,
        skip -> Bool,
        details -> Nullable<Text>,
        parent_node -> Nullable<Text>,
        roadmap_uuid -> Text,
    }
}

diesel::joinable!(nodes -> maps (roadmap_uuid));

diesel::allow_tables_to_appear_in_same_query!(maps, nodes,);
