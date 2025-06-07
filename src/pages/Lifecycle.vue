<template>
    <div class="lifecycle">
        <h1>Lifecycle</h1>
        <h2>当前计数: {{ sum }}</h2>
        <button @click="changeSum">点我+1</button>
    </div>

    <body>
        <p>vue3生命周期整体分为四个阶段:创建、挂载、更新、销毁,每个阶段都有两个钩子,一前一后</p>
        <b>创建阶段: setup </b> <br />
        <b>挂载阶段: onBeforeMount、onMounted </b><br />
        <b>更新阶段: onBeforeUpdate、onUpdated </b><br />
        <b>卸载阶段: onBeforeUnmount、onUnmounted </b><br />
        <b>常用的hooks: onMounted、onUpdated、onBeforeUnmount </b>
    </body>
</template>

<script lang="ts" setup name="Lifecycle">
import {
    ref,
    onBeforeMount,
    onMounted,
    onBeforeUpdate,
    onUpdated,
    onBeforeUnmount,
    onUnmounted
} from 'vue'

// 数据
let sum = ref(0)
// 方法
function changeSum() {
    sum.value += 1
}
console.log('setup')
// 生命周期钩子
onBeforeMount(() => {
    console.log('进入页面——>挂载之前')
})
onMounted(() => {
    console.log('进入页面——>挂载完毕')
})
onBeforeUpdate(() => {
    console.log('页面变化——>更新之前')
})
onUpdated(() => {
    console.log('页面变化——>更新完毕')
})
onBeforeUnmount(() => {
    console.log('离开页面——>卸载之前')
})
onUnmounted(() => {
    console.log('离开页面——>卸载完毕')
})
</script>