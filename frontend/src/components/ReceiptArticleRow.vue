<script setup>
import { ref } from 'vue'

const props = defineProps({
  item: Object,
  index: Number,
  isActive: Boolean,
  isDuplicate: Boolean,
  categoriesList: Array,
  commonProductSuggestions: Array,
  formatPrice: Function
})

const emit = defineEmits([
  'toggleEdit', 'updateQuantity', 'updatePrice', 'remove',
  'rename', 'manuallyEnterEan', 'startEanScanner'
])

const nameValidationMsg = ref('✨ Format optimiert')
const brandValidationMsg = ref('✨ Format optimiert')

const ALLOWED_NAME_REGEX = /[^A-Z0-9 \-&.,%]/g;
const ALLOWED_BRAND_REGEX = /[^A-Z0-9 \-&%]/g;

const handleLiveSanitize = (field, originalValue) => {
  let transformed = originalValue
    .toUpperCase()
    .replace(/Ä/g, 'AE')
    .replace(/Ö/g, 'OE')
    .replace(/Ü/g, 'UE')
    .replace(/ß/g, 'SS');

  const regex = field === 'name' ? ALLOWED_NAME_REGEX : ALLOWED_BRAND_REGEX;

  if (regex.test(transformed)) {
    if (field === 'name') nameValidationMsg.value = '⚠️ Unerlaubte Sonderzeichen entfernt';
    if (field === 'brand') brandValidationMsg.value = '⚠️ Unerlaubte Sonderzeichen entfernt';
    transformed = transformed.replace(regex, '');
  } else {
    if (field === 'name') nameValidationMsg.value = '✨ Perfekt formatiert';
    if (field === 'brand') brandValidationMsg.value = '✨ Perfekt formatiert';
  }

  // Direktes Schreiben in das reaktive Prop-Objekt, das vom Store kommt
  props.item[field] = transformed
}

const handleBlurCleanup = (field, currentValue) => {
  props.item[field] = currentValue.replace(/\s+/g, ' ').trim();
}
</script>

<template>
  <div
    :class="[
      'rounded-xl transition-all border duration-200',
      isActive
        ? (isDuplicate ? 'bg-amber-50/60 border-amber-500/40 p-4 space-y-3.5 shadow-sm' : 'bg-[#F4F7F5] border-[#2D6A4F]/20 p-4 space-y-3.5')
        : (isDuplicate ? 'bg-amber-50/40 border-amber-500/25 hover:bg-amber-50/70 p-3 flex items-center justify-between gap-3 text-amber-950' :
          (item.ean ? 'bg-[#EAF5ED]/60 border-[#2D6A4F]/15 hover:bg-[#EAF5ED]/90 p-3 flex items-center justify-between gap-3' : 'bg-white hover:bg-[#F4F7F5]/30 border-zinc-100 p-3 flex items-center justify-between gap-3'))
    ]"
  >
    <template v-if="isActive">
      <div class="flex justify-between items-start gap-2">
        <div class="flex-1 space-y-2">
          <div class="grid grid-cols-1 sm:grid-cols-2 gap-2">
            <div>
              <label class="text-[9px] font-bold text-[#6B7C74] uppercase tracking-wider block mb-0.5">Artikelbezeichnung</label>
              <input
                v-model="item.name"
                @input="handleLiveSanitize('name', $event.target.value)"
                @blur="handleBlurCleanup('name', item.name)"
                class="w-full bg-white border border-zinc-300 rounded-lg p-2 text-xs font-bold focus:outline-none text-[#2D3A34] min-h-[38px]"
              />
              <span class="text-[8px] block mt-0.5" :class="nameValidationMsg.includes('⚠️') ? 'text-amber-600 font-semibold' : 'text-zinc-400'">{{ nameValidationMsg }}</span>
            </div>
            <div>
              <label class="text-[9px] font-bold text-[#6B7C74] uppercase tracking-wider block mb-0.5">Marke / Hersteller</label>
              <input
                v-model="item.brand"
                @input="handleLiveSanitize('brand', $event.target.value)"
                @blur="handleBlurCleanup('brand', item.brand)"
                placeholder="Z.B. EIGENMARKE, UNKNOWN..."
                class="w-full bg-white border border-zinc-300 rounded-lg p-2 text-xs font-bold focus:outline-none text-[#2D3A34] min-h-[38px]"
              />
              <span class="text-[8px] block mt-0.5" :class="brandValidationMsg.includes('⚠️') ? 'text-amber-600 font-semibold' : 'text-zinc-400'">{{ brandValidationMsg }}</span>
            </div>
          </div>

          <div>
            <label class="text-[9px] font-bold text-[#6B7C74] uppercase tracking-wider block mb-0.5">Kategorie</label>
            <select
              v-model="item.category"
              class="w-full bg-white border border-zinc-300 rounded-lg px-2 py-1.5 text-xs font-semibold focus:outline-none focus:border-[#2D6A4F] text-[#2D3A34] min-h-[38px] cursor-pointer"
            >
              <option v-for="cat in categoriesList" :key="cat.value" :value="cat.value">
                {{ cat.label }}
              </option>
            </select>
          </div>

          <div v-if="item.ean" :class="['text-[10px] font-mono font-bold px-1', isDuplicate ? 'text-amber-700' : 'text-[#2D6A4F]']">
            EAN: {{ item.ean }} <span v-if="isDuplicate">(Duplikat!)</span>
          </div>
          <div class="flex flex-wrap gap-1">
            <button
              v-for="sug in commonProductSuggestions"
              :key="sug"
              type="button"
              @click="emit('rename', index, sug.toUpperCase())"
              class="text-[9px] bg-white border border-zinc-200 px-2 py-1 rounded text-[#6B7C74] font-medium min-h-[28px]"
            >
              {{ sug }}
            </button>
          </div>
        </div>
      </div>

      <div class="flex flex-col gap-3 pt-3 border-t border-zinc-200/40">
        <div class="flex flex-wrap items-center justify-between gap-2">
          <div class="flex items-center bg-white border border-zinc-200 rounded-xl overflow-hidden h-10 shadow-sm">
            <button type="button" @click="emit('updateQuantity', index, -1)" class="px-4 text-zinc-500 font-bold hover:bg-zinc-100 text-sm h-full w-12 flex items-center justify-center border-r border-zinc-100">-</button>
            <span class="px-3 text-xs font-mono font-bold text-[#2D3A34] min-w-[36px] text-center">{{ item.quantity || 1 }}x</span>
            <button type="button" @click="emit('updateQuantity', index, 1)" class="px-4 text-zinc-500 font-bold hover:bg-zinc-100 text-sm h-full w-12 flex items-center justify-center border-l border-zinc-100">+</button>
          </div>

          <div class="flex items-center gap-1.5">
            <button type="button" @click="emit('manuallyEnterEan', index)" class="text-xs font-bold px-3 py-2 rounded-xl border border-zinc-300 bg-white hover:bg-zinc-100 text-zinc-700 h-10 flex items-center shadow-sm cursor-pointer">
              ⌨️ Manuell
            </button>
            <button
              type="button"
              @click="emit('startEanScanner', index)"
              :class="['text-xs font-bold px-4 py-2 rounded-xl border transition-colors cursor-pointer h-10 flex items-center shadow-sm', isDuplicate ? 'bg-amber-100 border-amber-400 text-amber-800' : (item.ean ? 'bg-[#EAF5ED] border-[#2D6A4F]/30 text-[#2D6A4F]' : 'bg-[#D8F3DC] border-[#74C69D] text-[#1B4332] animate-pulse')]"
            >
              {{ item.ean ? '✓ EAN erneut scannen' : 'EAN Scan' }}
            </button>
          </div>
        </div>

        <div class="bg-white border border-zinc-200 rounded-xl p-3 flex flex-col gap-2.5 shadow-sm">
          <div class="flex justify-between items-center px-1">
            <span class="text-[10px] text-[#6B7C74] font-bold uppercase tracking-wider">Einzelpreis</span>
            <div class="flex items-center bg-[#F4F7F5] border border-zinc-200 rounded-lg px-3 w-32 focus-within:border-[#2D6A4F] min-h-[36px]">
              <input
                type="number"
                step="0.01"
                v-model.number="item.price"
                class="text-right text-xs font-mono font-bold text-[#2D3A34] bg-transparent w-full focus:outline-none"
              />
              <span class="text-xs font-bold text-[#6B7C74] ml-1">€</span>
            </div>
          </div>
          <div class="grid grid-cols-6 bg-[#F4F7F5] rounded-lg p-1 gap-1">
            <button type="button" @click="emit('updatePrice', index, -1.00)" class="py-2.5 text-xs font-bold text-zinc-600 bg-white border border-zinc-200/60 rounded-md shadow-sm active:scale-95 transition-all min-h-[38px]">-1€</button>
            <button type="button" @click="emit('updatePrice', index, -0.10)" class="py-2.5 text-xs font-bold text-zinc-500 bg-white border border-zinc-200/60 rounded-md shadow-sm active:scale-95 transition-all min-h-[38px]">-10c</button>
            <button type="button" @click="emit('updatePrice', index, -0.01)" class="py-2.5 text-xs font-bold text-zinc-400 bg-white border border-zinc-200/60 rounded-md shadow-sm active:scale-95 transition-all min-h-[38px]">-1c</button>
            <button type="button" @click="emit('updatePrice', index, 0.01)" class="py-2.5 text-xs font-bold text-zinc-400 bg-white border border-zinc-200/60 rounded-md shadow-sm active:scale-95 transition-all min-h-[38px]">+1c</button>
            <button type="button" @click="emit('updatePrice', index, 0.10)" class="py-2.5 text-xs font-bold text-zinc-500 bg-white border border-zinc-200/60 rounded-md shadow-sm active:scale-95 transition-all min-h-[38px]">+10c</button>
            <button type="button" @click="emit('updatePrice', index, 1.00)" class="py-2.5 text-xs font-bold text-zinc-600 bg-white border border-zinc-200/60 rounded-md shadow-sm active:scale-95 transition-all min-h-[38px]">+1€</button>
          </div>
        </div>

        <div class="grid grid-cols-3 gap-2 pt-1">
          <button type="button" @click="emit('remove', index)" class="bg-red-50 hover:bg-red-100 text-red-600 font-bold text-xs py-3 rounded-xl border border-red-200/60 flex items-center justify-center min-h-[46px]">Löschen</button>
          <button type="button" @click="emit('toggleEdit', index)" class="bg-[#2D6A4F] hover:bg-[#1B4332] text-white font-bold text-xs py-3 rounded-xl shadow-sm flex items-center justify-center min-h-[46px]">Speichern</button>
          <button type="button" @click="emit('toggleEdit', index)" class="bg-zinc-100 hover:bg-zinc-200 text-zinc-600 font-bold text-xs py-3 rounded-xl border border-zinc-300/40 flex items-center justify-center min-h-[46px]">Abbrechen</button>
        </div>
      </div>
    </template>

    <template v-if="!isActive">
      <div @click="emit('toggleEdit', index)" class="flex-1 min-w-0 cursor-pointer py-1">
        <div class="text-xs font-bold text-[#2D3A34] truncate flex items-center gap-1.5 flex-wrap">
          <span>{{ item.name }}</span>
          <span v-if="item.brand && item.brand !== 'UNKNOWN'" class="text-[10px] text-[#6B7C74] font-medium bg-zinc-100 px-1.5 py-0.5 rounded border border-zinc-200/30">
            {{ item.brand }}
          </span>
          <span v-if="!item.ean" class="w-1.5 h-1.5 rounded-full bg-amber-500" title="EAN fehlt"></span>
          <span v-else-if="isDuplicate" class="w-2 h-2 rounded-full bg-red-500 animate-ping" title="EAN doppelt"></span>
        </div>
        <div :class="['text-[10px] font-medium mt-0.5 flex flex-wrap items-center gap-1.5', isDuplicate ? 'text-amber-800' : 'text-[#6B7C74]']">
          <span>{{ item.quantity || 1 }}x · {{ Number(item.price).toFixed(2) }} €</span>
          <span class="bg-[#F4F7F5] text-[#2D6A4F] px-1.5 py-0.5 rounded text-[9px] font-bold border border-zinc-200/30">
            {{ categoriesList.find(c => c.value === item.category)?.label || '❓ Unkategorisiert' }}
          </span>
          <span v-if="item.ean" :class="['font-mono font-bold', isDuplicate ? 'text-red-700' : 'text-[#2D6A4F]']">
            | EAN: {{ item.ean }} {{ isDuplicate ? '(Doppelt)' : '' }}
          </span>
        </div>
      </div>

      <div class="flex items-center gap-1.5 shrink-0">
        <span class="text-xs font-mono font-bold text-[#2D3A34] px-1">
          {{ formatPrice(Number(item.price || 0) * (item.quantity || 1)) }}
        </span>
        <button type="button" @click="emit('manuallyEnterEan', index)" class="text-xs font-bold px-2.5 py-2 rounded-xl border border-zinc-200 bg-white hover:bg-zinc-50 text-zinc-600 min-h-[40px] flex items-center shadow-sm">
          ⌨️
        </button>
        <button
          type="button"
          @click="emit('startEanScanner', index)"
          :class="['text-xs font-bold px-3 py-2 rounded-xl transition-all min-h-[40px] flex items-center shadow-sm border', isDuplicate ? 'bg-amber-100 text-amber-900 border-amber-300' : (item.ean ? 'bg-[#EAF5ED] text-[#2D6A4F] border-[#2D6A4F]/20' : 'bg-[#D8F3DC] text-[#1B4332] border-[#74C69D]')]"
        >
          {{ item.ean ? (isDuplicate ? 'EAN korrigieren' : 'EAN ändern') : 'EAN Scan' }}
        </button>
      </div>
    </template>
  </div>
</template>
