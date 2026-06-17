<script setup>
import { computed } from 'vue'
import { useBonsaiStore } from '../stores/BonsaiStore.js'

defineProps({
  formatPrice: {
    type: Function,
    default: (p) => `${Number(p).toFixed(2)} €`
  }
})

const store = useBonsaiStore()

const receiptCount = computed(() => store.receiptsHistory.length)

const retailerBars = computed(() => {
  const groups = {}

  store.receiptsHistory.forEach(r => {
    const name = r.retailer || 'Unbekannt'
    groups[name] = (groups[name] || 0) + (r.totalAmount || 0)
  })

  const entries = Object.entries(groups)
  const max = Math.max(...entries.map(([, v]) => v), 1)

  return entries.map(([name, total]) => ({
    name,
    total,
    height: Math.max((total / max) * 100, 10) + '%'
  }))
})
</script>

<template>
  <div class="animate-[fadeIn_0.3s_ease-out]">
    <div class="mb-4">
      <h2 class="text-lg font-bold text-[#2D3A34]">Finanz- & KI Insights</h2>
    </div>

    <div class="bg-gradient-to-br from-white to-[#D8F3DC] p-4 rounded-[20px] mb-5 shadow-sm border border-black/5">
      <div class="flex justify-between items-center mb-2">
        <h3 class="text-sm font-bold text-[#2D6A4F]">KI-Schnittstelle (Ollama)</h3>
        <span class="text-[10px] bg-[#2D6A4F] text-white px-2 py-0.5 rounded-full">Qwen2.5-VL ready</span>
      </div>
      <p class="text-xs text-[#2D3A34] leading-relaxed">Dein lokales Modell hat diesen Monat {{ receiptCount }} Belege analysiert und automatisch kategorisiert. Keine Cloud, 100% privat.</p>
    </div>

    <div v-if="retailerBars.length === 0" class="bg-white rounded-[20px] p-8 shadow-sm border border-black/5 text-center">
      <p class="text-sm text-[#6B7C74]">Noch keine Ausgaben erfasst.</p>
    </div>

    <div v-else class="flex justify-around items-end h-40 bg-white rounded-[20px] p-5 shadow-sm border border-black/5">
      <div v-for="bar in retailerBars" :key="bar.name"
           class="w-11 bg-gradient-to-t from-[#2D6A4F] to-[#52B788] rounded-t-lg flex items-end justify-center pb-2 transition-all duration-500 min-h-[24px]"
           :style="{ height: bar.height }">
        <span class="text-[9px] text-white [writing-mode:vertical-lr] rotate-180">{{ bar.name }}</span>
      </div>
    </div>
  </div>
</template>
