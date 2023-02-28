// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{fs::{File, self}, path::Path, env::current_dir, io::{ErrorKind, Write}};

use directories::ProjectDirs;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![create_network])
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