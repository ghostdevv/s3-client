#![cfg_attr(
	all(not(debug_assertions), target_os = "windows"),
	windows_subsystem = "windows"
)]

use tauri::generate_handler;

mod bucket;
mod db;
mod paths;
mod state;

#[tokio::main]
async fn main() {
	let context = tauri::generate_context!();
	let paths = paths::AppPaths::new(&context.config());

	let db = db::init(&paths).await;
	let state = state::AppState { db };

	tauri::Builder::default()
		.manage(state)
		.invoke_handler(generate_handler![bucket::list_buckets])
		.run(context)
		.expect("error while running tauri application");
}
