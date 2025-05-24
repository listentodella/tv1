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

#[derive(Debug)]
struct MyTauriState {
    info: String,
    val: usize,
}
#[derive(serde::Serialize)]
struct CustomResponse {
    message: String,
    other_val: usize,
}

async fn some_function() -> Option<String> {
    Some("response".into())
}
// 异步命令使用 async_runtime::spawn 在单线程上执行, 但命令本身的函数只需要用async 关键字标记即可, 非常简单
// 不带 async 关键字的命令将在主线程上执行，除非使用 #[tauri::command(async)] 定义
#[tauri::command]
async fn my_async_custom_command(
    app_handle: tauri::AppHandle, //获取app的句柄
    window: tauri::Window,        //获取该命令被调用时所在的window
    number: usize,
    my_tauri_state: tauri::State<'_, MyTauriState>, // 获取tauri的状态
) -> Result<CustomResponse, String> {
    println!("Called from window {}", window.label());
    println!("tauri state is {:?}", my_tauri_state);

    // 获取app的路径
    let app_dir = app_handle.path_resolver().app_data_dir();
    println!("app dir is {:?}", app_dir);
    // 监听全局快捷键
    use tauri::GlobalShortcutManager;
    let _ = app_handle
        .global_shortcut_manager()
        // 注册快捷键, 并绑定一个回调函数
        .register("CTRL + U", move || println!("CTRL + U pressed"))
        .map_err(|e| println!("Error registering shortcut: {}", e));

    let ret = some_function().await;
    if let Some(message) = ret {
        Ok(CustomResponse {
            message: message + " " + &my_tauri_state.info,
            other_val: number + my_tauri_state.val,
        })
    } else {
        Err("Error occurred".into())
    }
}

fn main() {
    tauri::Builder::default()
        // 这个manage是非常重要的, 否则 命令上无法使用 tauri::State
        .manage(MyTauriState {
            info: "Tauri State Info".into(),
            val: 100,
        })
        //这里是个vector形式的写法, 而不是多次链式调用 invoke_handler
        .invoke_handler(tauri::generate_handler![
            greet,
            cmd_no_args,
            cmd_string_arg,
            cmd_string_arg_snake_case,
            cmd_return_string,
            cmd_return_result,
            cmd_return_map_err,
            cmd_return_this_error,
            my_async_custom_command
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
