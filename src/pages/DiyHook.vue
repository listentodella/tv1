<template>
    <div class="diyhook">
        <h1>DIY Hook</h1>
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