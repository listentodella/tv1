// 引入 createApp 用于创建应用
import { createApp } from "vue";
// 引入 App 根组件
import App from "./App.vue";

// createApp 创建 "根" 组件 --- App.vue
// mount 挂载 "枝叶" app
createApp(App).mount("#app");
