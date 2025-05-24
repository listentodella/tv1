<template>
    <div class="person">
        <h3>Car Info: 1 {{ car.brand }} - ${{ car.price }}w</h3>
        <h3>Total: {{ sum }}</h3>
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

let car = ref({ brand: "AITO", price: 100 })
let sum = ref(0)

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