// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

extern crate cash_in_out;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![cash_in_out::read_csv, cash_in_out::count_true])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
