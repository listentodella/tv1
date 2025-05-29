<template>
    <ul class="news">
        <li> {{ route.params.id }}</li>
        <li> {{ route.params.title }}</li>
        <li> {{ route.params.content }} </li>
    </ul>
</template>

<script lang="ts" setup name="News">
// 接收参数
import { useRoute } from 'vue-router';
const route = useRoute();
console.log(route);

</script>
