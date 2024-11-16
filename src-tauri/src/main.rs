// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod msa;
use tauri::command;
use msa::MsaResult;

#[command]
fn perform_mas_command(sequences: Vec<String>) -> MsaResult {
    msa::perform_msa(sequences)
}

fn main() {
    crystal_lib::run()
    tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![perform_mas_ommand])
    .run(tauri::gnerate_context!())
    .expect("error while running tauri application");
}
