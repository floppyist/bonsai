<script setup>
import { ref, computed, watch } from 'vue'

const props = defineProps({
  receiptData: {
    type: Object,
    default: () => ({
      store_name: '',
      date: '',
      articles: [],
      total_amount: null
    })
  },
  formatPrice: {
    type: Function,
    default: (p) => `${Number(p).toFixed(2)} €`
  }
})

const emit = defineEmits(['startEanScanner', 'saveAndCloseReceipt'])

const editedStore = ref(props.receiptData?.store_name || 'Unbekannter Händler')
const editedDate = ref(props.receiptData?.date || new Date().toISOString().split('T')[0])

const articles = ref(
  props.receiptData?.articles
    ? props.receiptData.articles.map(item => ({
        ...item,
        price: item.price !== undefined && item.price !== null
          ? Number(item.price)
          : (item.price_unit !== undefined ? Number(item.price_unit) : 0.00),
        quantity: item.quantity || 1,
        name: item.name || 'Unbekannter Artikel',
        ean: item.ean || null
      }))
    : []
)

watch(() => props.receiptData?.articles, (newArticles) => {
  if (newArticles) {
    newArticles.forEach((art, i) => {
      if (articles.value[i] && art.ean !== undefined) {
        articles.value[i].ean = art.ean
      }
    })
  }
}, { deep: true })

const updateEan = (index, newEan) => {
  if (articles.value[index]) {
    articles.value[index].ean = newEan
  }
}

defineExpose({
  updateEan
})

const commonStores = ['Edeka', 'REWE', 'Aldi', 'Lidl', 'Kaufland', 'Bio-Markt', 'Netto', 'Penny']
const showStoreDropdown = ref(false)

const selectStore = (name) => {
  editedStore.value = name
  showStoreDropdown.value = false
}

const updateQuantity = (index, delta) => {
  const item = articles.value[index]
  if (item) {
    item.quantity = Math.max(1, (item.quantity || 1) + delta)
  }
}

const updatePrice = (index, delta) => {
  const item = articles.value[index]
  if (item) {
    item.price = Math.max(0, Number((Number(item.price || 0) + delta).toFixed(2)))
  }
}

const removeArticle = (index) => {
  articles.value.splice(index, 1)
  if (activeEditIndex.value === index) {
    activeEditIndex.value = null
  } else if (activeEditIndex.value > index) {
    activeEditIndex.value--
  }
}

const addNewArticle = () => {
  articles.value.push({
    name: 'Neuer Artikel',
    quantity: 1,
    price: 1.00,
    ean: null
  })
  activeEditIndex.value = articles.value.length - 1
}

const activeEditIndex = ref(null)
const commonProductSuggestions = ['Obst & Gemüse', 'Molkereiprodukte', 'Brot & Backwaren', 'Getränke', 'Fleisch & Fisch', 'Snacks']

const toggleEditRow = (index) => {
  activeEditIndex.value = activeEditIndex.value === index ? null : index
}

const renameArticle = (index, newName) => {
  articles.value[index].name = newName
}

const isMissingEan = computed(() => {
  return articles.value.some(item => !item.ean)
})

const hasDuplicateEan = computed(() => {
  const eans = articles.value
    .map(item => item.ean)
    .filter(ean => ean !== null && ean !== undefined && ean !== '')
  return new Set(eans).size !== eans.length
})

const isDuplicateArticle = (item) => {
  if (!item.ean) return false
  return articles.value.filter(a => a.ean === item.ean).length > 1
}

const totalAmount = computed(() => {
  return articles.value.reduce((sum, item) => sum + (Number(item.price || 0) * (item.quantity || 1)), 0)
})

const scannedTotalAmount = computed(() => {
  const total = props.receiptData?.total_amount
  if (total && typeof total === 'object' && 'value' in total) {
    return total.value !== undefined && total.value !== null ? Number(total.value) : null
  }
  return total !== undefined && total !== null ? Number(total) : null
})

const hasAmountMismatch = computed(() => {
  if (scannedTotalAmount.value === null) return false
  return totalAmount.value.toFixed(2) !== scannedTotalAmount.value.toFixed(2)
})

const isStoreInvalid = computed(() => !editedStore.value || editedStore.value.trim() === '' || editedStore.value === 'Unbekannter Händler')
const isDateInvalid = computed(() => !editedDate.value || editedDate.value.trim() === '')
const isPriceInvalid = computed(() => totalAmount.value <= 0)

const isFormInvalid = computed(() => {
  return isStoreInvalid.value || isDateInvalid.value || isPriceInvalid.value || isMissingEan.value || hasDuplicateEan.value
})

const handleSave = () => {
  if (isFormInvalid.value) return
  const updatedData = {
    ...props.receiptData,
    store_name: editedStore.value,
    date: editedDate.value,
    articles: articles.value,
    total_amount: totalAmount.value
  }
  emit('saveAndCloseReceipt', updatedData)
}
</script>

<template>
  <div class="animate-[fadeIn_0.3s_ease-out] space-y-4 pb-12 w-full max-w-full overflow-hidden">

    <div class="bg-white rounded-[24px] p-5 shadow-sm border border-black/5 space-y-4">

      <div class="relative">
        <label class="text-[10px] font-bold text-[#6B7C74] uppercase tracking-wider block mb-1">Händler</label>
        <div class="flex gap-2">
          <button
            @click="showStoreDropdown = !showStoreDropdown"
            :class="[
              'flex-1 text-left rounded-xl px-4 py-3 text-sm font-bold flex justify-between items-center cursor-pointer min-h-[48px] transition-all duration-200 border',
              isStoreInvalid
                ? 'bg-amber-50/50 border-amber-500/40 text-amber-900 shadow-sm shadow-amber-50'
                : 'bg-[#F4F7F5] border-zinc-200/60 text-[#2D3A34]'
            ]"
          >
            <span>{{ editedStore }}</span>
            <span class="text-xs opacity-50">▼</span>
          </button>
          <input
            v-model="editedStore"
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
            v-for="store in commonStores"
            :key="store"
            @click="selectStore(store)"
            class="text-left text-xs font-semibold py-2.5 px-3 rounded-lg hover:bg-[#F4F7F5] hover:text-[#2D6A4F] text-[#2D3A34] transition-colors min-h-[40px]"
          >
            {{ store }}
          </button>
        </div>
      </div>

      <div class="w-full block">
        <label class="text-[10px] font-bold text-[#6B7C74] uppercase tracking-wider block mb-1">Belegdatum</label>
        <div class="w-full relative block">
          <input
            type="date"
            v-model="editedDate"
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

    <div class="bg-white rounded-[24px] p-5 shadow-sm border border-black/5 space-y-4">
      <div class="flex justify-between items-center mb-1">
        <h3 class="text-xs font-bold text-[#2D6A4F] uppercase tracking-wider">Erkannte Artikel</h3>
      </div>

      <div class="space-y-2 divider-y divider-zinc-50">
        <div
          v-for="(item, index) in articles"
          :key="index"
          :class="[
            'rounded-xl transition-all border duration-200',
            activeEditIndex === index
              ? (isDuplicateArticle(item)
                  ? 'bg-amber-50/60 border-amber-500/40 p-4 space-y-3.5 shadow-sm'
                  : 'bg-[#F4F7F5] border-[#2D6A4F]/20 p-4 space-y-3.5')
              // Zustand 2: Reihe ist geschlossen
              : (isDuplicateArticle(item)
                  ? 'bg-amber-50/40 border-amber-500/25 hover:bg-amber-50/70 p-3 flex items-center justify-between gap-3 text-amber-950'
                  : (item.ean
                      ? 'bg-[#EAF5ED]/60 border-[#2D6A4F]/15 hover:bg-[#EAF5ED]/90 p-3 flex items-center justify-between gap-3'
                      : 'bg-white hover:bg-[#F4F7F5]/30 border-zinc-100 p-3 flex items-center justify-between gap-3'
                    )
                )
          ]"
        >
          <template v-if="activeEditIndex === index">
            <div class="flex justify-between items-start gap-2">
              <div class="flex-1 space-y-1.5">
                <input
                  v-model="item.name"
                  class="w-full bg-white border border-zinc-300 rounded-lg p-2 text-xs font-bold focus:outline-none text-[#2D3A34] min-h-[38px]"
                />
                <div v-if="item.ean" :class="['text-[10px] font-mono font-bold px-1', isDuplicateArticle(item) ? 'text-amber-700' : 'text-[#2D6A4F]']">
                  EAN: {{ item.ean }} <span v-if="isDuplicateArticle(item)">(Duplikat!)</span>
                </div>
                <div class="flex flex-wrap gap-1">
                  <button
                    v-for="sug in commonProductSuggestions"
                    :key="sug"
                    @click="renameArticle(index, sug)"
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
                  <button @click="updateQuantity(index, -1)" class="px-4 text-zinc-500 font-bold hover:bg-zinc-100 text-sm h-full w-12 flex items-center justify-center border-r border-zinc-100">-</button>
                  <span class="px-3 text-xs font-mono font-bold text-[#2D3A34] min-w-[36px] text-center">{{ item.quantity || 1 }}x</span>
                  <button @click="updateQuantity(index, 1)" class="px-4 text-zinc-500 font-bold hover:bg-zinc-100 text-sm h-full w-12 flex items-center justify-center border-l border-zinc-100">+</button>
                </div>

                <button
                  @click="emit('startEanScanner', index)"
                  :class="[
                    'text-xs font-bold px-4 py-2 rounded-xl border transition-colors cursor-pointer h-10 flex items-center shadow-sm',
                    isDuplicateArticle(item)
                      ? 'bg-amber-100 border-amber-400 text-amber-800 hover:bg-amber-200'
                      : (item.ean
                          ? 'bg-[#EAF5ED] border-[#2D6A4F]/30 text-[#2D6A4F] hover:bg-[#D8F3DC]'
                          : 'bg-[#D8F3DC] border-[#74C69D] text-[#1B4332] animate-pulse'
                        )
                  ]"
                >
                  {{ item.ean ? '✓ EAN erneut scannen' : 'EAN Scan' }}
                </button>
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
                  <button @click="updatePrice(index, -1.00)" class="py-2.5 text-xs font-bold text-zinc-600 bg-white border border-zinc-200/60 rounded-md shadow-sm active:scale-95 transition-all min-h-[38px]">-1€</button>
                  <button @click="updatePrice(index, -0.10)" class="py-2.5 text-xs font-bold text-zinc-500 bg-white border border-zinc-200/60 rounded-md shadow-sm active:scale-95 transition-all min-h-[38px]">-10c</button>
                  <button @click="updatePrice(index, -0.01)" class="py-2.5 text-xs font-bold text-zinc-400 bg-white border border-zinc-200/60 rounded-md shadow-sm active:scale-95 transition-all min-h-[38px]">-1c</button>
                  <button @click="updatePrice(index, 0.01)" class="py-2.5 text-xs font-bold text-zinc-400 bg-white border border-zinc-200/60 rounded-md shadow-sm active:scale-95 transition-all min-h-[38px]">+1c</button>
                  <button @click="updatePrice(index, 0.10)" class="py-2.5 text-xs font-bold text-zinc-500 bg-white border border-zinc-200/60 rounded-md shadow-sm active:scale-95 transition-all min-h-[38px]">+10c</button>
                  <button @click="updatePrice(index, 1.00)" class="py-2.5 text-xs font-bold text-zinc-600 bg-white border border-zinc-200/60 rounded-md shadow-sm active:scale-95 transition-all min-h-[38px]">+1€</button>
                </div>
              </div>

              <div class="grid grid-cols-3 gap-2 pt-1">
                <button
                  @click="removeArticle(index)"
                  class="bg-red-50 hover:bg-red-100 text-red-600 font-bold text-xs py-3 rounded-xl transition-colors border border-red-200/60 cursor-pointer flex items-center justify-center min-h-[46px]"
                >
                  Löschen
                </button>
                <button
                  @click="toggleEditRow(index)"
                  class="bg-[#2D6A4F] hover:bg-[#1B4332] text-white font-bold text-xs py-3 rounded-xl transition-colors shadow-sm cursor-pointer flex items-center justify-center min-h-[46px]"
                >
                  Speichern
                </button>
                <button
                  @click="toggleEditRow(index)"
                  class="bg-zinc-100 hover:bg-zinc-200 text-zinc-600 font-bold text-xs py-3 rounded-xl transition-colors border border-zinc-300/40 cursor-pointer flex items-center justify-center min-h-[46px]"
                >
                  Verwerfen
                </button>
              </div>
            </div>
          </template>

          <template v-if="activeEditIndex !== index">
            <div @click="toggleEditRow(index)" class="flex-1 min-w-0 cursor-pointer py-1">
              <div class="text-xs font-bold text-[#2D3A34] truncate flex items-center gap-2">
                <span>{{ item.name }}</span>
                <span v-if="!item.ean" class="w-1.5 h-1.5 rounded-full bg-amber-500" title="EAN fehlt"></span>
                <span v-else-if="isDuplicateArticle(item)" class="w-2 h-2 rounded-full bg-red-500 animate-ping" title="EAN doppelt"></span>
              </div>
              <div :class="['text-[10px] font-medium mt-0.5', isDuplicateArticle(item) ? 'text-amber-800' : 'text-[#6B7C74]']">
                {{ item.quantity || 1 }}x · {{ Number(item.price).toFixed(2) }} €
                <span v-if="item.ean" :class="['ml-1 font-mono font-bold', isDuplicateArticle(item) ? 'text-red-700' : 'text-[#2D6A4F]']">
                  | EAN: {{ item.ean }} {{ isDuplicateArticle(item) ? '(Doppelt)' : '' }}
                </span>
              </div>
            </div>

            <div class="flex items-center gap-3 shrink-0">
              <span class="text-xs font-mono font-bold text-[#2D3A34] px-1">
                {{ formatPrice(Number(item.price || 0) * (item.quantity || 1)) }}
              </span>

              <button
                @click="emit('startEanScanner', index)"
                :class="[
                  'text-xs font-bold px-3 py-2 rounded-xl transition-all cursor-pointer min-h-[40px] flex items-center shadow-sm border',
                  isDuplicateArticle(item)
                    ? 'bg-amber-100 hover:bg-amber-200 text-amber-900 border-amber-300'
                    : (item.ean
                        ? 'bg-[#EAF5ED] hover:bg-[#D8F3DC] text-[#2D6A4F] border-[#2D6A4F]/20'
                        : 'bg-[#D8F3DC] hover:bg-[#b7e4c7] text-[#1B4332] border-[#74C69D]'
                      )
                ]"
              >
                {{ item.ean ? (isDuplicateArticle(item) ? 'EAN korrigieren' : 'EAN ändern') : 'EAN Scan' }}
              </button>
            </div>
          </template>
        </div>
      </div>

      <button
        @click="addNewArticle"
        class="w-full bg-[#F4F7F5] hover:bg-[#D8F3DC] text-[#2D6A4F] border-2 border-dashed border-[#74C69D] text-xs font-bold py-3.5 rounded-xl flex items-center justify-center gap-1.5 transition-all cursor-pointer active:scale-[0.99] mt-2 min-h-[48px]"
      >
        <span class="text-sm">+</span> Weiteren Artikel hinzufügen
      </button>
    </div>

    <div class="bg-gradient-to-br from-[#2D6A4F] to-[#1B4332] rounded-[24px] p-5 text-white space-y-4 shadow-md">
      <div class="flex justify-between items-center">
        <div>
          <span class="text-[10px] text-white/70 uppercase tracking-wider block font-medium">Berechnete Summe</span>
          <span class="text-2xl font-black font-mono tracking-tight">{{ formatPrice(totalAmount) }}</span>
        </div>
        <span class="text-xs bg-white/10 px-3 py-1 rounded-full text-[#CCFF33] font-bold">
          {{ articles.length }} Positionen
        </span>
      </div>

      <div
        v-if="hasDuplicateEan"
        class="bg-red-500/20 text-red-300 border border-red-500/30 rounded-xl p-3 text-xs font-semibold flex items-start gap-2.5 animate-[fadeIn_0.2s_ease-out]"
      >
        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round" class="text-red-400 shrink-0 mt-0.5">
          <circle cx="12" cy="12" r="10"/>
          <line x1="12" y1="8" x2="12" y2="12"/>
          <line x1="12" y1="16" x2="12.01" y2="16"/>
        </svg>
        <div>
          <span class="font-bold block text-white">Doppelte Barcodes!</span>
          <span class="opacity-90">Einige Artikel besitzen dieselbe EAN. Jeder Artikel muss eindeutig sein (gelb markiert).</span>
        </div>
      </div>

      <div
        v-if="hasAmountMismatch && !hasDuplicateEan"
        class="bg-amber-500/20 text-amber-300 border border-amber-500/30 rounded-xl p-3 text-xs font-semibold flex items-start gap-2.5 animate-[fadeIn_0.2s_ease-out]"
      >
        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round" class="text-amber-400 shrink-0 mt-0.5">
          <path d="m21.73 18-8-14a2 2 0 0 0-3.48 0l-8 14A2 2 0 0 0 4 21h16a2 2 0 0 0 1.73-3Z"/>
          <line x1="12" y1="9" x2="12" y2="13"/>
          <line x1="12" y1="17" x2="12.01" y2="17"/>
        </svg>
        <div>
          <span class="font-bold block text-white">Betrag weicht ab!</span>
          <span class="opacity-90">Gescanntes Total vom Beleg: {{ formatPrice(scannedTotalAmount) }}</span>
        </div>
      </div>

      <button
        @click="handleSave"
        :disabled="isFormInvalid"
        :class="[
          'w-full font-bold py-4 px-4 rounded-xl text-sm shadow-lg transition-all duration-200 min-h-[48px]',
          isFormInvalid
            ? 'bg-zinc-700/50 text-zinc-400 border border-zinc-600/30 cursor-not-allowed opacity-60'
            : 'bg-[#CCFF33] hover:bg-[#b2eb2a] text-zinc-900 cursor-pointer active:scale-[0.99]'
        ]"
      >
        <template v-if="isMissingEan">Bitte erst alle Barcodes scannen</template>
        <template v-else-if="hasDuplicateEan">Doppelte Barcodes korrigieren</template>
        <template v-else-if="isPriceInvalid">Preis/Summe darf nicht 0 € sein</template>
        <template v-else-if="isStoreInvalid">Händler-Namen korrigieren</template>
        <template v-else-if="isDateInvalid">Gültiges Belegdatum angeben</template>
        <template v-else>Daten bestätigen & Einbuchen</template>
      </button>
    </div>

  </div>
</template>

<style scoped>
input[type="date"]::-webkit-calendar-picker-indicator {
  margin: 0;
  padding: 0;
  cursor: pointer;
}
</style>
