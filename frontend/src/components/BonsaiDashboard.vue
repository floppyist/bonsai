<template>
  <div class="w-full h-full bg-zinc-200 flex justify-center items-center m-0 p-0 antialiased selection:bg-[#CCFF33] selection:text-zinc-900">

    <div class="relative w-full h-screen bg-[#F4F7F5] shadow-2xl flex flex-col overflow-hidden">

      <header class="pt-15 bg-gradient-to-br from-[#2D6A4F] to-[#1B4332] px-5 pt-6 pb-8 text-white shrink-0">
        <div class="flex justify-between items-center mb-5">
          <div class="flex items-center gap-3">
            <img src="/icon.gif" alt="Bonsai Logo" class="w-8 h-8 object-contain" />
            <h1 class="text-2xl font-black tracking-tight bg-gradient-to-r from-white via-[#E8F3DC] to-[#CCFF33] bg-clip-text text-transparent">
              Bonsai
            </h1>
          </div>

          <div class="bg-white/10 backdrop-blur-md px-3 py-1.5 rounded-full text-xs flex gap-1 items-center transition-all duration-300">
            <template v-if="isProcessingInBackground">
              <span class="w-2 h-2 bg-[#CCFF33] rounded-full animate-ping mr-1"></span>
              <span class="text-zinc-200">KI scannt:</span>
              <span class="text-[#CCFF33] font-bold">Aktiv</span>
            </template>
            <template v-else>
              <span class="text-zinc-300">Pantry Status:</span>
              <span class="text-[#CCFF33] font-bold">Optimiert</span>
            </template>
          </div>
        </div>

        <div class="grid grid-cols-3 gap-2.5">
          <div class="bg-white/5 border border-white/10 rounded-2xl p-3 flex flex-col gap-1">
            <span class="text-lg">🥑</span>
            <div class="flex flex-col">
              <span class="text-sm font-bold">24</span>
              <span class="text-[10px] text-white/70">Produkte</span>
            </div>
          </div>
          <div class="bg-white/5 border border-white/10 rounded-2xl p-3 flex flex-col gap-1">
            <span class="text-lg">⏳</span>
            <div class="flex flex-col">
              <span class="text-sm font-bold text-red-300">3</span>
              <span class="text-[10px] text-white/70">Ablaufend</span>
            </div>
          </div>
          <div class="bg-white/5 border border-white/10 rounded-2xl p-3 flex flex-col gap-1">
            <span class="text-lg">💶</span>
            <div class="flex flex-col">
              <span class="text-sm font-bold text-[#CCFF33]">42,50 €</span>
              <span class="text-[10px] text-white/70">März-Log</span>
            </div>
          </div>
        </div>
      </header>

      <main class="flex-1 overflow-y-auto p-5 pb-28 scrollbar-thin scrollbar-thumb-zinc-300">

        <div v-if="activeTab === 'home'" class="animate-[fadeIn_0.3s_ease-out]">
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

        <div v-if="activeTab === 'analytics'" class="animate-[fadeIn_0.3s_ease-out]">
          <div class="mb-4">
            <h2 class="text-lg font-bold text-[#2D3A34]">Finanz- & KI Insights</h2>
          </div>

          <div class="bg-gradient-to-br from-white to-[#D8F3DC] p-4 rounded-[20px] mb-5 shadow-sm border border-black/5">
            <div class="flex justify-between items-center mb-2">
              <h3 class="text-sm font-bold text-[#2D6A4F]">KI-Schnittstelle (Ollama)</h3>
              <span class="text-[10px] bg-[#2D6A4F] text-white px-2 py-0.5 rounded-full">Qwen2.5-VL ready</span>
            </div>
            <p class="text-xs text-[#2D3A34] leading-relaxed">Dein lokales Modell hat diesen Monat 14 Belege analysiert und automatisch kategorisiert. Keine Cloud, 100% privat.</p>
          </div>

          <div class="flex justify-around items-end h-40 bg-white rounded-[20px] p-5 shadow-sm border border-black/5">
            <div class="w-11 bg-[#52B788] rounded-t-lg flex items-end justify-center pb-2 transition-all duration-500" style="height: 80%"><span class="text-[9px] text-white [writing-mode:vertical-lr] rotate-180">Rewe</span></div>
            <div class="w-11 bg-[#52B788] rounded-t-lg flex items-end justify-center pb-2 transition-all duration-500" style="height: 45%"><span class="text-[9px] text-white [writing-mode:vertical-lr] rotate-180">Lidl</span></div>
            <div class="w-11 bg-[#52B788] rounded-t-lg flex items-end justify-center pb-2 transition-all duration-500" style="height: 95%"><span class="text-[9px] text-white [writing-mode:vertical-lr] rotate-180">Bio-Markt</span></div>
            <div class="w-11 bg-[#52B788] rounded-t-lg flex items-end justify-center pb-2 transition-all duration-500" style="height: 30%"><span class="text-[9px] text-white [writing-mode:vertical-lr] rotate-180">Sonstige</span></div>
          </div>
        </div>

        <div v-if="activeTab === 'receipt' && receiptData" class="animate-[fadeIn_0.3s_ease-out]">
          <div class="mb-4">
            <h2 class="text-lg font-bold text-[#2D3A34]">Neuer Beleg</h2>
            <p class="text-xs text-[#6B7C74] font-medium">{{ receiptData.retailer?.value || 'Unbekannter Händler' }} • {{ receiptData.date?.value || 'Datum fehlt' }}</p>
          </div>

          <div class="bg-white rounded-[20px] p-4 shadow-sm border border-black/5 mb-5">
            <div class="space-y-3">
              <div v-for="(item, index) in receiptData.articles" :key="index" class="flex justify-between items-start border-b border-zinc-100 pb-3 last:border-0 last:pb-0">
                <div class="flex-1 pr-3">
                  <div class="text-sm font-semibold text-[#2D3A34] mb-1">
                    {{ item.quantity }}x {{ item.name }}
                  </div>

                  <div class="flex items-center">
                    <span v-if="item.ean" class="text-[10px] font-mono bg-zinc-100 text-zinc-500 px-1.5 py-0.5 rounded">
                      EAN: {{ item.ean }}
                    </span>
                    <button v-else @click="triggerEanScan(index, item.name)" class="bg-[#CCFF33] hover:bg-[#bceb2e] text-[#1B4332] border border-[#B7E4C7] px-2 py-1 rounded flex items-center gap-1 text-[10px] font-bold shadow-sm active:scale-95 transition-all">
                      <svg xmlns="http://www.w3.org/2000/svg" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M3 7V5a2 2 0 0 1 2-2h2"/><path d="M17 3h2a2 2 0 0 1 2 2v2"/><path d="M21 17v2a2 2 0 0 1-2 2h-2"/><path d="M7 21H5a2 2 0 0 1-2-2v-2"/><rect x="7" y="7" width="10" height="10" rx="1"/></svg>
                      EAN scannen
                    </button>
                  </div>
                </div>
                <div class="text-sm font-bold text-[#2D3A34] pt-0.5">
                  {{ formatPrice(item.price_total) }}
                </div>
              </div>
            </div>

            <div class="mt-4 pt-3 border-t-2 border-dashed border-zinc-200 flex justify-between items-center">
              <span class="font-bold text-[#2D3A34] text-sm">Gesamtsumme</span>
              <span class="font-black text-[#2D6A4F] text-lg">{{ formatPrice(receiptData.total_amount?.value || 0) }}</span>
            </div>
          </div>

          <button @click="saveAndCloseReceipt" class="w-full bg-[#2D6A4F] text-white font-bold py-3.5 rounded-[16px] shadow-md active:scale-[0.98] transition-transform">
            In Vorratskammer übernehmen
          </button>
        </div>

      </main>

      <input type="file" accept="image/*" capture="environment" ref="fileInput" @change="handleFileUpload" class="hidden" />
      <input type="file" accept="image/*" capture="environment" ref="eanInput" @change="handleEanUpload" class="hidden" />

      <transition
        enter-active-class="transition duration-300 ease-out"
        enter-from-class="opacity-0 backdrop-blur-0"
        enter-to-class="opacity-100 backdrop-blur-md"
        leave-active-class="transition duration-200 ease-in"
        leave-from-class="opacity-100 backdrop-blur-md"
        leave-to-class="opacity-0 backdrop-blur-0"
      >
        <div v-if="showOverlay" class="absolute inset-0 bg-[#1A241F]/90 z-50 flex flex-col items-center justify-center p-6 text-white text-center">

          <div class="relative w-24 h-24 mb-6 flex items-center justify-center">
            <div class="absolute inset-0 border-4 border-white/10 rounded-full"></div>
            <div class="absolute inset-0 border-4 border-t-[#CCFF33] border-r-transparent border-b-transparent border-l-transparent rounded-full animate-spin"></div>
            <img src="/icon.gif" alt="Bonsai Processing" class="w-12 h-12 object-contain opacity-80" />
          </div>

          <h3 class="text-xl font-bold text-[#CCFF33] mb-2">Bonsai Vision Engine</h3>

          <div class="bg-white/5 border border-white/10 px-4 py-2.5 rounded-xl text-xs font-mono text-zinc-300 max-w-[280px] mb-8">
            <span v-if="scanStep === 1">📤 Foto wird hochgeladen...</span>
            <span v-if="scanStep === 2">🧠 Ollama analysiert Beleg...</span>
            <span v-if="scanStep === 3">✅ Auswertung abgeschlossen!</span>
          </div>

          <button @click="showOverlay = false" class="bg-white/10 hover:bg-white/15 border border-white/20 text-white px-6 py-2.5 rounded-xl text-xs font-semibold tracking-wide transition-colors">
            Im Hintergrund weiterlaufen lassen
          </button>
        </div>
      </transition>

      <nav class="absolute bottom-0 left-0 right-0 h-[76px] bg-white/85 backdrop-blur-xl border-t border-black/5 grid grid-cols-[1fr_80px_1fr] items-center justify-items-center pb-[env(safe-area-inset-bottom)] z-10">
        <button @click="activeTab = 'home'" :class="['bg-none border-0 flex flex-col items-center gap-1 w-full transition-colors duration-200', activeTab === 'home' ? 'text-[#2D6A4F]' : 'text-[#6B7C74]']">
          <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="w-5 h-5"><path d="m3 9 9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z"/><polyline points="9 22 9 12 15 12 15 22"/></svg>
          <span class="text-[11px] font-semibold">Pantry</span>
        </button>

        <div class="relative w-[70px] h-[70px] -mt-9">
          <button @click="triggerCamera" class="w-[66px] h-[66px] rounded-full bg-gradient-to-br from-[#CCFF33] to-[#52B788] border-4 border-[#F4F7F5] shadow-[0_8px_25px_rgba(82,183,136,0.4)] flex items-center justify-center text-zinc-900 cursor-pointer active:scale-90 active:translate-y-1 transition-all duration-200" aria-label="Kassenzettel scannen">
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round" class="w-6 h-6"><path d="M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z"/><polyline points="3.27 6.96 12 12.01 20.73 6.96"/><line x1="12" y1="22.08" x2="12" y2="12"/></svg>
          </button>
        </div>

        <button @click="activeTab = 'analytics'" :class="['bg-none border-0 flex flex-col items-center gap-1 w-full transition-colors duration-200', activeTab === 'analytics' ? 'text-[#2D6A4F]' : 'text-[#6B7C74]']">
          <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="w-5 h-5"><line x1="18" y1="20" x2="18" y2="10"/><line x1="12" y1="20" x2="12" y2="4"/><line x1="6" y1="20" x2="6" y2="14"/></svg>
          <span class="text-[11px] font-semibold">Budget</span>
        </button>
      </nav>

    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue'

const activeTab = ref('home')
const scanStep = ref(0)
const receiptData = ref(null)

const showOverlay = ref(false)
const isProcessingInBackground = ref(false)

const fileInput = ref(null)
const eanInput = ref(null)
const currentTargetEanIndex = ref(null)

const pantryItems = ref([
  { id: 1, name: 'Frische Bio-Avocado', category: 'Gemüse & Früchte', emoji: '🥑', stock: 90, stockColor: '#52B788', daysLeft: '3 Tage', status: 'urgent' },
  { id: 2, name: 'Hafermilch (Barista)', category: 'Getränke', emoji: '🥛', stock: 40, stockColor: '#D8F3DC', daysLeft: '12 Tage', status: 'good' },
  { id: 3, name: 'Cherrytomaten', category: 'Gemüse & Früchte', emoji: '🍅', stock: 15, stockColor: '#FF4D4D', daysLeft: '2 Tage', status: 'urgent' },
  { id: 4, name: 'Vollkornnudeln', category: 'Pasta & Getreide', emoji: '🍝', stock: 100, stockColor: '#52B788', daysLeft: '180 Tage', status: 'infinite' },
  { id: 5, name: 'Griechischer Joghurt', category: 'Milchprodukte', emoji: '🥣', stock: 75, stockColor: '#74C69D', daysLeft: '6 Tage', status: 'warn' },
  { id: 6, name: 'Eier (Freiland)', category: 'Frischeprodukte', emoji: '🥚', stock: 60, stockColor: '#74C69D', daysLeft: '8 Tage', status: 'good' },
])

const formatPrice = (price) => {
  if (typeof price !== 'number') return price;
  return new Intl.NumberFormat('de-DE', { style: 'currency', currency: 'EUR' }).format(price);
};

const triggerCamera = () => {
  if (fileInput.value) fileInput.value.click()
}

const handleFileUpload = async (event) => {
  const file = event.target.files[0];
  if (!file) return;

  showOverlay.value = true;
  isProcessingInBackground.value = true;
  scanStep.value = 1;
  receiptData.value = null;

  try {
    scanStep.value = 2;

    const response = await fetch('http://192.168.188.176:3000/api/upload-receipt', {
      method: 'POST',
      headers: { 'Content-Type': file.type },
      body: file,
    });

    if (!response.ok) {
      const errorText = await response.text().catch(() => response.statusText);
      throw new Error(`Server error (${response.status}): ${errorText}`);
    }

    const data = await response.json();
    receiptData.value = data;
    scanStep.value = 3;

    setTimeout(() => {
      showOverlay.value = false;
      isProcessingInBackground.value = false;
      activeTab.value = 'receipt';
    }, 800);

  } catch (error) {
    console.error('Error while uploading: ', error);
    alert(error.message);
    showOverlay.value = false;
    isProcessingInBackground.value = false;
  } finally {
    event.target.value = '';
  }
};

const triggerEanScan = (index, articleName) => {
  currentTargetEanIndex.value = index;
  if (eanInput.value) {
    eanInput.value.click();
  }
};

const handleEanUpload = async (event) => {
  const file = event.target.files[0];
  if (!file || currentTargetEanIndex.value === null) return;

  try {
    const response = await fetch('http://192.168.188.176:3000/api/scan-barcode', {
      method: 'POST',
      headers: { 'Content-Type': file.type },
      body: file,
    });

    if (response.ok) {
      const data = await response.json();
      receiptData.value.articles[currentTargetEanIndex.value].ean = data.ean;
    } else {
      receiptData.value.articles[currentTargetEanIndex.value].ean = "400123456789";
    }
  } catch (err) {
    console.error("Barcode-Scan fehlgeschlagen, benutze Entwickler-Fallback:", err);
    receiptData.value.articles[currentTargetEanIndex.value].ean = "400999999999";
  } finally {
    currentTargetEanIndex.value = null;
    event.target.value = '';
  }
};

const saveAndCloseReceipt = () => {
  receiptData.value = null;
  activeTab.value = 'home';
};
</script>

<style>
@keyframes fadeIn {
  from { opacity: 0; transform: translateY(8px); }
  to { opacity: 1; transform: translateY(0); }
}
</style>
