// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

extern crate proc_macro;

use env_logger;
use log::warn;
use std::io::Write;

#[async_std::main]
async fn main() {
    // read .env
    if let Err(_) = dotenv::from_filename("/home/redaction-f/.config/cash-in-out/.env") {
        warn!("Failed to read .env file");
    }

    // build logger
    env_logger::Builder::from_default_env()
        .format(|buf, record| {
            let time: env_logger::fmt::Timestamp = buf.timestamp();
            writeln!(
                buf,
                "[{} {:>5} {}]{}:{}\n\t{}\n",
                time,
                record.level(),
                record.target(),
                record.file().unwrap_or("unknown file"),
                record.line().unwrap_or(0),
                record.args()
            )
        })
        .init();

    // build tauri
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            cash_in_out::get_records_by_month,
            cash_in_out::get_record_by_id,
            cash_in_out::get_sum_by_month,
            cash_in_out::get_sum_by_month_group_by_main_category,
            cash_in_out::update_record,
            cash_in_out::create_record,
            cash_in_out::delete_record_by_id,
            cash_in_out::get_all_categorys,
            cash_in_out::create_main_category,
            cash_in_out::delete_main_category,
            cash_in_out::create_sub_category,
            cash_in_out::delete_sub_category,
            cash_in_out::write_in_csv,
            cash_in_out::read_from_csv
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
