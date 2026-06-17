<script setup>
import { ref } from 'vue'

defineProps({
  store: String,
  date: String,
  isStoreInvalid: Boolean,
  isDateInvalid: Boolean
})

const emit = defineEmits(['update:store', 'update:date'])

const commonStores = [
  'REWE', 'Edeka', 'Kaufland', 'Globus', 'hit', 'Tegut',
  'Aldi Süd', 'Aldi Nord', 'Lidl', 'Netto', 'Penny', 'Norma',
  'dm', 'Rossmann', 'Müller', 'Denns BioMarkt', 'Alnatura', 'Bio-Markt',
  'OBI', 'Bauhaus', 'Hornbach', 'Toom', 'Toom Baumarkt',
  'MediaMarkt', 'Saturn', 'Thalia',
  'Action', 'TEDi', 'Decathlon', 'IKEA'
].sort((a, b) => a.localeCompare(b, 'de'))

const showStoreDropdown = ref(false)

const selectStore = (name) => {
  emit('update:store', name)
  showStoreDropdown.value = false
}
</script>

<template>
  <div class="bg-white rounded-[24px] p-5 shadow-sm border border-black/5 space-y-4">
    <div class="relative">
      <label class="text-[10px] font-bold text-[#6B7C74] uppercase tracking-wider block mb-1">Händler</label>
      <div class="flex gap-2">
        <button
          type="button"
          @click="showStoreDropdown = !showStoreDropdown"
          :class="[
            'flex-1 text-left rounded-xl px-4 py-3 text-sm font-bold flex justify-between items-center cursor-pointer min-h-[48px] transition-all duration-200 border',
            isStoreInvalid
              ? 'bg-amber-50/50 border-amber-500/40 text-amber-900 shadow-sm shadow-amber-50'
              : 'bg-[#F4F7F5] border-zinc-200/60 text-[#2D3A34]'
          ]"
        >
          <span>{{ store || 'Bitte wählen...' }}</span>
          <span class="text-xs opacity-50">▼</span>
        </button>
        <input
          :value="store"
          @input="emit('update:store', $event.target.value)"
          placeholder="Oder tippen..."
          :class="[
            'w-1/3 rounded-xl px-3 py-3 text-xs focus:outline-none min-h-[48px] transition-all duration-200 border',
            isStoreInvalid
              ? 'bg-amber-50/50 border-amber-500/40 text-amber-900 focus:border-amber-500'
              : 'bg-[#F4F7F5] border-zinc-200/60 text-[#2D3A34] focus:border-[#2D6A4F]'
          ]"
        />
      </div>

      <div v-if="showStoreDropdown" class="absolute left-0 right-0 mt-1 bg-white border border-zinc-100 rounded-xl shadow-xl z-10 p-2 grid grid-cols-2 gap-1 animate-[fadeIn_0.15s_ease-out]">
        <button
          v-for="item in commonStores"
          :key="item"
          type="button"
          @click="selectStore(item)"
          class="text-left text-xs font-semibold py-2.5 px-3 rounded-lg hover:bg-[#F4F7F5] hover:text-[#2D6A4F] text-[#2D3A34] transition-colors min-h-[40px]"
        >
          {{ item }}
        </button>
      </div>
    </div>

    <div class="w-full block">
      <label class="text-[10px] font-bold text-[#6B7C74] uppercase tracking-wider block mb-1">Belegdatum</label>
      <div class="w-full relative block">
        <input
          type="date"
          :value="date"
          @input="emit('update:date', $event.target.value)"
          :class="[
            'w-full max-w-full box-border rounded-xl px-4 py-3 text-sm font-bold focus:outline-none min-h-[48px] appearance-none transition-all duration-200 border',
            isDateInvalid
              ? 'bg-amber-50/50 border-amber-500/40 text-amber-900 focus:border-amber-500 shadow-sm shadow-amber-50'
              : 'bg-[#F4F7F5] border-zinc-200/60 text-[#2D3A34] focus:border-[#2D6A4F]'
          ]"
        />
      </div>
    </div>
  </div>
</template>
