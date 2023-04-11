use std::{fs, io::Read, path::Path};

use rusqlite::{Connection, Result, Batch};
use regex::Regex;
use log::{debug, LevelFilter};
use env_logger::Builder;

#[tauri::command]
pub fn create_database(databaseDir: String) {
    let conn = Connection::open(&databaseDir).expect("Could not connect to db");
    // Create tables if they do not exist
    conn.execute(
        "CREATE TABLE IF NOT EXISTS nodes (
            id INTEGER PRIMARY KEY,
            name TEXT UNIQUE
        )",
        (),
    ).expect("Could not create table nodes");
    
    conn.execute(
        "CREATE TABLE IF NOT EXISTS links (
            source_node TEXT REFERENCES nodes(name),
            target_node TEXT REFERENCES nodes(name)
        )",
        (),
    ).expect("Could not create table links");
}

#[derive(Debug)]
struct Link {
    source_node: String,
    target_node: String
}

#[tauri::command]
pub fn index_nodes(databaseDir: String, nodesDir: String) -> Result<(), String> {
    let mut conn = Connection::open(&databaseDir).expect("Could not connect to db");

    let mut builder = Builder::new();
    builder.filter(None, LevelFilter::Debug).init();

    // enable logging
    conn.trace(Some(|sql| {
        debug!("Executing SQL statement: {}", sql);
    }));

    let node_paths = fs::read_dir(nodesDir).unwrap();
    // empty tables
    conn.execute_batch(
        "DELETE FROM links;
        DELETE FROM nodes;"
    ).expect("Could not delete tables");

    for path_result in node_paths {
        let path = path_result.unwrap().path();
        let mut node_name = path.file_name().unwrap().to_str().unwrap().to_string();
        node_name.truncate(node_name.len() - 3);
        
        // add to nodes table
        conn.execute(
            "INSERT OR IGNORE INTO nodes (name) VALUES (?1)",
            [&node_name],
        ).expect("Could not insert into nodes table");

        // find references
        let mut file = fs::File::open(&path).unwrap();
        let mut node_content = String::new();
        file.read_to_string(&mut node_content).unwrap();

        let re = Regex::new(r#"(#|(\[\[))(?P<content>[^\s\]]+)(\]\])?"#).unwrap();
        let mut refs: Vec<String> = Vec::new();

        for capture in re.captures_iter(&node_content) {
            if let Some(match_str) = capture.name("content") {
                refs.push(match_str.as_str().to_string());
            }
        }

        for link in refs {
            let sql = r"
                INSERT OR IGNORE INTO nodes (name) VALUES (?2);
                INSERT INTO links (source_node, target_node) VALUES (?1, ?2);
                ";
            let mut batch = Batch::new(&conn, sql);
            while let Some(mut stmt) = batch.next().unwrap() {
                stmt.execute([&node_name, &link]).unwrap();
            }
        }
    }
    let mut ref_stmt = conn.prepare("SELECT source_node, target_node FROM links").unwrap();
    let refers = ref_stmt.query_map([], |row| {
        Ok(Link {
            source_node: row.get(0).unwrap(),
            target_node: row.get(1).unwrap()
        })
    }).unwrap();

    for refer in refers {
        println!("REFERENCE: {:?}\n\n", refer.unwrap());
    }

    Ok(())
}

#[tauri::command]
pub fn add_node(databaseDir: String, nodeName: String) {
    let conn = Connection::open(&databaseDir).expect("Could not open db");
    
    conn.execute("
                 INSERT OR IGNORE INTO nodes (name) VALUES (?1)
                 ",
                 [&nodeName]
    ).expect("Could not insert node into table nodes");
}

#[tauri::command]
pub fn update_references(databaseDir: String, nodePath: String) {
    let conn = Connection::open(&databaseDir).expect("Could not open db");

    let mut file = fs::File::open(&nodePath).expect("Could not open file");
    let mut node_content = String::new();
    file.read_to_string(&mut node_content).unwrap();

    let node_path = Path::new(&nodePath);
    let mut node_name = node_path.file_name().unwrap().to_str().unwrap().to_string();
    node_name.truncate(node_name.len() - 3);

    let re = Regex::new(r#"(#|(\[\[))(?P<content>[^\s\]]+)(\]\])?"#).unwrap();
    let mut refs: Vec<String> = Vec::new();

    for capture in re.captures_iter(&node_content) {
        if let Some(match_str) = capture.name("content") {
            refs.push(match_str.as_str().to_string());
        }
    }

    conn.execute("DELETE FROM links WHERE source_node = ?1", [&node_name]).expect("Could not delete source node references from links table");
    for link in refs {
        let sql = r"
            INSERT OR IGNORE INTO nodes (name) VALUES (?2);
            INSERT INTO links (source_node, target_node) VALUES (?1, ?2)
            ";
        let mut batch = Batch::new(&conn, sql);
        while let Some(mut stmt) = batch.next().unwrap() {
            stmt.execute([&node_name, &link]).unwrap();
        }
    }
}
