<template>
    <div class="news">
        <h1>News</h1>
        <!-- 导航区 -->
        <ul>
            <li v-for="news in newsList" :key="news.id">
                <!-- <RouterLink to="/news/detail">{{ news.title }}</RouterLink> -->
                <!-- 命名路由逐渐展现优势, 不需要手动去拼路径名称 -->
                <RouterLink :to="{ name: 'detail', params: { id: news.id, title: news.title, content: news.content } }">
                    {{ news.title }}</RouterLink>
            </li>
        </ul>
        <!-- 展示区 -->
        <div class="news-content">
            <RouterView></RouterView>
        </div>
    </div>
</template>

<script lang="ts" setup name="News">
import { reactive } from 'vue'
import { RouterLink, RouterView } from 'vue-router';
const newsList = reactive([
    {
        id: "qwer01",
        title: 'News001',
        content: "This is the content of News001"
    },
    {
        id: "qwer02",
        title: 'News002',
        content: "This is the content of News002"
    },
    {
        id: "qwer03",
        title: 'News003',
        content: "This is the content of News003"
    },
    {
        id: "qwer04",
        title: 'News004',
        content: "This is the content of News004"
    },
    {
        id: "qwer05",
        title: 'News005 has no Content',
    },
])

</script>
