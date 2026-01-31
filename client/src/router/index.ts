import { createRouter, createWebHistory } from 'vue-router'
import HomeView from '@/views/HomeView.vue'

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: '/',
      name: 'home',
      component: HomeView,
    },
    {
      path: '/send',
      name: 'send',
      component: () => import('@/views/SendView.vue'),
    },
    {
      path: '/receive',
      name: 'receive',
      component: () => import('@/views/ReceiveView.vue'),
    },
  ],
})

export default router
