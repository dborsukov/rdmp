use crate::schema::{maps, nodes};
use diesel::prelude::*;
use serde::Deserialize;

#[derive(Deserialize, Insertable, Queryable, Identifiable, Debug)]
#[diesel(primary_key(uuid))]
#[diesel(table_name = maps)]
pub struct Roadmap {
    #[diesel(sql_type=Text)]
    pub uuid: String,
    #[diesel(sql_type=Text)]
    pub title: String,
    #[diesel(sql_type=Text)]
    pub description: String,
}

#[derive(Insertable, Associations, Identifiable, Queryable, Debug)]
#[diesel(belongs_to(Roadmap, foreign_key = roadmap_uuid))]
#[diesel(primary_key(uuid))]
#[diesel(table_name = nodes)]
pub struct Node {
    #[diesel(sql_type=Text)]
    pub uuid: String,
    #[diesel(sql_type=Text)]
    pub title: String,
    #[diesel(sql_type=Text)]
    pub description: String,
    #[diesel(sql_type=Text)]
    pub node_type: String,
    #[diesel(sql_type=Integer)]
    pub node_order: i32,
    #[diesel(sql_type=Boolean)]
    pub done: bool,
    #[diesel(sql_type=Boolean)]
    pub skip: bool,
    #[diesel(sql_type=Nullable<Text>)]
    pub details: Option<String>,
    #[diesel(sql_type=Nullable<Text>)]
    pub parent_node: Option<String>,
    #[diesel(sql_type=Text)]
    pub roadmap_uuid: String,
}
