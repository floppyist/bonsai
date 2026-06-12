<script setup>
import { ref, nextTick } from 'vue'
import { useBonsaiStore } from '../stores/BonsaiStore.js'

import ReceiptHeader from '../components/ReceiptHeader.vue'
import ReceiptArticleRow from '../components/ReceiptArticleRow.vue'
import ReceiptSummary from '../components/ReceiptSummary.vue'

const props = defineProps({
  formatPrice: {
    type: Function,
    default: (p) => `${Number(p).toFixed(2)} €`
  }
})

const emit = defineEmits(['startEanScanner', 'saveAndCloseReceipt'])

const store = useBonsaiStore()

const showEanInput = ref(false);
const eanInputIndex = ref(null);
const eanInputValue = ref('');
const eanInputRef = ref(null);

const activeEditIndex = ref(null)

const manuallyEnterEan = (index) => {
  eanInputIndex.value = index
  eanInputValue.value = store.currentReceipt.articles[index]?.ean || ''
  showEanInput.value = true
  nextTick(() => eanInputRef.value?.focus())
}

const confirmEanInput = () => {
  if (eanInputValue.value.trim()) {
    store.lookupEanForArticle(eanInputIndex.value, eanInputValue.value.trim())
  }

  showEanInput.value = false
  eanInputIndex.value = null
  eanInputValue.value = ''
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
      :store="store.currentReceipt.retailer"
      @update:store="store.updateRetailer"
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
          :categoriesList="store.categoriesList"
          :formatPrice="formatPrice"
          @toggleEdit="activeEditIndex = activeEditIndex === index ? null : index"
          @updateQuantity="(idx, delta) => store.updateQuantity(idx, delta)"
          @updatePrice="(idx, delta) => store.updatePrice(idx, delta)"
          @remove="(idx) => { store.removeArticle(idx); activeEditIndex = activeEditIndex === idx ? null : (activeEditIndex > idx ? activeEditIndex - 1 : activeEditIndex); }"
          @rename="(name) => item.name = name"
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

    <div v-if="showEanInput"
      class="fixed inset-0 bg-black/30 flex items-center justify-center z-50"
      @click.self="showEanInput = false">
      <div class="bg-white rounded-2xl p-5 shadow-xl mx-4 w-full max-w-sm">
        <h3 class="text-sm font-bold text-[#2D3A34] mb-3">EAN-Code eingeben</h3>
        <input ref="eanInputRef" v-model="eanInputValue"
          type="text" inputmode="numeric"
          placeholder="z.B. 4012345678900"
          class="w-full border border-zinc-200 rounded-xl px-4 py-3 text-sm mb-3 outline-none focus:border-[#2D6A4F]"
          @keydown.enter="confirmEanInput"
          @keydown.escape="showEanInput = false" />
        <div class="flex gap-2">
          <button @click="showEanInput = false" class="flex-1 py-2.5 text-sm font-medium text-zinc-500 bg-zinc-100 rounded-xl cursor-pointer">Abbrechen</button>
          <button @click="confirmEanInput" class="flex-1 py-2.5 text-sm font-medium text-white bg-[#2D6A4F] rounded-xl cursor-pointer">Bestätigen</button>
        </div>
      </div>
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
