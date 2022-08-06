use crate::bucket;
use s3::{creds::Credentials, Bucket, Region};

async fn resolve_bucket(bucket: &bucket::Bucket) -> Bucket {
	let credentials = Credentials::new(Some(&bucket.key_id), Some(&bucket.key), None, None, None)
		.expect("Failed to create credentials");

	let region = Region::Custom {
		region: bucket.region.clone(),
		endpoint: bucket.endpoint.clone(),
	};

	Bucket::new(&bucket.bucket, region, credentials).expect("Failed to create bucket")
}

#[tauri::command]
pub async fn list_bucket_tree(bucket: bucket::Bucket) -> Vec<String> {
	let creds = resolve_bucket(&bucket).await;

	let objects = creds
		.list(String::default(), Some(String::default()))
		.await
		.expect("Error listing objects");

	let mut paths: Vec<String> = vec![];

	for object in objects {
		let formatted = object
			.contents
			.iter()
			.map(|o| o.key.to_owned())
			.collect::<Vec<String>>();

		for path in formatted {
			paths.push(path);
		}
	}

	return paths;
}
