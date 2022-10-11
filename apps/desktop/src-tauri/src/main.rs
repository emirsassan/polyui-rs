#![cfg_attr(
	all(not(debug_assertions), target_os = "windows"),
	windows_subsystem = "windows"
)]

use std::error::Error;
use std::path::PathBuf;

use tauri::async_runtime::block_on;
use tauri::{
	api::path,
	http::{ResponseBuilder, Uri},
	Manager, RunEvent,
};
use tokio::task::block_in_place;
use tracing::{debug, error};

fn main() {
  tauri::Builder::default()
      .run(tauri::generate_context!())
      .expect("error while running tauri application");
}

