use log::error;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::path::PathBuf;
use tauri::command;

use crate::db::establish_connection;
use crate::fs;
use crate::models;
use diesel::dsl::{count, not};
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
    #[serde(rename(serialize = "nodeType", deserialize = "nodeType"))]
    node_type: String,
    #[serde(rename(serialize = "nodeOrder", deserialize = "nodeOrder"))]
    node_order: i32,
    done: bool,
    skip: bool,
    details: Option<String>,
    children: Vec<Node>,
}

#[command]
pub fn load_roadmap(query_uuid: &str) -> Result<Roadmap, String> {
    use crate::schema::maps::dsl::maps;
    let conn = &mut establish_connection();

    let map_model = match maps.find(&query_uuid).first::<models::Roadmap>(conn) {
        Ok(map) => map,
        Err(err) => {
            error!("Failed to load roadmap {query_uuid}: {err}");
            return Err(err.to_string());
        }
    };

    let map = Roadmap {
        uuid: map_model.uuid.clone(),
        title: map_model.title.clone(),
        description: map_model.description.clone(),
        nodes: get_main_nodes(conn, &map_model)?,
    };

    Ok(map)
}

#[command]
pub fn load_all_roadmaps() -> Result<Vec<Roadmap>, String> {
    use crate::schema::maps::dsl::maps;
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
        });
    }

    Ok(allmaps)
}

fn get_main_nodes(conn: &mut SqliteConnection, map: &models::Roadmap) -> Result<Vec<Node>, String> {
    use crate::schema::nodes::dsl::{node_order, node_type};
    let mut main_nodes = Vec::new();

    let results = match models::Node::belonging_to(map)
        .filter(node_type.eq("main").or(node_type.eq("root")))
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
            node_type: node.node_type.clone(),
            node_order: node.node_order,
            done: node.done,
            skip: node.skip,
            details: node.details.clone(),
            children: get_child_nodes(conn, &node)?,
        });
    }

    Ok(main_nodes)
}

fn get_child_nodes(conn: &mut SqliteConnection, node: &models::Node) -> Result<Vec<Node>, String> {
    use crate::schema::nodes::dsl::{node_order, nodes, parent_node};
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
            node_type: node.node_type.clone(),
            node_order: node.node_order,
            done: node.done,
            skip: node.skip,
            details: node.details.clone(),
            children: get_child_nodes(conn, &node)?,
        });
    }

    Ok(child_nodes)
}

#[command]
pub fn add_roadmap(roadmap: models::Roadmap) -> Result<(), String> {
    use crate::schema::maps::dsl::maps;
    let conn = &mut establish_connection();
    if let Err(err) = diesel::insert_into(maps).values(roadmap).execute(conn) {
        return Err(format!("Failed to add roadmap: {err}"));
    }
    Ok(())
}

#[command]
pub fn update_roadmap(roadmap: models::Roadmap) -> Result<(), String> {
    use crate::schema::maps::dsl::{description, maps, title};
    let conn = &mut establish_connection();
    if let Err(err) = diesel::update(maps.find(roadmap.uuid))
        .set((title.eq(roadmap.title), description.eq(roadmap.description)))
        .execute(conn)
    {
        return Err(format!("Failed to update roadmap: {err}"));
    }
    Ok(())
}

#[command]
pub fn remove_roadmap(query_uuid: &str) -> Result<(), String> {
    use crate::schema::maps::dsl::{maps, uuid};
    let conn = &mut establish_connection();
    if let Err(err) = diesel::delete(maps.filter(uuid.eq(query_uuid))).execute(conn) {
        return Err(format!("Failed to delete roadmap: {err}"));
    }
    Ok(())
}

#[command]
pub fn add_node(
    node: Node,
    parent_node_uuid: Option<String>,
    query_roadmap_uuid: String,
) -> Result<(), String> {
    use crate::schema::nodes::dsl::nodes;
    let conn = &mut establish_connection();
    let node_model = models::Node {
        uuid: node.uuid,
        title: node.title,
        description: node.description,
        node_type: node.node_type,
        node_order: node.node_order,
        done: false,
        skip: false,
        details: None,
        parent_node: parent_node_uuid,
        roadmap_uuid: query_roadmap_uuid,
    };
    if let Err(err) = diesel::insert_into(nodes).values(node_model).execute(conn) {
        return Err(format!("Failed to add node: {err}"));
    }
    Ok(())
}

#[command]
pub fn update_node(node: Node) -> Result<(), String> {
    use crate::schema::nodes::dsl::{description, nodes, title};
    let conn = &mut establish_connection();
    if let Err(err) = diesel::update(nodes.find(node.uuid))
        .set((title.eq(node.title), description.eq(node.description)))
        .execute(conn)
    {
        return Err(format!("Failed to update node: {err}"));
    }
    Ok(())
}

#[command]
pub fn remove_node(query_uuid: &str) -> Result<(), String> {
    use crate::schema::nodes::dsl::{nodes, uuid};
    let conn = &mut establish_connection();
    if let Err(err) = diesel::delete(nodes.filter(uuid.eq(query_uuid))).execute(conn) {
        return Err(format!("Failed to delete node: {err}"));
    }
    Ok(())
}

#[command]
pub fn set_done(query_uuid: &str, query_done: bool) -> Result<(), String> {
    use crate::schema::nodes::dsl::{done, nodes, skip};
    let conn = &mut establish_connection();
    if let Err(err) = diesel::update(nodes.find(query_uuid))
        .set((done.eq(query_done), skip.eq(false)))
        .execute(conn)
    {
        return Err(format!("Failed to update node: {err}"));
    }
    Ok(())
}

#[command]
pub fn set_skip(query_uuid: &str, query_skip: bool) -> Result<(), String> {
    use crate::schema::nodes::dsl::{done, nodes, skip};
    let conn = &mut establish_connection();
    if let Err(err) = diesel::update(nodes.find(query_uuid))
        .set((skip.eq(query_skip), done.eq(false)))
        .execute(conn)
    {
        return Err(format!("Failed to update node: {err}"));
    }
    Ok(())
}

#[command]
pub fn load_details(query_uuid: &str) -> Result<Option<String>, String> {
    use crate::schema::nodes::dsl::{details, nodes, uuid};
    let conn = &mut establish_connection();
    match nodes
        .select(details)
        .filter(uuid.eq(query_uuid))
        .first::<Option<String>>(conn)
    {
        Ok(string) => Ok(string),
        Err(err) => {
            error!("Failed to load roadmap {query_uuid}: {err}");
            Err(err.to_string())
        }
    }
}

#[command]
pub fn save_details(query_uuid: &str, query_string: Option<&str>) -> Result<(), String> {
    use crate::schema::nodes::dsl::{details, nodes};
    let conn = &mut establish_connection();
    if let Err(err) = diesel::update(nodes.find(query_uuid))
        .set(details.eq(query_string))
        .execute(conn)
    {
        return Err(format!("Failed to update node: {err}"));
    }
    Ok(())
}

#[command]
pub fn load_roadmaps_amount() -> Result<i64, String> {
    use crate::schema::maps::dsl::maps;
    let conn = &mut establish_connection();
    match maps.count().get_result(conn) {
        Ok(amount) => Ok(amount),
        Err(err) => Err(format!("Failed to count roadmaps: {err}")),
    }
}

#[command]
pub fn load_nodes_amount() -> Result<i64, String> {
    use crate::schema::nodes::dsl::{node_type, nodes};
    let conn = &mut establish_connection();
    match nodes
        .count()
        .filter(not(node_type.eq("root")))
        .get_result(conn)
    {
        Ok(amount) => Ok(amount),
        Err(err) => Err(format!("Failed to count nodes: {err}")),
    }
}

#[command]
pub fn read_settings() -> Result<Map<String, Value>, String> {
    let settings_string =
        match std::fs::read_to_string(fs::get_app_base_dir().join("settings.json")) {
            Ok(path) => path,
            Err(err) => return Err(format!("Failed to read settings file: {err}")),
        };
    match serde_json::from_str(&settings_string) {
        Ok(map) => Ok(map),
        Err(err) => Err(format!("Failed to read settings from file: {err}")),
    }
}

#[command]
pub fn write_settings(settings: Map<String, Value>) -> Result<(), String> {
    if let Err(err) = std::fs::write(
        fs::get_app_base_dir().join("settings.json"),
        serde_json::to_string_pretty(&settings).unwrap(),
    ) {
        return Err(format!("Failed to write settings: {err}"));
    };
    Ok(())
}

#[command]
pub fn export_roadmap(query_uuid: &str, title: &str, folder: PathBuf) -> Result<(), String> {
    let file_path = match folder.join(format!("{title}.rdmp")).to_str() {
        Some(str) => str.to_owned(),
        None => return Err("Generic error".to_string()),
    };
    if let Err(err) = std::fs::copy(fs::get_app_base_dir().join("rdmp.db"), &file_path) {
        return Err(err.to_string());
    }
    let conn = &mut establish_connection();
    let mut export_conn = match SqliteConnection::establish(&file_path) {
        Ok(conn) => conn,
        Err(err) => return Err(err.to_string()),
    };
    // delete data
    use crate::schema::maps::dsl::maps;
    use crate::schema::nodes::dsl::{nodes, roadmap_uuid};
    if let Err(err) = diesel::delete(maps).execute(&mut export_conn) {
        return Err(err.to_string());
    }
    if let Err(err) = diesel::delete(nodes).execute(&mut export_conn) {
        return Err(err.to_string());
    }
    // copy roadmap
    let map = match maps.find(&query_uuid).first::<models::Roadmap>(conn) {
        Ok(map) => map,
        Err(err) => return Err(err.to_string()),
    };
    if let Err(err) = diesel::insert_into(maps)
        .values(map)
        .execute(&mut export_conn)
    {
        return Err(err.to_string());
    };
    // copy nodes
    let nodes_list = match nodes
        .filter(roadmap_uuid.eq(&query_uuid))
        .load::<models::Node>(conn)
    {
        Ok(list) => list,
        Err(err) => return Err(err.to_string()),
    };
    if let Err(err) = diesel::insert_into(nodes)
        .values(nodes_list)
        .execute(&mut export_conn)
    {
        return Err(err.to_string());
    };
    Ok(())
}

#[command]
pub fn import_roadmap(path: &str) -> Result<Roadmap, String> {
    let conn = &mut establish_connection();
    let mut import_conn = match SqliteConnection::establish(path) {
        Ok(conn) => conn,
        Err(err) => return Err(err.to_string()),
    };
    // setup
    use crate::schema::maps::dsl::{maps, uuid};
    use crate::schema::nodes::dsl::nodes;
    let map = match maps.first::<models::Roadmap>(&mut import_conn) {
        Ok(map) => map,
        Err(err) => return Err(err.to_string()),
    };
    // duplicate check
    let count: i64 = match maps
        .filter(uuid.eq(&map.uuid))
        .select(count(uuid))
        .first(conn)
    {
        Ok(count) => count,
        Err(err) => return Err(err.to_string()),
    };
    if count > 0 {
        return Err("Roadmap you are trying to import already exists".to_string());
    };
    // copy roadmap
    if let Err(err) = diesel::insert_into(maps).values(&map).execute(conn) {
        return Err(err.to_string());
    };
    // copy nodes
    let nodes_list = match nodes.load::<models::Node>(&mut import_conn) {
        Ok(list) => list,
        Err(err) => return Err(err.to_string()),
    };
    if let Err(err) = diesel::insert_into(nodes).values(nodes_list).execute(conn) {
        return Err(err.to_string());
    };
    load_roadmap(&map.uuid)
}

#[command]
pub fn expand_nodes_around(query_uuid: &str, around: i32) -> Result<(), String> {
    use crate::schema::nodes::dsl::{node_order, nodes, roadmap_uuid};
    let conn = &mut establish_connection();
    if let Err(err) =
        diesel::update(nodes.filter((roadmap_uuid.eq(query_uuid)).and(node_order.ge(around))))
            .set(node_order.eq(node_order + 1))
            .execute(conn)
    {
        return Err(format!("Failed to expand nodes: {err}"));
    }
    Ok(())
}

#[command]
pub fn squash_nodes_around(query_uuid: &str, around: i32) -> Result<(), String> {
    use crate::schema::nodes::dsl::{node_order, nodes, roadmap_uuid};
    let conn = &mut establish_connection();
    if let Err(err) =
        diesel::update(nodes.filter((roadmap_uuid.eq(query_uuid)).and(node_order.gt(around))))
            .set(node_order.eq(node_order - 1))
            .execute(conn)
    {
        return Err(format!("Failed to squash nodes: {err}"));
    }
    Ok(())
}
