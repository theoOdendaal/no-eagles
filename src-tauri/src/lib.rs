use std::fs;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            validate_file_exists,
            get_current_directory,
            get_list_of_files,
            read_csv_file,
        ])
        .run(tauri::generate_context!())
        .expect("error while running 'no-eagles'");
}

/*
#[tauri::command]
fn validate_file_exists() -> bool {
    let file = Path::new(r"C:/Users/TheoOdendaal/source/repos/no-eagles\bloemhof 1 jo.csv");
    println!("{:?}", file.exists() && file.is_file());
    file.exists() && file.is_file()
}
*/

#[tauri::command]
fn validate_file_exists(directory: &str) -> bool {
    let file = Path::new(directory.trim());
    file.exists() && file.is_file()
}
// TODO make this function return &'a str.
#[tauri::command]
fn get_current_directory() -> String {
    std::env::current_dir()
        .map(|p| p.display().to_string())
        .unwrap_or_else(|_| "Unable to get current dir".to_string())
}

#[tauri::command]
fn get_list_of_files(directory: &str) -> Result<Vec<String>, String> {
    fn visit_dir(dir: &Path, files: &mut Vec<String>) -> Result<(), String> {
        let entries = fs::read_dir(dir).map_err(|e| format!("Failed to read dir: {e}"))?;

        for entry_result in entries {
            let entry = entry_result.map_err(|e| format!("Failed to read entry: {e}"))?;
            let path = entry.path();
            let metadata = entry
                .metadata()
                .map_err(|e| format!("Failed to read metadata: {e}"))?;

            if metadata.is_file() {
                if let Some(path_str) = path.to_str() {
                    files.push(path_str.to_string());
                }
            } else if metadata.is_dir() {
                visit_dir(&path, files)?; // Recursive call
            }
        }

        Ok(())
    }

    let mut collected_files = Vec::new();
    let path = Path::new(directory.trim());

    if !path.exists() || !path.is_dir() {
        return Err("Provided path is not a valid directory.".to_string());
    }

    visit_dir(path, &mut collected_files)?;

    Ok(collected_files)
}
/*
#[tauri::command]
fn get_list_of_files(directory: &str) -> Result<Vec<String>, String> {
    match fs::read_dir(directory) {
        Ok(entries) => {
            let files = entries
                .flatten()
                .filter_map(|entry| {
                    let file_type = entry.file_type().ok()?;
                    if file_type.is_file() {
                        let name = entry.path().to_str()?.to_string();
                        return Some(name);
                    }
                    None
                })
                .collect();
            Ok(files)
        }
        Err(e) => Err(format!("Failed to read directory: {e}")),
    }
}
    */

#[tauri::command]
fn read_csv_file(file_path: &str) -> Result<Vec<Vec<String>>, String> {
    println!("Hello");
    let file = fs::File::open(file_path).map_err(|e| e.to_string())?;
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
