use std::env;

#[derive(Debug)]
pub struct AppPaths {
	pub db: String,
	pub dir: String,
}

impl AppPaths {
	pub fn new(config: &tauri::Config) -> Self {
		let dir = match env::var("DEVELOPMENT").is_ok() {
			true => env::current_dir().unwrap().join(".appdata"),
			false => tauri::api::path::app_dir(config).unwrap(),
		};

		AppPaths {
			db: dir.join("s3client.sqlite").to_string_lossy().to_string(),
			dir: dir.to_string_lossy().to_string(),
		}
	}
}
