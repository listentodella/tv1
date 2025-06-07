import { reactive, onMounted } from 'vue'
import axios, { AxiosError } from 'axios'


// 
export default function useDog() {
    // 泛型写法
    let dogList = reactive<string[]>([])

    // 方法
    async function getDog() {
        try {
            // 异步请求
            // {message: "https://xxx.jpg", status: "success"}
            let { data } = await axios.get('https://dog.ceo/api/breeds/image/random')
            console.log(data);

            // 维护数据
            dogList.push(data.message)
        } catch (error) {
            // 处理错误
            const err = <AxiosError>error
            console.log(err.message);
        }
    }

    // 挂载hook
    onMounted(() => {
        // 首次加载, 保证有数据
        getDog()
    })

    // 向外暴露数据
    return { dogList, getDog }
}