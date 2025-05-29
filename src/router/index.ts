// 1. 导入router
import { createRouter, createWebHistory } from "vue-router"
// import Home from "@/pages/Home.vue"
// import News from "@/pages/News.vue"
// import About from "@/pages/About.vue"
import Home from "../pages/Home.vue"
import News from "../pages/News.vue"
import About from "../pages/About.vue"
import Detail from "../pages/Detail.vue"
import Person from "../pages/Person.vue"


// 2. 创建 router
const router = createRouter({
    // createWebHistory 适用于大多数场景, 地址栏不会有 # 符号, 但是后端一定要
    history: createWebHistory(),
    // 有哪些路由规则
    routes: [
        {
            name: "home",
            path: "/home",
            component: Home
        },
        {
            name: "news",
            path: "/news",
            component: News,
            children: [
                {
                    name: "detail",
                    // params 形式的传参, ? 代表可选参数
                    path: "detail/:id/:title/:content?",
                    component: Detail
                }
            ]
        },
        {
            name: "about",
            path: "/about",
            component: About
        },
        {
            name: "person",
            path: "/person",
            component: Person
        }
    ]
})

// 导出供外部使用
export default router
