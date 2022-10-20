#![cfg_attr(
	all(not(debug_assertions), target_os = "windows"),
	windows_subsystem = "windows"
)]

use tauri::generate_handler;

mod bucket;
mod db;
mod paths;
mod s3;
mod state;

#[tokio::main]
async fn main() {
	let context = tauri::generate_context!();
	let paths = paths::AppPaths::new(&context.config());

	let db = db::init(&paths).await;
	let state = state::AppState { db };

	tauri::Builder::default()
		.manage(state)
		.invoke_handler(generate_handler![
			bucket::list_buckets,
			bucket::create_bucket,
			s3::list_bucket_tree,
			s3::get_file_meta
		])
		.run(context)
		.expect("error while running tauri application");
}
