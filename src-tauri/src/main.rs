// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://v1.tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn cmd_no_args() {
    println!("invoked from TS without args");
}

#[tauri::command]
fn cmd_string_arg(invoke_message: String) {
    println!("invoked from TS, msg = {}", invoke_message);
}

// 注意是 `rename_all` instead of `rename`
#[tauri::command(rename_all = "snake_case")]
fn cmd_string_arg_snake_case(invoke_message: String) {
    println!("snake_case invoked from TS, msg = {}", invoke_message);
}

#[tauri::command]
fn cmd_return_string() -> String {
    println!("invoked from TS, return a String");
    "Hello From Rust!".into()
}

fn main() {
    tauri::Builder::default()
        //这里是个vector形式的写法, 而不是多次链式调用 invoke_handler
        .invoke_handler(tauri::generate_handler![
            greet,
            cmd_no_args,
            cmd_string_arg,
            cmd_string_arg_snake_case,
            cmd_return_string
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
