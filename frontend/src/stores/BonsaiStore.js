import { defineStore } from 'pinia'

export const useBonsaiStore = defineStore('bonsai', {
  state: () => ({
    isDevMode: false,

    receiptsHistory: [
      { id: 1, retailer: 'Rewe Markt', date: '2026-06-08', itemCount: 5, totalAmount: 24.50 }
    ],

    pantryItems: [
      { id: 1, name: 'Milch', category: 'Milchprodukte & Eier', emoji: '🥛', stock: 80, stockColor: '#52B788', status: 'good', daysLeft: '5 Tage' },
      { id: 2, name: 'Tomaten', category: 'Obst & Gemüse', emoji: '🥦', stock: 20, stockColor: '#EF4444', status: 'urgent', daysLeft: '1 Tag' }
    ],

    currentReceipt: {
      retailer: '',
      date: '',
      articles: [],
      scannedTotalAmount: null
    },

    pantrySummary: [],

    isSubmittingReceipt: false,
  }),

  getters: {
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
    setReceiptData(rawData) {
      if (!rawData) return

      const rawStore = rawData.retailer?.value || rawData.store_name || rawData.retailer || 'Unbekannter Händler'
      this.currentReceipt.retailer = typeof rawStore === 'object' ? 'Unbekannter Händler' : String(rawStore)

      const rawDate = rawData.date?.value || rawData.date || new Date().toISOString().split('T')[0]
      this.currentReceipt.date = typeof rawDate === 'object' ? new Date().toISOString().split('T')[0] : String(rawDate)

      this.currentReceipt.scannedTotalAmount = Number(rawData.total_amount?.value ?? rawData.total_amount ?? null)

      this.currentReceipt.articles = Array.isArray(rawData.articles) ? rawData.articles.map(item => ({
        ...item,
        price: Number(item.price ?? item.price_unit ?? 0.00),
        quantity: item.quantity || 1,
        name: String(item.name || 'Unbekannter Artikel').toUpperCase(),
        ean: item.ean || null,
        category: item.category || 'UNCATEGORIZED',
        brand: String(item.brand || 'UNKNOWN').toUpperCase()
      })) : []
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
        name: 'NEUER ARTIKEL',
        quantity: 1,
        price: 1.00,
        ean: null,
        category: 'UNCATEGORIZED',
        brand: 'UNKNOWN'
      })
    },

    async lookupEanForArticle(index, ean) {
      if (!this.currentReceipt.articles[index]) return
      this.currentReceipt.articles[index].ean = ean

      try {
        const response = await fetch(`/api/ean/${ean}`)
        if (!response.ok) return
        const data = await response.json()
        if (data.found && data.name) {
          this.currentReceipt.articles[index].name = data.name.toUpperCase()
          if (data.category) this.currentReceipt.articles[index].category = data.category
          if (data.brand) this.currentReceipt.articles[index].brand = data.brand.toUpperCase()
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

    async fetchPantrySummary() {
      try {
        const response = await fetch('/api/pantry/summary')
        if (!response.ok) return
        this.pantrySummary = await response.json()
      } catch (error) {
        console.error('Failed fetching pantry summary:', error)
      }
    }
  }
})
