// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{fs::{File, self, OpenOptions}, path::Path, env::current_dir, io::{ErrorKind, Write}};
use comrak::{markdown_to_html, ComrakOptions};
use directories::ProjectDirs;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![create_network, create_node, open_node, parse_md, save_node, search_nodes])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
fn create_network(location: String, name: String) {
    let mut cratis_dir = location.clone();
    cratis_dir.push_str(&format!("/{}", &name));
    let mut nodes_dir = cratis_dir.clone();
    nodes_dir.push_str("/nodes");
    let mut journal_dir = cratis_dir.clone();
    journal_dir.push_str("/journal");

    // create network directories
    fs::create_dir(cratis_dir).expect("Could not create network directory");
    fs::create_dir(nodes_dir).expect("Could not create nodes directory");
    fs::create_dir(journal_dir).expect("Could not create journal directory");
}

#[tauri::command]
fn create_node(location: String, name: String) {
    let node_path = format!("{location}/{name}.md").to_string();
    if !Path::new(&node_path).exists() {
        let node_file = File::create(node_path).expect("Could not create node");
    }
}

#[tauri::command]
fn open_node(nodePath: String) -> String {
    fs::read_to_string(nodePath).expect("Could not open node")
}

#[tauri::command]
fn parse_md(content: String) -> String {
    markdown_to_html(&content, &ComrakOptions::default())
} 

#[tauri::command]
fn save_node(nodeStr: String, nodePath: String) {
    if nodePath != "" {
        let mut file = OpenOptions::new().write(true).open(nodePath).expect("Could not open file");
        file.write_all(nodeStr.as_bytes()).expect("Could not write node");
    }
}    

#[tauri::command]
fn search_nodes(searchVal: String, cratisDir: String) -> Vec<String> {
    let mut nodes = Vec::new();
    let mut nodes_dir = cratisDir.clone();
    nodes_dir.push_str("/nodes/");

    let node_paths = fs::read_dir(nodes_dir).unwrap();

    for path in node_paths {
        // iterate through all nodes and add to nodes vec if matches
        let mut node_name = path.unwrap().file_name().to_str().unwrap().to_string();
        node_name.truncate(node_name.len() - 3);
        
        if node_name.contains(&searchVal) {
            nodes.push(node_name);
        }
    }
    nodes
}
