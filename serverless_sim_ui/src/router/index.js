import Vue from 'vue'
import Router from 'vue-router'
import NetworkTopology from '@/views/NetworkTopology.vue'

Vue.use(Router)

export default new Router({
    mode: 'history',
    base: process.env.BASE_URL,
    routes: [
        // 其他路由配置
        {
            path: '/network-topology',
            name: 'NetworkTopology',
            component: NetworkTopology
        }
    ]
})

