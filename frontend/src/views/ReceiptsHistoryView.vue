<script setup>
import { ref, onMounted } from 'vue'
import { useBonsaiStore } from '../stores/BonsaiStore.js'

defineProps({
  formatPrice: {
    type: Function,
    default: (p) => `${Number(p).toFixed(2)} €`
  }
})

const store = useBonsaiStore()

const expandedId = ref(null)

const toggleReceipt = async (receiptId) => {
  if (expandedId.value === receiptId) {
    expandedId.value = null
    return
  }

  expandedId.value = receiptId
  await store.fetchReceiptDetails(receiptId)
}

const categoryLabel = (value) => {
  return store.categoriesList.find(c => c.value === value)?.label || '❓ Unkategorisiert'
}

onMounted(() => {
  store.fetchReceiptsHistory()
})
</script>

<template>
  <div class="animate-[fadeIn_0.3s_ease-out]">
    <div class="mb-4">
      <h2 class="text-lg font-bold text-[#2D3A34]">Deine Belege</h2>
      <p class="text-xs text-[#6B7C74] font-medium">Historie aller gescannten Einkäufe</p>
    </div>

    <div v-if="store.isLoadingReceipts && store.receiptsHistory.length === 0" class="bg-white rounded-[20px] p-8 shadow-sm border border-black/5 text-center">
      <p class="text-sm text-[#6B7C74]">Belege werden geladen…</p>
    </div>

    <div v-else-if="store.receiptsHistory.length === 0" class="bg-white rounded-[20px] p-8 shadow-sm border border-black/5 text-center">
      <p class="text-sm text-[#6B7C74]">Noch keine Belege gescannt.</p>
    </div>

    <div v-else class="space-y-3">
      <div v-for="receipt in store.receiptsHistory" :key="receipt.id"
           class="bg-white rounded-[20px] shadow-sm border border-black/5 overflow-hidden transition-all"
           :class="{ 'border-[#2D6A4F]/20 shadow-md': expandedId === receipt.id }">

        <button type="button" @click="toggleReceipt(receipt.id)"
                class="w-full p-4 flex justify-between items-center text-left cursor-pointer hover:bg-[#F4F7F5]/40 transition-colors">
          <div class="flex items-center gap-3 min-w-0">
            <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 24 24" fill="none"
                 stroke="currentColor" stroke-width="3" stroke-linecap="round" stroke-linejoin="round"
                 class="text-[#6B7C74] shrink-0 transition-transform duration-200"
                 :class="{ 'rotate-90': expandedId === receipt.id }">
              <polyline points="9 18 15 12 9 6" />
            </svg>
            <div class="min-w-0">
              <h3 class="text-sm font-semibold text-[#2D3A34] truncate">{{ receipt.retailer }}</h3>
              <p class="text-[11px] text-[#6B7C74]">{{ receipt.date }} • {{ receipt.itemCount }} Artikel</p>
            </div>
          </div>
          <span class="font-bold text-[#2D6A4F] text-sm shrink-0 ml-2">{{ formatPrice(receipt.totalAmount) }}</span>
        </button>

        <div v-if="expandedId === receipt.id" class="border-t border-zinc-100 px-4 py-3 bg-[#F4F7F5]/30 animate-[fadeIn_0.2s_ease-out]">
          <div v-if="store.loadingReceiptDetailId === receipt.id && !store.receiptDetails[receipt.id]"
               class="text-xs text-[#6B7C74] py-2 text-center">
            Artikel werden geladen…
          </div>

          <div v-else-if="store.receiptDetails[receipt.id] && store.receiptDetails[receipt.id].length > 0" class="space-y-2">
            <div v-for="(item, idx) in store.receiptDetails[receipt.id]" :key="idx"
                 class="flex items-start justify-between gap-3 bg-white rounded-xl p-3 border border-zinc-100">
              <div class="min-w-0 flex-1">
                <div class="text-xs font-bold text-[#2D3A34] truncate flex items-center gap-1.5 flex-wrap">
                  <span>{{ item.name }}</span>
                  <span v-if="item.brand && item.brand !== 'UNKNOWN'"
                        class="text-[10px] text-[#6B7C74] font-medium bg-zinc-100 px-1.5 py-0.5 rounded border border-zinc-200/30">
                    {{ item.brand }}
                  </span>
                </div>
                <div class="text-[10px] font-medium mt-0.5 flex flex-wrap items-center gap-1.5 text-[#6B7C74]">
                  <span>{{ item.quantity }}x · {{ Number(item.price).toFixed(2) }} €</span>
                  <span class="bg-[#F4F7F5] text-[#2D6A4F] px-1.5 py-0.5 rounded text-[9px] font-bold border border-zinc-200/30">
                    {{ categoryLabel(item.category) }}
                  </span>
                  <span v-if="item.ean" class="font-mono font-bold text-[#2D6A4F]">| EAN: {{ item.ean }}</span>
                </div>
              </div>
              <span class="text-xs font-mono font-bold text-[#2D3A34] shrink-0">
                {{ formatPrice(Number(item.price || 0) * (item.quantity || 1)) }}
              </span>
            </div>
          </div>

          <div v-else class="text-xs text-[#6B7C74] py-2 text-center">
            Keine Artikel zu diesem Beleg gefunden.
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
