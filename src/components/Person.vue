<template>
    <div class="person">
        <h3>Car Info: 1 {{ car.brand }} - ${{ car.price }}w</h3>
        <button @click="changeBrand">修改汽车的品牌</button>
        <button @click="changePrice">修改汽车的价格</button>
        <button @click="changeCar">修改汽车</button>
        <hr>
        <h3>Sum = {{ sum }}</h3>
        <button @click="changeSum">点我sum+1</button>
    </div>
</template>


<!-- <script lang="ts">
// 新版本的vue3已经不需要了
export default {
    name: "Person"
};
</script> -->

<script setup lang="ts" setup_name="Person">
import { ref, reactive } from "vue";
/* ref 与 reactive 
区别:
    - ref 创建的变量必须使用 .value 访问, 但是可以完整替换(可以使用 vue official 插件auto-insert自动添加 .value)
    - reactive 只能部分更新, 如果重新分配一个完整的新对象, 会失去响应式特性(只能靠Object.assign来完全更新)
原则:
    - 若需要一个基本类型的响应式数据, 必须使用 ref
    - 若需要一个响应式对象, 层级不深, ref, reactive 都可以
    - 若需要一个响应式对象, 层级较深, 建议使用 reactive
*/

// let car = ref({ brand: "AITO", price: 100 })
let car = reactive({ brand: "AITO", price: 100 })
let sum = ref(0)

function changeCar() {
    // console.log("total change car var will lose reactive :(");
    // car = { brand: "BYD", price: 200 }// 失去响应式
    // car = reactive({ brand: "BYD", price: 200 })// 是一个全新的reactive对象,但页面不会更新
    car = Object.assign(car, { brand: "BYD", price: 200 })// 页面可以更新
}

function changePrice() {
    // for ref version
    // car.value.price += 10
    // console.log("car price changed to", car.value.price)
    // for reactive version
    car.price += 10
    console.log("car price changed to", car.price)
}

function changeBrand() {
    car.brand = "XIAOMI"
    console.log("car brand changed to", car.brand)
}

function changeSum() {
    sum.value += 1
    console.log("sum changed to", sum.value)
}

</script>

<style scoped>
    .person {
        /* background-color: #ddd; */
        background-color: skyblue;
        box-shadow: 0 0 10px;
        border-radius: 10px;
        padding: 20px;
    }
</style>