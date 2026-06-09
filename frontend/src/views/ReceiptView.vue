<script setup>
import { ref, computed, watch } from 'vue'
import ReceiptHeader from '../components/ReceiptHeader.vue'
import ReceiptArticleRow from '../components/ReceiptArticleRow.vue'
import ReceiptSummary from '../components/ReceiptSummary.vue'

const props = defineProps({
  receiptData: {
    type: Object,
    default: () => ({ retailer: { value: '' }, date: { value: '' }, articles: [], total_amount: { value: null } })
  },
  formatPrice: {
    type: Function,
    default: (p) => `${Number(p).toFixed(2)} €`
  }
})

const emit = defineEmits(['startEanScanner', 'saveAndCloseReceipt'])

const editedStore = ref('')
const editedDate = ref('')
const articles = ref([])
const activeEditIndex = ref(null)
const isSubmitting = ref(false)

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

const parseIncomingData = (data) => {
  if (!data) return

  const rawStore = data.retailer?.value || data.store_name || data.retailer || 'Unbekannter Händler'
  editedStore.value = typeof rawStore === 'object' ? 'Unbekannter Händler' : String(rawStore)

  const rawDate = data.date?.value || data.date || new Date().toISOString().split('T')[0]
  editedDate.value = typeof rawDate === 'object' ? new Date().toISOString().split('T')[0] : String(rawDate)

  articles.value = Array.isArray(data.articles) ? data.articles.map(item => ({
    ...item,
    price: Number(item.price ?? item.price_unit ?? 0.00),
    quantity: item.quantity || 1,
    name: String(item.name || 'Unbekannter Artikel').toUpperCase(),
    ean: item.ean || null,
    category: item.category || 'UNCATEGORIZED',
    brand: String(item.brand || 'UNKNOWN').toUpperCase()
  })) : []
}

parseIncomingData(props.receiptData)
watch(() => props.receiptData, (newData) => parseIncomingData(newData), { deep: true })

const updateField = (index, field, value) => {
  if (articles.value[index]) articles.value[index][field] = value
}

const updateEan = async (index, newEan) => {
  if (!articles.value[index]) return
  articles.value[index].ean = newEan
  try {
    const response = await fetch(`/api/ean/${newEan}`)
    if (!response.ok) return
    const data = await response.json()
    if (data.found && data.name) {
      articles.value[index].name = data.name.toUpperCase()
      if (data.category) articles.value[index].category = data.category
      if (data.brand) articles.value[index].brand = data.brand.toUpperCase()
    }
  } catch (error) {
    console.error('EAN Lookup failed:', error)
  }
}
defineExpose({ updateEan })

const updateQuantity = (index, delta) => {
  if (articles.value[index]) articles.value[index].quantity = Math.max(1, (articles.value[index].quantity || 1) + delta)
}

const updatePrice = (index, delta) => {
  if (articles.value[index]) articles.value[index].price = Math.max(0, Number((Number(articles.value[index].price || 0) + delta).toFixed(2)))
}

const removeArticle = (index) => {
  articles.value.splice(index, 1)
  activeEditIndex.value = activeEditIndex.value === index ? null : (activeEditIndex.value > index ? activeEditIndex.value - 1 : activeEditIndex.value)
}

const addNewArticle = () => {
  articles.value.push({ name: 'NEUER ARTIKEL', quantity: 1, price: 1.00, ean: null, category: 'UNCATEGORIZED', brand: 'UNKNOWN' })
  activeEditIndex.value = articles.value.length - 1
}

const totalAmount = computed(() => articles.value.reduce((sum, item) => sum + (Number(item.price || 0) * (item.quantity || 1)), 0))
const scannedTotalAmount = computed(() => Number(props.receiptData?.total_amount?.value ?? props.receiptData?.total_amount ?? null))
const hasAmountMismatch = computed(() => scannedTotalAmount.value !== null && totalAmount.value.toFixed(2) !== scannedTotalAmount.value.toFixed(2))
const isStoreInvalid = computed(() => !editedStore.value || editedStore.value.trim() === '' || editedStore.value === 'Unbekannter Händler')
const isDateInvalid = computed(() => !editedDate.value || editedDate.value.trim() === '')
const isPriceInvalid = computed(() => totalAmount.value <= 0)
const isMissingEan = computed(() => articles.value.some(item => !item.ean))
const hasDuplicateEan = computed(() => {
  const eans = articles.value.map(item => item.ean).filter(Boolean)
  return new Set(eans).size !== eans.length
})
const isFormInvalid = computed(() => isStoreInvalid.value || isDateInvalid.value || isPriceInvalid.value || isMissingEan.value || hasDuplicateEan.value)

const manuallyEnterEan = (index) => {
  const input = prompt('Bitte EAN-Barcode manuell eingeben:', articles.value[index]?.ean || '')
  if (input !== null && input.trim()) updateEan(index, input.trim())
}

const handleSave = async () => {
  if (isFormInvalid.value) return
  isSubmitting.value = true
  const payload = {
    retailer: editedStore.value,
    receipt_date: editedDate.value,
    total_amount: Number(totalAmount.value.toFixed(2)),
    articles: articles.value
  }
  try {
    const response = await fetch('/api/finalize-receipt', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(payload)
    })
    if (!response.ok) throw new Error(`Server-Fehler: ${response.status}`)
    emit('saveAndCloseReceipt')
  } catch (error) {
    alert('Speichern fehlgeschlagen: ' + error.message)
  } finally {
    isSubmitting.value = false
  }
}
</script>

<template>
  <div class="animate-[fadeIn_0.3s_ease-out] space-y-4 pb-12 w-full max-w-full overflow-hidden">

    <ReceiptHeader
      v-model:store="editedStore"
      v-model:date="editedDate"
      :isStoreInvalid="isStoreInvalid"
      :isDateInvalid="isDateInvalid"
    />

    <div class="bg-white rounded-[24px] p-5 shadow-sm border border-black/5 space-y-4">
      <h3 class="text-xs font-bold text-[#2D6A4F] uppercase tracking-wider">Erkannte Artikel</h3>

      <div class="space-y-2 divider-y divider-zinc-50">
        <ReceiptArticleRow
          v-for="(item, index) in articles"
          :key="index"
          :item="item"
          :index="index"
          :isActive="activeEditIndex === index"
          :isDuplicate="item.ean ? articles.filter(a => a.ean === item.ean).length > 1 : false"
          :categoriesList="categoriesList"
          :commonProductSuggestions="commonProductSuggestions"
          :formatPrice="formatPrice"
          @toggleEdit="activeEditIndex = activeEditIndex === index ? null : index"
          @updateField="updateField"
          @updateQuantity="updateQuantity"
          @updatePrice="updatePrice"
          @remove="removeArticle"
          @rename="(idx, name) => articles[idx].name = name"
          @manuallyEnterEan="manuallyEnterEan"
          @startEanScanner="(idx) => emit('startEanScanner', idx)"
        />
      </div>

      <button
        type="button"
        @click="addNewArticle"
        class="w-full bg-[#F4F7F5] hover:bg-[#D8F3DC] text-[#2D6A4F] border-2 border-dashed border-[#74C69D] text-xs font-bold py-3.5 rounded-xl flex items-center justify-center gap-1.5 transition-all cursor-pointer mt-2 min-h-[48px]"
      >
        <span>+</span> Weiteren Artikel hinzufügen
      </button>
    </div>

    <ReceiptSummary
      :totalAmount="totalAmount"
      :scannedTotalAmount="scannedTotalAmount"
      :itemCount="articles.length"
      :hasDuplicateEan="hasDuplicateEan"
      :hasAmountMismatch="hasAmountMismatch"
      :isFormInvalid="isFormInvalid"
      :isSubmitting="isSubmitting"
      :isMissingEan="isMissingEan"
      :isPriceInvalid="isPriceInvalid"
      :isStoreInvalid="isStoreInvalid"
      :isDateInvalid="isDateInvalid"
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
