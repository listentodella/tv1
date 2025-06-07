<template>
    <div class="tauri">
        <h1>Tauri</h1>
    </div>

    <form class="row" @submit.prevent="greet">
        <input id="greet-input" v-model="name" placeholder="Enter a name..." />
        <button type="submit">Greet</button>
    </form>
    <p>{{ greetMsg }}</p>
    <li v-for="cmd in cmdList" :key="cmd.id">
        <button
            @click="invoke(cmd.name, cmd.param).then((msg) => console.log(msg)).catch((err) => console.error(err))">{{
                cmd.desc
            }}</button>
    </li>
</template>

<script lang="ts" setup name="Tauri">
import { ref } from "vue";
// 这是调用 rust 代码必须得导入
import { invoke } from "@tauri-apps/api/core";
// 全局事件
import { emit, listen } from "@tauri-apps/api/event";
// 特定窗口事件
import { getCurrentWebviewWindow, WebviewWindow } from "@tauri-apps/api/webviewWindow";
const appWindow = getCurrentWebviewWindow()


// 监听 click event并获取一个函数以移除event listener
// 也可以使用 `once` 函数订阅一个event, 并在触发时自动event listener
// const unlisten = await listen("click", (event) => {
//   // event.event 是event名称(当你想用一个回调处理不同类型的event时会很有用)
//   // event.payload 是event的负载对象
//   console.log(event);
//   unlisten()// 取消监听
// })
// 携带payload对象,触发 click event
// emit("click", { theMessage: "Tauri is awesome!" }) // 发送event

// 触发一个仅当前窗口可见的event
appWindow.emit("event", { message: "Tauri is awesome in this window only!" })
// 创建一个新的webview窗口,并触发一个仅对它可见的event
const webview = new WebviewWindow("window")
webview.emit("event")


// 直接调用即可, 但是结果是在rust侧查看, 而不是vue侧
invoke("cmd_no_args")
// 参数应该带有驼峰式风格的 JSON 对象 传递
// 参数可以是任何类型, 只要它们实现了 serde::Deserialize 即可
invoke("cmd_string_arg", { invokeMessage: "Hello!" })

// snake case 风格的传递, rust侧必须用相应的宏修饰
invoke("cmd_string_arg_snake_case", { invoke_message: "snake_hello" })

// 尽管rust侧的函数入参实际更复杂:
// my_custom_command(
//     app_handle: tauri::AppHandle,
//     window: tauri::Window,
//     number: usize,
//     my_tauri_state: tauri::State<'_, MyTauriState>
// ) -> Result<struct CustomResponse {
//            message: String,other_val: usize, }, String>
// 但是tauri::{AppHandle, Window, State} 这两个参数会被自动注入?
// 所以只需要传入 number 参数即可

// interface也可以, 但是含义与type完全不同
// interface CustomResponse {
//   message: string;
//   other_val: number;
// }
type CustomResponse = {
    message: string;
    other_val: number;
}
// invoke("my_async_custom_command", { number: 10 }).then((ret:any) => { // 这会失去TS类型检查的意义, 并非最佳实践
// invoke的默认返回类型是 Promise<any>, 但是我们可以使用泛型来指定返回值的类型
invoke<CustomResponse>("my_async_custom_command", { number: 10 }).then((ret) => { // 充分利用TS类型检查:)
    // 返回值直接打印即可
    // 但是也可以将值取出, 这样方便后续提取另作他用
    console.log(ret);
    let message = ret.message;
    let val = ret.other_val;
    console.warn(message);
    console.warn(val);
})
    .catch((err) => { console.error(err) })



const greetMsg = ref("");
const name = ref("");
async function greet() {
    // Learn more about Tauri commands at https://v1.tauri.app/v1/guides/features/command
    greetMsg.value = await invoke("greet", { name: name.value });
}

const cmdList = [
    { id: "cmd_001", name: "cmd_no_args", param: {}, desc: "无参调用" },
    { id: "cmd_002", name: "cmd_string_arg", param: { invokeMessage: "HelloWorld!" }, desc: "带参调用(string)" },
    { id: "cmd_003", name: "cmd_string_arg_snake_case", param: { invoke_message: "hello_world!" }, desc: "带参调用(snake_case)" },

    // rust侧可以返回值到vue侧
    // 同样要求返回值类型实现 serde::Serialize
    { id: "cmd_004", name: "cmd_return_string", param: {}, desc: "返回字符串" },

    // rust侧返回的结果是 Result<T, E> 类型
    // 如果 ok 为 true, msg 就是 T, 如果 ok 为 false, msg 就是 E
    { id: "cmd_005", name: "cmd_return_result", param: { ok: true }, desc: "返回Result Ok" },
    { id: "cmd_006", name: "cmd_return_result", param: { ok: false }, desc: "返回Result Err" },
    { id: "cmd_007", name: "cmd_return_map_err", param: { ok: true }, desc: "返回自定义错误 Ok" },
    { id: "cmd_008", name: "cmd_return_map_err", param: { ok: false }, desc: "返回自定义错误 Err" },
    { id: "cmd_009", name: "cmd_return_this_error", param: { num: 0 }, desc: "返回(thiserror) std err" },
    { id: "cmd_010", name: "cmd_return_this_error", param: { num: 1 }, desc: "返回(thiserror) DIY err" },
    { id: "cmd_011", name: "cmd_return_this_error", param: { num: 2 }, desc: "返回(thiserror) Ok" },
    { id: "cmd_012", name: "my_async_custom_command", param: { number: 10 }, desc: "自定义异步调用" },
]

</script>
