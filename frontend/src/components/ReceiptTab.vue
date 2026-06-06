<script setup>
defineProps({
  receiptData: Object,
  formatPrice: Function
})

defineEmits(['startEanScanner', 'saveAndCloseReceipt'])
</script>

<template>
  <div class="animate-[fadeIn_0.3s_ease-out]">
    <div class="mb-4">
      <h2 class="text-lg font-bold text-[#2D3A34]">Neuer Beleg</h2>
      <p class="text-xs text-[#6B7C74] font-medium">{{ receiptData.retailer?.value || 'Unbekannter Händler' }} • {{ receiptData.date?.value || 'Datum fehlt' }}</p>
    </div>

    <div class="bg-white rounded-[20px] p-4 shadow-sm border border-black/5 mb-5">
      <div class="space-y-3">
        <div v-for="(item, index) in receiptData.articles" :key="index" class="flex justify-between items-start border-b border-zinc-100 pb-3 last:border-0 last:pb-0">
          <div class="flex-1 pr-3">
            <div class="text-sm font-semibold text-[#2D3A34] mb-1">
              {{ item.quantity }}x {{ item.name }}
            </div>

            <div class="flex items-center">
              <span v-if="item.ean" class="text-[10px] font-mono bg-zinc-100 text-zinc-500 px-1.5 py-0.5 rounded animate-[fadeIn_0.2s_ease-out]">
                EAN: {{ item.ean }}
              </span>

              <button v-else @click="$emit('startEanScanner', index)" class="bg-[#CCFF33] hover:bg-[#bceb2e] text-[#1B4332] border border-[#B7E4C7] px-2 py-1 rounded flex items-center gap-1 text-[10px] font-bold shadow-sm active:scale-95 transition-all">
                <svg xmlns="http://www.w3.org/2000/svg" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M3 7V5a2 2 0 0 1 2-2h2"/><path d="M17 3h2a2 2 0 0 1 2 2v2"/><path d="M21 17v2a2 2 0 0 1-2 2h-2"/><path d="M7 21H5a2 2 0 0 1-2-2v-2"/><rect x="7" y="7" width="10" height="10" rx="1"/></svg>
                EAN scannen
              </button>
            </div>
          </div>
          <div class="text-sm font-bold text-[#2D3A34] pt-0.5">
            {{ formatPrice(item.price_total) }}
          </div>
        </div>
      </div>

      <div class="mt-4 pt-3 border-t-2 border-dashed border-zinc-200 flex justify-between items-center">
        <span class="font-bold text-[#2D3A34] text-sm">Gesamtsumme</span>
        <span class="font-black text-[#2D6A4F] text-lg">{{ formatPrice(receiptData.total_amount?.value || 0) }}</span>
      </div>
    </div>

    <button @click="$emit('saveAndCloseReceipt')" class="w-full bg-[#2D6A4F] text-white font-bold py-3.5 rounded-[16px] shadow-md active:scale-[0.98] transition-transform">
      In Vorratskammer übernehmen
    </button>
  </div>
</template>
