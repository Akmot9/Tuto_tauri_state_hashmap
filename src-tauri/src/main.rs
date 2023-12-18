
use std::sync::{Mutex, Arc};
use std::collections::HashMap;
use tauri::State;
use rand::Rng;
use tokio::time::{self, Duration};
use csv::Writer;
use thiserror::Error;

#[derive(Debug, Error, serde::Serialize)]
enum MyError {
    #[error("IO Error: {0}")]
    IoError(String),

    #[error("CSV Error: {0}")]
    CsvError(String),

    #[error("UTF-8 Conversion Error: {0}")]
    Utf8Error(String),
}
struct StringTauriState(Arc<Mutex<HashMap<String, u32>>>);

#[tauri::command]
fn push_to_hash_map(word: String, shared_hash_map: State<StringTauriState>) -> HashMap<String, u32> {
    let mut hash_map = shared_hash_map.0.lock().expect("Failed to lock the mutex");
    *hash_map.entry(word).or_insert(0) += 1;

    hash_map.clone()
}

#[tauri::command(async,rename_all = "snake_case")]
fn create_threads(num_threads: usize, shared_hash_map: State<'_, StringTauriState>) -> Result<(), String> {
    let shared_hash_map = shared_hash_map.0.clone();

    for _ in 0..num_threads {
        let hash_map_clone = shared_hash_map.clone();
        tokio::spawn(async move {
            let mut interval = time::interval(Duration::from_secs(1));
            loop {
                interval.tick().await;
                let random_number = rand::thread_rng().gen_range(1..=10);
                let key = format!("Number {}", random_number);
                let mut map = hash_map_clone.lock().expect("Failed to lock the mutex");
                *map.entry(key).or_insert(0) += 1;
            }
        });
    }
    Ok(())
}

#[tauri::command]
fn get_hash_map_state(shared_hash_map: State<'_, StringTauriState>) -> HashMap<String, u32> {
    let hash_map = shared_hash_map.0.lock().expect("Failed to lock the mutex");
    hash_map.clone()
}

#[tauri::command(async)]
fn save_hash_map_to_csv(file_path: String, shared_hash_map: State<'_, StringTauriState>) -> Result<String, MyError> {
    let hash_map = shared_hash_map.0.lock().map_err(|e| 
        MyError::IoError(e.to_string())
    )?;

    let mut wtr = Writer::from_writer(vec![]);
    for (key, value) in hash_map.iter() {
        wtr.write_record(&[key, &value.to_string()]).map_err(|e| 
            MyError::CsvError(e.to_string())
        )?;
    }
    wtr.flush().map_err(|e| 
        MyError::CsvError(e.to_string())
    )?;
    
    let data = String::from_utf8(wtr.into_inner().map_err(|e| 
        MyError::CsvError(e.to_string())
    )?).map_err(|e| 
        MyError::Utf8Error(e.to_string())
    )?;

    std::fs::write(&file_path, data).map_err(|e| 
        MyError::IoError(e.to_string())
    )?;

    Ok(format!("Data saved to {}", &file_path))
}

fn main() {
    tauri::Builder::default()
        .manage(StringTauriState(Arc::new(Mutex::new(HashMap::new()))))
        .invoke_handler(tauri::generate_handler![
            get_hash_map_state,
            push_to_hash_map, 
            create_threads,
            save_hash_map_to_csv])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
