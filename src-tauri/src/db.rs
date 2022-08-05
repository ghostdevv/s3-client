use crate::paths;
use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};
use std::fs;

pub async fn init(paths: &paths::AppPaths) -> SqlitePool {
	let db_exists = Sqlite::database_exists(&paths.db)
		.await
		.expect("Error whilst checking if db exists");

	if !db_exists {
		fs::create_dir_all(&paths.dir).expect("Failed to create db data directory");

		Sqlite::create_database(&paths.db)
			.await
			.expect("Failed to create sqlite database")
	}

	let pool = SqlitePool::connect(&paths.db)
		.await
		.expect("Failed to connect to sqlite database");

	sqlx::migrate!("./migrations")
		.run(&pool)
		.await
		.expect("Failed to migrate database");

	return pool;
}
