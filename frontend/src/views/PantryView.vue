<script setup>
import { ref, computed, onMounted } from 'vue'
import { useBonsaiStore } from '../stores/BonsaiStore.js'

const store = useBonsaiStore()
const selectedCategory = ref(null)
const categoryDetails = ref([])

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

const activeCategories = computed(() => {
  return categoriesList.map(cat => {
    const backendData = store.pantrySummary.find(item => item.category === cat.value)
    const emoji = cat.label.match(/[\p{Emoji_Presentation}\p{Emoji}\u200d]+/u)?.[0] || '❓'
    const cleanLabel = cat.label.replace(emoji, '').trim()

    return {
      value: cat.value,
      cleanLabel,
      emoji,
      totalQuantity: backendData ? backendData.total_quantity : 0,
      totalPrice: backendData ? backendData.total_price : 0
    }
  }).filter(cat => cat.totalQuantity > 0)
})

async function selectCategory(cat) {
  selectedCategory.value = cat
  try {
    const response = await fetch(`/api/pantry/category/${cat.value}`)
    if (response.ok) {
      categoryDetails.value = await response.json()
    }
  } catch (error) {
    console.error(error)
  }
}

onMounted(() => {
  store.fetchPantrySummary()
})
</script>

<template>
  <div class="animate-[fadeIn_0.3s_ease-out]">
    <div v-if="!selectedCategory">
      <div class="flex justify-between items-center mb-4">
        <h2 class="text-lg font-bold text-[#2D3A34]">Deine Vorratskammer</h2>
        <span class="text-xs bg-[#B7E4C7] text-[#2D6A4F] px-2.5 py-1 rounded-xl font-semibold">
          Kategorien ({{ activeCategories.length }})
        </span>
      </div>

      <div class="grid grid-cols-2 gap-3.5">
        <div
          v-for="cat in activeCategories"
          :key="cat.value"
          @click="selectCategory(cat)"
          class="bg-white rounded-[20px] p-3.5 border border-black/5 active:scale-[0.96] transition-transform cursor-pointer"
        >
          <div class="flex justify-between items-start mb-2.5">
            <span class="text-3xl">{{ cat.emoji }}</span>
          </div>
          <h3 class="text-sm font-semibold text-[#2D3A34] truncate mb-0.5">{{ cat.cleanLabel }}</h3>

          <div class="mt-4 flex justify-between text-xs text-[#6B7C74]">
            <div>
              <p class="text-[10px] uppercase tracking-wider text-zinc-400 font-bold">Menge</p>
              <p class="font-semibold text-zinc-700">{{ cat.totalQuantity }} Stk.</p>
            </div>
            <div class="text-right">
              <p class="text-[10px] uppercase tracking-wider text-zinc-400 font-bold">Wert</p>
              <p class="font-semibold text-[#2D6A4F]">{{ cat.totalPrice.toFixed(2) }} €</p>
            </div>
          </div>
        </div>
      </div>
    </div>

    <div v-else>
      <div class="flex items-center gap-3 mb-6">
        <button
          @click="selectedCategory = null"
          class="p-2 bg-zinc-100 active:bg-zinc-200 rounded-full transition-colors"
        >
          <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round"><path d="m15 18-6-6 6-6"/></svg>
        </button>
        <div>
          <span class="text-2xl mr-1">{{ selectedCategory.emoji }}</span>
          <h2 class="text-lg inline-block font-bold text-[#2D3A34]">{{ selectedCategory.cleanLabel }}</h2>
        </div>
      </div>

      <div class="bg-white rounded-[20px] border border-black/5 overflow-hidden">
        <div v-if="categoryDetails.length === 0" class="p-8 text-center text-zinc-400 text-sm">
          Keine Artikel in dieser Kategorie vorhanden.
        </div>
        <div v-else class="divide-y divide-zinc-100">
          <div v-for="item in categoryDetails" :key="item.name" class="p-4 flex justify-between items-center">
            <div>
              <p class="font-semibold text-sm text-[#2D3A34]">{{ item.name }}</p>
              <p class="text-xs text-zinc-400">{{ item.brand }}</p>
            </div>
            <div class="text-right">
              <p class="text-sm font-bold text-zinc-700">{{ item.quantity }}x</p>
              <p class="text-xs text-[#2D6A4F]">{{ item.price.toFixed(2) }} €</p>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>
