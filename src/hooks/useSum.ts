import { ref, onMounted, computed } from 'vue'


export default function () {
    let sum = ref(0)

    let bigSum = computed(() => {
        return sum.value * 1000
    })

    const increment = () => {
        sum.value += 1
    }

    const decrement = () => {
        sum.value -= 1
    }

    onMounted(() => {
        console.log("useSum mounted")
        increment()
    })

    // 向外暴露数据
    return { sum, bigSum, increment, decrement }
}