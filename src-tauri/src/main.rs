// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use base64::{Engine, engine::general_purpose};
use lib::create_hex_string_array;
mod lib;
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command


#[tauri::command]
fn fmt_image(img: String, conv: i32) -> String {
    let img_data = general_purpose::STANDARD
        .decode(img).unwrap();

    let mut output = String::new();

    if lib::convTypes::RGB8 as i32 == conv {
        output = create_hex_string_array(img_data);
    }

    output
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![fmt_image])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}