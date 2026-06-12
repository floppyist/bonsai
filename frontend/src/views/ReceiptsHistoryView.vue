<script setup>
import { useBonsaiStore } from '../stores/BonsaiStore.js'

defineProps({
  formatPrice: {
    type: Function,
    default: (p) => `${Number(p).toFixed(2)} €`
  }
})

const store = useBonsaiStore()
</script>

<template>
  <div class="animate-[fadeIn_0.3s_ease-out]">
    <div class="mb-4">
      <h2 class="text-lg font-bold text-[#2D3A34]">Deine Belege</h2>
      <p class="text-xs text-[#6B7C74] font-medium">Historie aller gescannten Einkäufe</p>
    </div>

    <div v-if="store.receiptsHistory.length === 0" class="bg-white rounded-[20px] p-8 shadow-sm border border-black/5 text-center">
      <p class="text-sm text-[#6B7C74]">Noch keine Belege gescannt.</p>
    </div>

    <div v-else class="space-y-3">
      <div v-for="receipt in store.receiptsHistory" :key="receipt.id"
           class="bg-white rounded-[20px] p-4 shadow-sm border border-black/5 flex justify-between items-center">
        <div>
          <h3 class="text-sm font-semibold text-[#2D3A34]">{{ receipt.retailer }}</h3>
          <p class="text-[11px] text-[#6B7C74]">{{ receipt.date }} • {{ receipt.itemCount }} Artikel</p>
        </div>
        <span class="font-bold text-[#2D6A4F] text-sm">{{ formatPrice(receipt.totalAmount) }}</span>
      </div>
    </div>
  </div>
</template>
