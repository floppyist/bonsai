import { createRouter, createWebHistory } from 'vue-router'
import ReceiptScanner from '../components/ReceiptScanner.vue' // Deine neue Komponente

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'home',
      component: ReceiptScanner
    },
  ]
})

export default router
