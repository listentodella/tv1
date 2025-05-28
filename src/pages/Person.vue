<template>
    <div class="person">
        <h3>Car Info: 1 {{ car.brand }} - ${{ car.price }}w</h3>
        <button @click="changeBrand">修改汽车的品牌</button>
        <button @click="changePrice">修改汽车的价格</button>
        <button @click="changeCar">修改汽车</button>
        <hr>
        <h3>Sum = {{ sum }}</h3>
        <button @click="changeSum">点我sum+1</button>
        <hr>
        <h3>Person: {{ name }} - {{ age }}</h3>
        <button @click="changeName">修改名字</button>
        <button @click="changeAge">修改年龄</button>
        <hr>
        <!-- v-model 双向绑定, 变量值可以显示到页面上, 页面上的修改也可以回到变量  -->
        姓: <input type="text" v-model="firstName" /> <br>
        名: <input type="text" v-model="lastName" /> <br>
        <!-- 全名: <span>{{ firstName }} {{ lastName }}</span> <br> -->
        <!-- 函数不会缓存, 每次调用都是重新计算 -->
        全名func: <span>{{ fullNameFunc() }}</span> <br>
        全名func: <span>{{ fullNameFunc() }}</span> <br>
        <!-- 计算属性有缓存, 只有当依赖的数据发生变化时, 才会重新计算-->
        全名computed: <span>{{ fullName }}</span> <br>
        全名computed: <span>{{ fullName }}</span> <br>
        <button @click="changeFullName">修改全名</button>
    </div>
</template>


<!-- <script lang="ts">
// 新版本的vue3已经不需要了
export default {
    name: "Person"
};
</script> -->

<script setup lang="ts" setup_name="Person">
import { ref, reactive, toRefs, computed } from "vue";
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

let person = reactive({
    name: "zhangsan",
    age: 18
})
// 只有通过 toRefs() 解构, 才能获取到响应式数据
// 每个成员都相当于用 ref() 包裹了一下, 而且源数据也会更新
// 否则只是相当于定义了一个全新的普通变量, 页面上不会更新
// toRef() 相当于阉割版的 toRefs(), 只能一个个成员解构
let {name, age } = toRefs(person)
function changeName() {
    name.value += "~"
    console.log(name.value, person.name);
}
function changeAge() {
    age.value += 1
    console.log(age.value, person.age);
}

let firstName = ref("zhang")
let lastName = ref("san")

function fullNameFunc() {
    console.log("fullNameFunc called");
    return firstName.value.slice(0, 1).toUpperCase() + firstName.value.slice(1) + "-" + lastName.value
}

// 不通过get/set, 则FullName 是只读的, 不能修改
// let fullName = computed(() => {
let fullName = computed( {
    get() {
        console.log("computed called");
        return firstName.value.slice(0, 1).toUpperCase() + firstName.value.slice(1) + "-" + lastName.value
    },
    set(val) {
        console.log("computed set called", val);
        const [str1, str2] = val.split("-")
        firstName.value = str1
        lastName.value = str2
    }
})

function changeFullName() {
    // 这将导致对应的set被调用
    fullName.value = "li-si"
}

</script>

<style scoped>
    .person {
        /* background-color: #ddd; */
        background-color: skyblue;
        box-shadow: 0 0 10px;
        border-radius: 10px;
        padding: 20px;
        /* margin-trim: 5px; */
        /* margin-inline: 5px; */
    }
</style>