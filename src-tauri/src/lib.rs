use std::fs;
use std::io::{BufRead, BufReader};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_current_directory,
            read_csv_file,
        ])
        .run(tauri::generate_context!())
        .expect("error while running 'no-eagles'");
}

#[tauri::command]
fn get_current_directory() -> String {
    std::env::current_dir()
        .map(|p| p.display().to_string())
        .unwrap_or_else(|_| "Unable to get current dir".to_string())
}

#[tauri::command]
fn get_available_files(partial_string: &str) -> Vec<String> {
    vec![]
}

#[tauri::command]
fn read_csv_file(file_path: String) -> Result<Vec<Vec<String>>, String> {
    let file = fs::File::open(&file_path).map_err(|e| e.to_string())?;
    let reader = BufReader::new(file);

    let mut lines = reader.lines();

    // Skip header (or keep it if you want)
    let _header = lines
        .next()
        .ok_or("CSV file is empty")?
        .map_err(|e| e.to_string())?;

    let mut records = Vec::new();

    for line in lines {
        let line = line.map_err(|e| e.to_string())?;
        let fields: Vec<String> = line
            .trim()
            .split(',')
            .map(|f| f.trim().to_string())
            .collect();

        records.push(fields);
    }

    Ok(records)
}
