fn main() {
	// trigger recompilation when a new migration is added
	println!("cargo:rerun-if-changed=migrations");

	// Build Tauri
	tauri_build::build()
}
