<script setup lang="ts">
import { RouterLink, RouterView } from "vue-router";

// 新版本的vue3已经不需要了
// export default {
//   name: "App",// 组件名称
//   components: {
//     Person, // 注册组件
//   },
// }

</script>

<template>
  <main class="container">
    <h1>Tauri + Vue</h1>
    <div class="row">
      <a href="https://v1.tauri.app/zh-cn/v1/guides/getting-started/setup" target="_blank">
        <img src="/tauri.svg" class="logo tauri" alt="Tauri logo" />
      </a>
      <a href="https://cn.vuejs.org/guide/introduction.html" target="_blank">
        <img src="./assets/vue.svg" class="logo vue" alt="Vue logo" />
      </a>
      <a href="https://wangdoc.com/typescript/" target="_blank">
        <img src="./assets/typescript.svg" class="logo ts" alt="TS logo" />
      </a>
      <a href="https://www.runoob.com/html/html-tutorial.html" target="_blank">
        <img src="./assets/html5.svg" class="logo html" alt="HTML logo" />
      </a>
    </div>
    <p>Click logos for tutorials</p>
  </main>

  <div class="app0">
    <!-- 导航区 -->
    <div class="navi">
      <!-- 可以直接写路径, 但是一旦路径较深, 就会变得麻烦 -->
      <!-- <RouterLink :to="{ path: 'home' }" active-class="active">首页</RouterLink> -->
      <RouterLink to="/home" active-class="active">首页</RouterLink>
      <!-- 可以通过名字跳转, 但要有name属性 -->
      <RouterLink :to="{ name: 'tauri' }" active-class="active">Tauri</RouterLink>
      <RouterLink :to="{ name: 'routers' }" active-class="active">嵌套路由</RouterLink>
      <RouterLink :to="{ name: 'reactivity' }" active-class="active">响应式</RouterLink>
      <RouterLink :to="{ name: 'lifecycle' }" active-class="active">生命周期</RouterLink>
      <RouterLink :to="{ name: 'diyhook' }" active-class="active">自定义Hook</RouterLink>
      <RouterLink :to="{ name: 'about' }" active-class="active">关于</RouterLink>
    </div>
    <!-- 展示区 -->
    <div class="main-content">
      <!-- 展示区占位即可 -->
      <RouterView></RouterView>
      <!-- <router-view></router-view> -->
    </div>
  </div>
</template>

<style scoped>
.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}

.navi {
  display: flex;
  gap: 5px;
  /* 每个链接之间的间距 */
  padding: 8px;
  background-color: #f8f8f8;
  border-radius: 8px;
}

.navi a {
  text-decoration: none;
  color: #333;
  padding: 6px 12px;
  border-radius: 4px;
  transition: background-color 0.2s;
}

.navi a.active {
  background-color: #007bff;
  color: white;
}

.navi a:hover {
  background-color: #e0e0e0;
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
  padding-top: 1vh;
  display: flexbox;
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