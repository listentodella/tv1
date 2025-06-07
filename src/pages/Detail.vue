<template>
    <ul class="news">
        <li> {{ id }}</li>
        <li> {{ title }}</li>
        <li> {{ content }} </li>
    </ul>
</template>

<script lang="ts" setup name="News">
// 如果不适用props的话, 只能通过如下的方式获取参数, 然后解构出来
// import { useRoute } from 'vue-router';
// const route = useRoute();
// console.log(route);

// 通过props 直接获取参数
defineProps(['id', 'title', 'content'])

</script>
