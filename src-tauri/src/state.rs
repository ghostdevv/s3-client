use sqlx::SqlitePool;

pub struct AppState {
	pub db: SqlitePool,
}

pub type TauriAppState<'a> = tauri::State<'a, AppState>;
