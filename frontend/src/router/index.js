import { createRouter, createWebHistory } from 'vue-router'

import PantryView from '../views/PantryView.vue'
import BudgetView from '@/views/BudgetView.vue'
import ReceiptView from '@/views/ReceiptView.vue'
import ReceiptsHistoryView from '../views/ReceiptsHistoryView.vue'
import SettingsView from '@/views/SettingsView.vue'

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
      component: BudgetView,
    },
    {
      path: '/receipt',
      name: 'receipt',
      component: ReceiptView,
    },
    {
      path: '/receipts',
      name: 'receipts',
      component: ReceiptsHistoryView,
    },
    {
      path: '/settings',
      name: 'settings',
      component: SettingsView,
    },
  ],
})

export default router
