<script setup>
import { ref, watch } from 'vue'
import { useBonsaiStore } from '../stores/BonsaiStore.js'
import ReceiptHeader from '../components/ReceiptHeader.vue'
import ReceiptArticleRow from '../components/ReceiptArticleRow.vue'
import ReceiptSummary from '../components/ReceiptSummary.vue'

const props = defineProps({
  receiptData: {
    type: Object,
    default: null
  },
  formatPrice: {
    type: Function,
    default: (p) => `${Number(p).toFixed(2)} €`
  }
})

const emit = defineEmits(['startEanScanner', 'saveAndCloseReceipt'])

const store = useBonsaiStore()
const activeEditIndex = ref(null)

const categoriesList = [
  { value: 'BAKERY', label: '🍞 Brot & Backwaren' },
  { value: 'DAIRY_EGGS', label: '🥛 Milchprodukte & Eier' },
  { value: 'FRUIT_VEGETABLES', label: '🥦 Obst & Gemüse' },
  { value: 'MEAT_FISH', label: '🥩 Fleisch, Fisch & Ersatz' },
  { value: 'STAPLE_FOODS', label: '🍝 Grundnahrungsmittel' },
  { value: 'CANNED_JARRED', label: '🥫 Konserven & Gläser' },
  { value: 'OILS_SAUCES_SPICES', label: '🫚 Öle, Saucen & Gewürze' },
  { value: 'BREAKFAST_SPREADS', label: '🍯 Frühstück & Aufstriche' },
  { value: 'SNACKS_SWEETS', label: '🍫 Süßwaren & Knabberzeug' },
  { value: 'BEVERAGES', label: '☕ Getränke' },
  { value: 'DRUGSTORE_BEAUTY', label: '🧴 Drogerie & Kosmetik' },
  { value: 'HOUSEHOLD_CLEANING', label: '🧹 Haushalt & Reinigung' },
  { value: 'PET_SUPPLIES', label: '🐾 Haustierbedarf' },
  { value: 'ELECTRONICS', label: '🔌 Elektronik & Zubehör' },
  { value: 'STATIONERY_OFFICE', label: '✏️ Schreibwaren & Büro' },
  { value: 'HOME_LIVING', label: '🍳 Wohnen & Küche' },
  { value: 'CLOTHING_TEXTILES', label: '🧦 Kleidung & Textilien' },
  { value: 'DIY_GARDEN', label: '🛠️ Baumarkt & Garten' },
  { value: 'BOOKS_MEDIA', label: '📚 Medien & Freizeit' },
  { value: 'UNCATEGORIZED', label: '❓ Unkategorisiert' }
]

const commonProductSuggestions = ['Obst & Gemüse', 'Molkereiprodukte', 'Brot & Backwaren', 'Getränke', 'Fleisch & Fisch', 'Snacks']

if (props.receiptData) {
  store.setReceiptData(props.receiptData)
}

watch(() => props.receiptData, (newData) => {
  if (newData) store.setReceiptData(newData)
}, { deep: true })

const manuallyEnterEan = (index) => {
  const input = prompt('Bitte EAN-Barcode manuell eingeben:', store.currentReceipt.articles[index]?.ean || '')
  if (input !== null && input.trim()) {
    store.lookupEanForArticle(index, input.trim())
  }
}

const handleSave = async () => {
  const success = await store.finalizeReceipt()
  if (success) {
    emit('saveAndCloseReceipt')
  }
}
</script>

<template>
  <div class="animate-[fadeIn_0.3s_ease-out] space-y-4 pb-12 w-full max-w-full overflow-hidden">

    <ReceiptHeader
      v-model:store="store.currentReceipt.retailer"
      v-model:date="store.currentReceipt.date"
      :isStoreInvalid="store.isStoreInvalid"
      :isDateInvalid="store.isDateInvalid"
    />

    <div class="bg-white rounded-[24px] p-5 shadow-sm border border-black/5 space-y-4">
      <h3 class="text-xs font-bold text-[#2D6A4F] uppercase tracking-wider">Erkannte Artikel</h3>

      <div class="space-y-2 divider-y divider-zinc-50">
        <ReceiptArticleRow
          v-for="(item, index) in store.currentReceipt.articles"
          :key="index"
          :item="item"
          :index="index"
          :isActive="activeEditIndex === index"
          :isDuplicate="item.ean ? store.currentReceipt.articles.filter(a => a.ean === item.ean).length > 1 : false"
          :categoriesList="categoriesList"
          :commonProductSuggestions="commonProductSuggestions"
          :formatPrice="formatPrice"
          @toggleEdit="activeEditIndex = activeEditIndex === index ? null : index"
          @updateQuantity="(idx, delta) => store.updateQuantity(idx, delta)"
          @updatePrice="(idx, delta) => store.updatePrice(idx, delta)"
          @remove="(idx) => { store.removeArticle(idx); activeEditIndex = activeEditIndex === idx ? null : (activeEditIndex > idx ? activeEditIndex - 1 : activeEditIndex); }"
          @rename="(idx, name) => item.name = name"
          @manuallyEnterEan="manuallyEnterEan"
          @startEanScanner="(idx) => emit('startEanScanner', idx)"
        />
      </div>

      <button
        type="button"
        @click="() => { store.addNewArticle(); activeEditIndex = store.currentReceipt.articles.length - 1; }"
        class="w-full bg-[#F4F7F5] hover:bg-[#D8F3DC] text-[#2D6A4F] border-2 border-dashed border-[#74C69D] text-xs font-bold py-3.5 rounded-xl flex items-center justify-center gap-1.5 transition-all cursor-pointer mt-2 min-h-[48px]"
      >
        <span>+</span> Weiteren Artikel hinzufügen
      </button>
    </div>

    <ReceiptSummary
      :totalAmount="store.currentTotalAmount"
      :scannedTotalAmount="store.currentReceipt.scannedTotalAmount"
      :itemCount="store.currentReceipt.articles.length"
      :hasDuplicateEan="store.hasDuplicateEan"
      :hasAmountMismatch="store.hasAmountMismatch"
      :isFormInvalid="store.isFormInvalid"
      :isSubmitting="store.isSubmittingReceipt"
      :isMissingEan="store.isMissingEan"
      :isPriceInvalid="store.isPriceInvalid"
      :isStoreInvalid="store.isStoreInvalid"
      :isDateInvalid="store.isDateInvalid"
      :formatPrice="formatPrice"
      @save="handleSave"
    />

  </div>
</template>

<style scoped>
input[type="date"]::-webkit-calendar-picker-indicator {
  margin: 0;
  padding: 0;
  cursor: pointer;
}
</style>
