pub fn data_dir() -> std::path::PathBuf {
    // Get data directory
    let data_dir = tauri::api::path::data_dir().expect("failed to get data dir");

    // Append 'isotope' to the data directory path
    let data_dir = data_dir.join("isotope");

    // Check if the data directory exists and create it if it doesn't
    if !data_dir.exists() {
        std::fs::create_dir(&data_dir).expect("failed to create data directory");
    }

    // Return the data directory path as a string
    data_dir
}

pub fn models_dir_path() -> String {
    // Get data directory
    let data_dir = data_dir();

    // Append 'model' to the data directory path
    let models_path = data_dir.join("models");

    // Check if the model directory exists and create it if it doesn't
    if !models_path.exists() {
        std::fs::create_dir(&models_path).expect("failed to create model directory");
    }

    // Return the model directory path as a string
    models_path.to_str().unwrap().to_string()
}
