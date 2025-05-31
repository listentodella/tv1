// 1. 导入router
import { createRouter, createWebHistory } from "vue-router"
// import Home from "@/pages/Home.vue"
// import News from "@/pages/News.vue"
// import About from "@/pages/About.vue"
import Home from "../pages/Home.vue"
import Routers from "../pages/Routers.vue"
import About from "../pages/About.vue"
import Detail from "../pages/Detail.vue"
import Reactivity from "../pages/Reactivity.vue"
import Tauri from "../pages/Tauri.vue"
import Lifecycle from "../pages/Lifecycle.vue"
import DiyHook from "../pages/DiyHook.vue"


// 2. 创建 router
const router = createRouter({
    // createWebHistory 适用于大多数场景, 地址栏不会有 # 符号, 但是后端一定要
    history: createWebHistory(),
    // 有哪些路由规则
    routes: [
        {
            // 重定向, 不要写name
            // 可以保证第一次打开页面时所在的页面
            // 以及其他希望重定向的场景
            path: "/",
            redirect: "about"
        },
        {
            name: "home",
            path: "/home",
            component: Home
        },
        {
            name: "routers",
            path: "/routers",
            component: Routers,
            children: [
                {
                    name: "detail",
                    // params 形式的传参, ? 代表可选参数
                    component: Detail,
                    // 作用：让路由组件更方便的收到参数
                    // 第一种写法: 将路由收到的所有**params**参数作为props传给路由组件, 不能传query参数
                    path: "detail/:id/:title/:content?",
                    props: true,

                    // 第二种写法: 函数形式, 自定义props传参, 主要用于传query参数, params也可, 但不如上面简洁
                    // path: "detail", // 配合传query参数时使用
                    // props(route) {
                    //    console.log("query", route.query);
                    //    return route.query
                    // }
                    // path: "detail/:id/:title/:content?",// 配合传params参数时使用
                    // props(route) {
                    //    console.log("params", route.params);
                    //    return route.params
                    // }

                    // 第三种写法: 对象形式, 自定义props传参
                    // 但只能写死, 不灵活, 一般用于给路由组件固定传参的场景(因为路由组件没有机会自带参数)
                    // props {id: '123', title: 'abc', content: 'def'}
                }
            ]
        },
        {
            name: "about",
            path: "/about",
            component: About
        },
        {
            name: "reactivity",
            path: "/reactivity",
            component: Reactivity
        },
        {
            name: "tauri",
            path: "/tauri",
            component: Tauri
        },
        {
            name: "lifecycle",
            path: "/lifecycle",
            component: Lifecycle
        },
        {
            name: "diyhook",
            path: "/diyhook",
            component: DiyHook
        },
    ]
})

// 导出供外部使用
export default router
