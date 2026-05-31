import { createRouter, createWebHistory } from 'vue-router'
import HomeView from '../views/HomeView.vue' // Falls vorhanden
import ReceiptScanner from '../components/ReceiptScanner.vue' // Deine neue Komponente

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'home',
      component: HomeView
    },
    {
      path: '/scan',
      name: 'scan',
      component: ReceiptScanner // Ruft das vorherige Skript auf
    },
    {
      path: '/about',
      name: 'about',
      component: () => import('../views/AboutView.vue')
    }
  ]
})

export default router
