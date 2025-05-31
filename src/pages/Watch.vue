<template>
    <div class="watch">
        <h1>Watch: 监视数据的变化</h1>
        <p>vue3中只能监视以下四种数据:</p>
    </div>
    <b> 1. ref 定义的数据 </b><br>
    <b> 2. reactive 定义的数据 </b><br>
    <b> 3. 函数返回一个值(getter函数)</b><br>
    <b> 4. 一个包含上述内容的数组 </b>
    <hr>
    <h2> 当前计数: {{ sum }} </h2>
    <button @click="changeSum">点我+1</button>
    <hr>
    <h2>姓名: {{ person.name }} 年龄: {{ person.age }}</h2>
    <button @click="changeName">修改名字</button>
    <button @click="changeAge">修改年龄</button>
    <button @click="chanegPerson">修改整个人</button>
    <hr>
    <h2>Obj: 修改obj.a.b.c {{ obj.a.b.c }}</h2>
    <button @click="changeObj">修改Obj</button>
    <button @click="changeWholeObj">修改整个Obj</button>
    <hr>
    <h2>Man: 姓名: {{ man.name }} 年龄: {{ man.age }} 车: {{ man.car.c1 }} {{ man.car.c2 }}</h2>
    <button @click="changeName2">修改姓名</button>
    <button @click="changeAge2">修改年龄</button>
    <button @click="changeCar1">修改车1</button>
    <button @click="changeCar2">修改车2</button>
    <button @click="changeWholeCar">修改所有车</button>
</template>

<script lang="ts" setup name="Watch">
import { ref, reactive, watch } from 'vue'
let sum = ref(0)
function changeSum() {
    sum.value += 1
}
// 监视, 情况1: ref 定义的 基本类型的数据
const stopWatch = watch(sum, (newVal, oldVal) => {
    console.log("监测到 sum 变化:", newVal, "-->", oldVal);
    if (newVal >= 10) {
        console.log("sum 已经大于等于 10, 停止监测");
        stopWatch()
    }
})

// 监视, 情况2: ref 定义的 对象类型的数据，直接写数据名
// 监视的是对象的**地址值**，若想监视对象内部的数据，要手动开启深度监视
let person = ref({ name: "zhangsan", age: 18 })
function changeName() {
    person.value.name += "~"
}
function changeAge() {
    person.value.age += 1
}
function chanegPerson() {
    person.value = { name: "lisi", age: 20 }
}

// 如果想监视person.value, 只能通过 deep 参数
// 否则只有整个person变化, 才能触发监视
watch(person, (newVal, oldVal) => {
    console.log("监测到 person 变化:", newVal, "-->", oldVal);
}, { deep: true, immediate: true })


// 监视, 情况3: reactivie 定义的 对象类型的数据, 默认强制开启深度监视, 不可关闭
// 无法监视地址值, 因为reactive 定义的对象地址值不会发生改变,
// newValue 和 oldValue 值相同, 都是新值, 它们指向的是同一个地址
// assign 看起来整体更新了对象被监测到, 但本质是逐个更新每个成员, 因此被监测到
let obj = reactive({
    a: { b: { c: 666 } }
})

function changeObj() {
    obj.a.b.c += 1
}

function changeWholeObj() {
    // obj = { a: { b: { c: 777 } } }// 直接复制, 地址值改变, 监测不到
    // obj = reactive({ a: { b: { c: 888 } } }) // reactive 定义的对象覆盖, 地址值改变, 监测不到
    Object.assign(obj, { a: { b: { c: 999 } } })// 本质是逐个更新每个成员, 地址值不变, 因此被监测到
}

// 无需手动开启deep
watch(obj, (newVal, oldVal) => {
    console.log("监测到 obj 变化:", newVal, "-->", oldVal);
})


// 监视, 情况4: 监视 ref / reactive 定义的 对象类型 中的某个属性
// 若该属性值 是基本类型, 则需要写成函数的形式, 此时 oldValue 是旧值, newValue 是新值
// 若该属性值 是对象类型, 可以直接写, 也可以写成函数的形式, 建议写成函数
// 如果对象监视的是地址值, 需要关注对象内部, 需要手动开启deep
let man = reactive({
    name: "zhangsan",
    age: 18,
    car: {
        c1: "BMW",
        c2: "Audi"
    }
})
function changeName2() {
    man.name += "~"
}
function changeAge2() {
    man.age += 1
}
function changeCar1() {
    man.car.c1 = "TSL"
}
function changeCar2() {
    man.car.c2 = "WXL"
}
function changeWholeCar() {
    man.car = { c1: "AITO", c2: "BYD" }
}

watch(() => man.name, (newVal, oldVal) => {
    console.log("监测到 man.name 变化:", newVal, "-->", oldVal);
})

// 直接写, 只能监测到部分变化
// 整体改变时, 对象地址变化了, 所以监视不到
watch(man.car, (newVal, oldVal) => {
    console.log("direct 监测到 man.car 部分变化:", newVal, "-->", oldVal);
}, { deep: true })

// 函数返回值监视的是对象的地址值
// 函数形式（不开启深度监视, 只能监测到整体变化
// 不开启deep时, 监视的是对象的地址值, 只有整体变化才会更新地址值, 并且新值是新值, 旧值是旧值, 但是对象内部属性新旧值都是新值...
watch(() => man.car, (newVal, oldVal) => {
    console.log("监测到 man.car 完全变化:", newVal, "-->", oldVal);
})

// 函数形式（开启深度监视, 所有变化都能监测
watch(() => man.car, (newVal, oldVal) => {
    console.log("func 监测到 man.car 所有变化:", newVal, "-->", oldVal);
}, { deep: true })


// 监视, 情况5: 监视上述的多个数据(数组形式)
watch([() => man.name, man.car], (newVal, oldVal) => {
    console.log("========name or car 变化了", newVal, "-->", oldVal);
}, { deep: true })


</script>