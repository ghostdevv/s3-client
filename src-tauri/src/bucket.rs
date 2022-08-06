use crate::state;
use sqlx::sqlite::SqliteRow;
use sqlx::Row;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Bucket {
	pub id: i32,
	pub name: String,
	pub bucket: String,
	pub endpoint: String,
	pub region: String,
	pub key_id: String,
	pub key: String,
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
pub async fn list_buckets(state: state::TauriAppState<'_>) -> Result<Vec<Bucket>, String> {
	let query = sqlx::query_as("SELECT * FROM bucket");

	let buckets: Vec<Bucket> = match query.fetch_all(&state.db).await {
		Ok(buckets) => buckets,
		Err(e) => return Err(format!("Error fetching buckets: {}", e)),
	};

	Ok(buckets)
}

#[derive(Debug, serde::Deserialize)]
pub struct NewBucketData {
	name: String,
	bucket: String,
	endpoint: String,
	region: String,
	key_id: String,
	key: String,
}

#[tauri::command]
pub async fn create_bucket(
	bucket: NewBucketData,
	state: state::TauriAppState<'_>,
) -> Result<u32, String> {
	let query = sqlx::query(
		"INSERT INTO bucket (name, bucket, endpoint, region, key_id, key) VALUES (?, ?, ?, ?, ?, ?) returning id",
	)
	.bind(&bucket.name)
	.bind(&bucket.bucket)
	.bind(&bucket.endpoint)
	.bind(&bucket.region)
	.bind(&bucket.key_id)
	.bind(&bucket.key);

	let result = match query.fetch_one(&state.db).await {
		Ok(result) => result,
		Err(e) => return Err(format!("Error creating bucket: {}", e)),
	};

	let id: u32 = result
		.try_get("id")
		.expect("Unable to get id from bucket create");

	Ok(id)
}
