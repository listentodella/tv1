<template>
    <div class="diyhook">
        <h1>DIY Hook</h1>
        <hr>
        <p>hook本质是一个函数, 把setup函数中使用的Composition API进行了封装</p>
        <p>自定义hook的优势: 复用代码,让setup中的逻辑更清晰</p>

        <h2>当前计数: {{ sum }} x1000 = {{ bigSum }}</h2>
        <button @click="increment()">点我+1</button>
        <button @click="decrement()">点我-1</button>
        <hr>
        <button @click="getDog()">点我获取狗狗</button><br>
        <img v-for="(u, index) in dogList" :key="index" :src="(u as string)">
    </div>
</template>

<script lang="ts" setup name="DiyHook">
import useSum from '../hooks/useSum'
import useDog from '../hooks/useDog';

// 解构出组件对外提供的变量或方法
const { sum, bigSum, increment, decrement } = useSum()
const { dogList, getDog } = useDog()

</script>


<style scoped>
img {
    height: 100px;
    margin-right: 10px;
}
</style>