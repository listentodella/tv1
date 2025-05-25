<script setup lang="ts">
import { ref } from "vue";
// 这是调用 rust 代码必须得导入
import { invoke } from "@tauri-apps/api/tauri";

import Person from "./components/Person.vue";

// 新版本的vue3已经不需要了
// export default {
//   name: "App",// 组件名称
//   components: {
//     Person, // 注册组件
//   },
// }

const greetMsg = ref("");
const name = ref("");

// 直接调用即可, 但是结果是在rust侧查看, 而不是vue侧
invoke("cmd_no_args")
// 参数应该带有驼峰式风格的 JSON 对象 传递
// 参数可以是任何类型, 只要它们实现了 serde::Deserialize 即可
invoke("cmd_string_arg", { invokeMessage: "Hello!" })

// snake case 风格的传递, rust侧必须用相应的宏修饰
invoke("cmd_string_arg_snake_case", { invoke_message: "snake_hello" })

// rust侧可以返回值到vue侧
// 同样要求返回值类型实现 serde::Serialize
invoke("cmd_return_string").then((msg) => console.log(msg));

// rust侧返回的结果是 Result<T, E> 类型
// 如果 ok 为 true, msg 就是 T, 如果 ok 为 false, msg 就是 E
invoke("cmd_return_result", { ok: true })
  .then((msg) => { console.log(msg) })
  .catch((err) => { console.error(err) })
invoke("cmd_return_result", { ok: false })
  .then((msg) => { console.log(msg) })
  .catch((err) => { console.error(err) })

invoke("cmd_return_map_err", { ok: true })
  .then((msg) => { console.log(msg) })
  .catch((err) => { console.error(err) })
invoke("cmd_return_map_err", { ok: false })
  .then((msg) => { console.log(msg) })
  .catch((err) => { console.error(err) })

invoke("cmd_return_this_error", { num: 0 })
  .then((msg) => { console.log(msg) })
  .catch((err) => { console.error(err) })
invoke("cmd_return_this_error", { num: 1 })
  .then((msg) => { console.log(msg) })
  .catch((err) => { console.error(err) })
invoke("cmd_return_this_error", { num: 2 })
  .then((msg) => { console.log(msg) })
  .catch((err) => { console.error(err) })

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

type CustomResponse = {
  message: string;
  other_val: number;
};
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

async function greet() {
  // Learn more about Tauri commands at https://v1.tauri.app/v1/guides/features/command
  greetMsg.value = await invoke("greet", { name: name.value });
}
</script>

<template>
  <div class="app">
    <h3>vue3+ts learn</h3>
    <Person />
  </div>
  <main class="container">
    <h1>Tauri + Vue</h1>
    <div class="row">
      <a href="https://v1.tauri.app/zh-cn/v1/guides/getting-started/setup" target="_blank">
        <img src="/tauri.svg" class="logo tauri" alt="Tauri logo" />
      </a>
      <a href="https://cn.vuejs.org/guide/introduction.html" target="_blank">
        <img src="./assets/vue.svg" class="logo vue" alt="Vue logo" />
      </a>
    </div>
    <p>Click Tauri or Vue logos for tutorials</p>

    <form class="row" @submit.prevent="greet">
      <input id="greet-input" v-model="name" placeholder="Enter a name..." />
      <button type="submit">Greet</button>
    </form>
    <p>{{ greetMsg }}</p>
  </main>
</template>

<style scoped>
.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}
</style>
<style>
:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.container {
  margin: 0;
  padding-top: 10vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
}

.logo {
  height: 3em;
  padding: 1.5em;
  will-change: filter;
  transition: 0.75s;
}

.logo.tauri:hover {
  filter: drop-shadow(0 0 2em #24c8db);
}

.row {
  display: flex;
  justify-content: center;
}

a {
  font-weight: 500;
  color: #646cff;
  text-decoration: inherit;
}

a:hover {
  color: #535bf2;
}

h1 {
  text-align: center;
}

input,
button {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: #0f0f0f;
  background-color: #ffffff;
  transition: border-color 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}

button {
  cursor: pointer;
}

button:hover {
  border-color: #396cd8;
}

button:active {
  border-color: #396cd8;
  background-color: #e8e8e8;
}

input,
button {
  outline: none;
}

#greet-input {
  margin-right: 5px;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }

  a:hover {
    color: #24c8db;
  }

  input,
  button {
    color: #ffffff;
    background-color: #0f0f0f98;
  }

  button:active {
    background-color: #0f0f0f69;
  }
}
</style>