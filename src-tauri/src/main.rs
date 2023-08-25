// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::time::Duration;
use std::thread;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
/// 这里的参数名字必须和js传入的参数的名字相同
fn greet(name: &str) -> String {
    let ten_seconds = Duration::from_secs(10);
    // Start a new thread to run the task
    let handle = thread::spawn(move || {
        loop {
            // Your task code here
            println!("Executing task...");
            // Sleep for 10 seconds
            thread::sleep(ten_seconds);
        }
    });
    // Wait for the thread to finish
    handle.join().unwrap();
    String::from("xxx")
}
