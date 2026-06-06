<script setup>
defineProps({
  pantryItems: Array
})
</script>

<template>
  <div class="animate-[fadeIn_0.3s_ease-out]">
    <div class="flex justify-between items-center mb-4">
      <h2 class="text-lg font-bold text-[#2D3A34]">Deine Vorratskammer</h2>
      <span class="text-xs bg-[#B7E4C7] text-[#2D6A4F] px-2.5 py-1 rounded-xl font-semibold">Alles (24)</span>
    </div>

    <div class="grid grid-cols-2 gap-3.5">
      <div v-for="item in pantryItems" :key="item.id" class="bg-white rounded-[20px] p-3.5 shadow-sm border border-black/5 active:scale-[0.96] transition-transform duration-200">
        <div class="flex justify-between items-start mb-2.5">
          <span class="text-3xl">{{ item.emoji }}</span>
          <span :class="[
            'text-[10px] px-2 py-0.5 rounded-lg font-bold',
            item.status === 'urgent' ? 'bg-red-50 text-red-500' : '',
            item.status === 'good' ? 'bg-emerald-50 text-emerald-700' : '',
            item.status === 'warn' ? 'bg-amber-50 text-amber-700' : '',
            item.status === 'infinite' ? 'bg-blue-50 text-blue-600' : ''
          ]">
            {{ item.daysLeft }}
          </span>
        </div>
        <h3 class="text-sm font-semibold text-[#2D3A34] truncate mb-0.5">{{ item.name }}</h3>
        <p class="text-[11px] text-[#6B7C74] mb-3">{{ item.category }}</p>

        <div class="h-1.5 bg-zinc-100 rounded-full overflow-hidden mb-1">
          <div class="h-full rounded-full transition-all duration-500 ease-out" :style="{ width: item.stock + '%', backgroundColor: item.stockColor }"></div>
        </div>
        <div class="flex justify-between text-[10px] text-[#6B7C74] font-medium">
          <span>Bestand</span>
          <span>{{ item.stock }}%</span>
        </div>
      </div>
    </div>
  </div>
</template>
