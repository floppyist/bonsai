import { defineStore } from 'pinia'

export const useBonsaiStore = defineStore('bonsai', {
  state: () => ({
    isDevMode: false,

    receiptsHistory: [
      { id: 1, retailer: 'Rewe Markt', date: '2026-06-08', itemCount: 5, totalAmount: 24.50 }
    ],

    pantryItems: [],

    currentReceipt: {
      retailer: '',
      date: '',
      articles: [],
      scannedTotalAmount: null
    },

    isLoadingPantry: false,
    isSubmittingReceipt: false,

    categoriesList: [
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
      { value: 'TOBACCO_ACCESSORIES', label: '🚬 Tabak & Zubehör' },
      { value: 'UNCATEGORIZED', label: '❓ Unkategorisiert' },
    ],
  }),

  getters: {
    pantrySummary: (state) => {
      const summaryMap = {}

      state.pantryItems.forEach(item => {
        if (!summaryMap[item.category]) {
          summaryMap[item.category] = { total_quantity: 0, total_price: 0 }
        }

        summaryMap[item.category].total_quantity += item.quantity
        summaryMap[item.category].total_price += item.price
      })

      return Object.keys(summaryMap).map(category => ({
        category,
        total_quantity: summaryMap[category].total_quantity,
        total_price: summaryMap[category].total_price
      }))
    },

    currentTotalAmount: (state) => {
      return state.currentReceipt.articles.reduce(
        (sum, item) => sum + (Number(item.price || 0) * (item.quantity || 1)), 0
      )
    },

    isStoreInvalid: (state) => {
      const store = state.currentReceipt.retailer

      return !store || store.trim() === '' || store === 'Unbekannter Händler'
    },

    isDateInvalid: (state) => {
      const date = state.currentReceipt.date

      return !date || date.trim() === ''
    },

    isPriceInvalid: (state) => {
      return state.currentReceipt.articles.reduce((sum, item) => sum + (Number(item.price || 0) * (item.quantity || 1)), 0) <= 0
    },

    isMissingEan: (state) => {
      return state.currentReceipt.articles.some(item => !item.ean)
    },

    hasDuplicateEan: (state) => {
      const eans = state.currentReceipt.articles.map(item => item.ean).filter(Boolean)

      return new Set(eans).size !== eans.length
    },

    hasAmountMismatch: (state) => {
      if (state.currentReceipt.scannedTotalAmount === null) return false

      const currentTotal = state.currentReceipt.articles.reduce((sum, item) => sum + (Number(item.price || 0) * (item.quantity || 1)), 0)

      return state.currentReceipt.scannedTotalAmount.toFixed(2) !== currentTotal.toFixed(2)
    },

    isFormInvalid() {
      return this.isStoreInvalid || this.isDateInvalid || this.isPriceInvalid || this.isMissingEan || this.hasDuplicateEan
    }
  },

  actions: {
    async fetchPantryItems() {
      this.isLoadingPantry = true
      try {
        const response = await fetch('/api/pantry/items')

        if (!response.ok) return

        this.pantryItems = await response.json()
      } catch (error) {
        console.error('Failed fetching pantry items:', error)
      } finally {
        this.isLoadingPantry = false
      }
    },

    setReceiptData(rawData) {
      if (!rawData) return

      const rawStore = rawData.retailer?.value || rawData.store_name || rawData.retailer || 'Unbekannter Händler'
      this.currentReceipt.retailer = typeof rawStore === 'object' ? 'Unbekannter Händler' : String(rawStore)

      const rawDate = rawData.date?.value || rawData.date || new Date().toISOString().split('T')[0]
      this.currentReceipt.date = typeof rawDate === 'object' ? new Date().toISOString().split('T')[0] : String(rawDate)

      this.currentReceipt.scannedTotalAmount = Number(rawData.total_amount?.value ?? rawData.total_amount ?? null)

      this.currentReceipt.articles = Array.isArray(rawData.articles) ? rawData.articles.map(item => {
        const originalName = item.name || 'Unbekannter Artikel';

        return {
          ...item,
          raw_ocr_name: item.raw_ocr_name ?? String(originalName),

          name: String(originalName).toUpperCase(),

          price: Number(item.price ?? item.price_unit ?? 0.00),
          quantity: item.quantity || 1,
          ean: item.ean || null,
          category: item.category || 'UNCATEGORIZED',
          brand: String(item.brand || 'UNKNOWN').toUpperCase()
        }
      }) : []
    },

    updateArticleField(index, field, value) {
      if (this.currentReceipt.articles[index]) {
        this.currentReceipt.articles[index][field] = value
      }
    },
    updateQuantity(index, delta) {
      const item = this.currentReceipt.articles[index]
      if (item) item.quantity = Math.max(1, (item.quantity || 1) + delta)
    },
    updatePrice(index, delta) {
      const item = this.currentReceipt.articles[index]
      if (item) item.price = Math.max(0, Number((Number(item.price || 0) + delta).toFixed(2)))
    },
    removeArticle(index) {
      this.currentReceipt.articles.splice(index, 1)
    },
    addNewArticle() {
      this.currentReceipt.articles.push({
        name: '',
        raw_ocr_name: '',
        brand: 'UNKNOWN',
        category: 'UNCATEGORIZED',
        price: 0.00,
        quantity: 1,
        ean: null
      });
    },

    async lookupEanForArticle(index, ean) {
      if (!this.currentReceipt.articles[index]) return

      this.currentReceipt.articles[index].ean = ean

      try {
        const response = await fetch(`/api/ean/${ean}`)
        if (!response.ok) return

        const data = await response.json()

        if (data && data.name) {
          this.currentReceipt.articles[index].name = String(data.name).toUpperCase()
          this.currentReceipt.articles[index].brand = String(data.brand || 'UNKNOWN').toUpperCase()
          this.currentReceipt.articles[index].category = data.category || 'UNCATEGORIZED'

          if (data.price || data.price_unit) {
            this.currentReceipt.articles[index].price = Number(data.price || data.price_unit || 0)
          }
        }
      } catch (error) {
        console.error('EAN Lookup failed:', error)
      }
    },

    async finalizeReceipt() {
      if (this.isFormInvalid) return false
      this.isSubmittingReceipt = true

      const payload = {
        retailer: this.currentReceipt.retailer,
        receipt_date: this.currentReceipt.date,
        total_amount: Number(this.currentTotalAmount.toFixed(2)),
        articles: this.currentReceipt.articles
      }

      try {
        const response = await fetch('/api/finalize-receipt', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify(payload)
        })
        if (!response.ok) throw new Error(`Server-Fehler: ${response.status}`)

        this.receiptsHistory.unshift({
          id: Date.now(),
          retailer: payload.retailer,
          date: payload.receipt_date,
          itemCount: payload.articles.length,
          totalAmount: payload.total_amount
        })

        this.currentReceipt = { retailer: '', date: '', articles: [], scannedTotalAmount: null }
        return true
      } catch (error) {
        alert('Speichern fehlgeschlagen: ' + error.message)
        return false
      } finally {
        this.isSubmittingReceipt = false
      }
    },

    updateRetailer(name) {
      this.currentReceipt.retailer = name
      this.revalidateCurrentArticles()
    },

    async revalidateCurrentArticles() {
      if (!this.currentReceipt.retailer || this.currentReceipt.articles.length === 0) return;

      try {
        const response = await fetch('/api/receipt/revalidate', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({
            retailer: this.currentReceipt.retailer,
            articles: this.currentReceipt.articles
          })
        });

        if (response.ok) {
          const updatedArticles = await response.json();

          this.currentReceipt.articles = updatedArticles;

          console.log('Artikel erfolgreich revalidiert für:', this.currentReceipt.retailer);
        } else {
          console.error("Revalidation fehlgeschlagen:", response.statusText);
        }
      } catch (err) {
        console.error("Fehler beim Live-Abgleich der Artikel:", err);
      }
    },
  }
})
