use log::error;
use serde::{Deserialize, Serialize};
use tauri::command;

use crate::db::establish_connection;
use crate::models;
use diesel::prelude::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct Roadmap {
    uuid: String,
    title: String,
    description: String,
    nodes: Vec<Node>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Node {
    uuid: String,
    title: String,
    description: String,
    #[serde(rename(serialize = "isMainNode", deserialize = "isMainNode"))]
    is_main_node: bool,
    children: Vec<Node>,
}

#[command]
pub fn load_all_roadmaps() -> Result<Vec<Roadmap>, String> {
    use crate::schema::maps::dsl::*;
    let conn = &mut establish_connection();
    let mut allmaps = Vec::new();

    let results = match maps.load::<models::Roadmap>(conn) {
        Ok(list) => list,
        Err(err) => {
            error!("Failed to load roadmaps: {err}");
            return Err(err.to_string());
        }
    };

    for map in results {
        allmaps.push(Roadmap {
            uuid: map.uuid.clone(),
            title: map.title.clone(),
            description: map.description.clone(),
            nodes: get_main_nodes(conn, &map)?,
        })
    }

    Ok(allmaps)
}

fn get_main_nodes(conn: &mut SqliteConnection, map: &models::Roadmap) -> Result<Vec<Node>, String> {
    use crate::schema::nodes::dsl::*;
    let mut main_nodes = Vec::new();

    let results = match models::Node::belonging_to(map)
        .filter(is_main_node.eq(true))
        .order(node_order.asc())
        .load::<models::Node>(conn)
    {
        Ok(list) => list,
        Err(err) => {
            error!("Failed to load main nodes: {err}");
            return Err(err.to_string());
        }
    };

    for node in results {
        main_nodes.push(Node {
            uuid: node.uuid.clone(),
            title: node.title.clone(),
            description: node.description.clone(),
            is_main_node: node.is_main_node.clone(),
            children: get_child_nodes(conn, &node)?,
        })
    }

    Ok(main_nodes)
}

fn get_child_nodes(conn: &mut SqliteConnection, node: &models::Node) -> Result<Vec<Node>, String> {
    use crate::schema::nodes::dsl::*;
    let mut child_nodes = Vec::new();

    let results = match nodes
        .filter(parent_node.eq(&node.uuid))
        .order(node_order.asc())
        .load::<models::Node>(conn)
    {
        Ok(list) => list,
        Err(err) => {
            error!("Failed to load child nodes: {err}");
            return Err(err.to_string());
        }
    };

    for node in results {
        child_nodes.push(Node {
            uuid: node.uuid.clone(),
            title: node.title.clone(),
            description: node.description.clone(),
            is_main_node: node.is_main_node.clone(),
            children: get_child_nodes(conn, &node)?,
        })
    }

    Ok(child_nodes)
}

#[command]
pub fn add_roadmap(roadmap: models::Roadmap) -> Result<(), String> {
    use crate::schema::maps::dsl::*;
    let conn = &mut establish_connection();
    diesel::insert_into(maps)
        .values(roadmap)
        .execute(conn)
        .expect("failed to insert roadmap");
    Ok(())
}

#[command]
pub fn update_roadmap(roadmap: models::Roadmap) -> Result<(), String> {
    use crate::schema::maps::dsl::*;
    let conn = &mut establish_connection();
    diesel::update(maps.find(roadmap.uuid))
        .set((title.eq(roadmap.title), description.eq(roadmap.description)))
        .execute(conn)
        .expect("failed to insert roadmap");
    Ok(())
}

#[command]
pub fn remove_roadmap(query_uuid: &str) -> Result<(), String> {
    use crate::schema::maps::dsl::*;
    let conn = &mut establish_connection();
    diesel::delete(maps.filter(uuid.eq(query_uuid)))
        .execute(conn)
        .expect("failed to delete roadmap");
    Ok(())
}
