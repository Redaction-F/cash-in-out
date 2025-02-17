// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

use std::io::Write;
use env_logger;

extern crate cash_in_out;

#[async_std::main]
async fn main() {
    dotenv::dotenv().expect("Failed to read .env file");
    env_logger::Builder::from_default_env()
        .format(|buf, record| {
            let time: env_logger::fmt::Timestamp = buf.timestamp();
            writeln!(buf, "[{} {:>5} {}]{}:{}\n\t{}", time, record.level(), record.target(), record.file().unwrap_or("unknown file"), record.line().unwrap_or(0), record.args())
        })
        .init();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![cash_in_out::first_get_from_db, cash_in_out::count_true])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
