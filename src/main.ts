// 引入 createApp 用于创建应用
import { createApp } from "vue";
// 引入 App 根组件
import App from "./App.vue";

// 由于导入的源名称为index, 可以简写
import router from "./router"
// import router from "./router/index"

// createApp 创建 "根" 组件 --- App.vue
// const app = createApp(App)
// 使用 router
// app.use(router)
// mount 挂载 "枝叶" app
// app.mount("#app");
createApp(App).use(router).mount("#app");
