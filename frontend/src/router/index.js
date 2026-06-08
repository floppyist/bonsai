import { createRouter, createWebHistory } from 'vue-router'

import PantryView from '../views/PantryView.vue'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'pantry',
      component: PantryView,
    },
    {
      path: '/budget',
      name: 'budget',
      component: () => import('../views/BudgetView.vue'), // Lazy Loading
    },
    {
      path: '/receipt',
      name: 'receipt',
      component: () => import('../views/ReceiptView.vue'), // Lazy Loading
    },
  ],
})

export default router
