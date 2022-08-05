use crate::state;
use sqlx::sqlite::SqliteRow;
use sqlx::Row;

#[derive(Debug)]
pub struct Bucket {
	id: i32,
	name: String,
	bucket: String,
	endpoint: String,
	region: String,
	key_id: String,
	key: String,
}

impl sqlx::FromRow<'_, SqliteRow> for Bucket {
	fn from_row(row: &SqliteRow) -> sqlx::Result<Self> {
		Ok(Bucket {
			id: row.try_get("id")?,
			name: row.try_get("name")?,
			bucket: row.try_get("bucket")?,
			endpoint: row.try_get("endpoint")?,
			region: row.try_get("region")?,
			key_id: row.try_get("key_id")?,
			key: row.try_get("key")?,
		})
	}
}

#[tauri::command]
pub async fn list_buckets(state: state::TauriAppState<'_>) -> Result<(), String> {
	let query = sqlx::query_as("SELECT * FROM bucket");

	let buckets: Vec<Bucket> = match query.fetch_all(&state.db).await {
		Ok(buckets) => buckets,
		Err(e) => return Err(format!("Error fetching buckets: {}", e)),
	};

	println!("{:?}", buckets);

	Ok(())
}
