// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use color_eyre::Result;

fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    desktop_lib::run()
}
