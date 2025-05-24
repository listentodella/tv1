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

#[tauri::command]
fn cmd_return_result(ok: bool) -> Result<String, String> {
    if ok {
        println!("return Ok to TS");
        Ok("Ok From Rust!".into())
    } else {
        println!("return Err to TS");
        Err("Error From Rust!".into())
    }
}

// map_err 是一个不错的闭包方法, 可以相对方便的将错误信息传递给 TS
#[tauri::command]
fn cmd_return_map_err(ok: bool) -> Result<(), String> {
    if ok {
        println!("return Ok but nothing to TS");
        Ok(())
    } else {
        std::fs::File::open("path/that/does/not/exist").map_err(|err| {
            println!("return err info to TS");
            err.to_string()
        })?;
        Ok(())
    }
}

// 使用thiserror
#[derive(Debug, thiserror::Error)]
enum MyError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("DIY error num:{0}")]
    MyErrNum(u8),
}
// 这是必须的
impl serde::Serialize for MyError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

#[tauri::command]
fn cmd_return_this_error(num: u8) -> Result<String, MyError> {
    if num == 0 {
        println!("return MyError::Io to TS");
        std::fs::File::open("path/that/does/not/exist")?;
        Ok(("should not reach here").into())
    } else if num == 1 {
        println!("return MyError::MyErrNum to TS");
        Err(MyError::MyErrNum(num))
    } else {
        println!("return ok to TS here");
        Ok("Ok From Rust in thiserror".into())
    }
}

fn main() {
    tauri::Builder::default()
        //这里是个vector形式的写法, 而不是多次链式调用 invoke_handler
        .invoke_handler(tauri::generate_handler![
            greet,
            cmd_no_args,
            cmd_string_arg,
            cmd_string_arg_snake_case,
            cmd_return_string,
            cmd_return_result,
            cmd_return_map_err,
            cmd_return_this_error
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
