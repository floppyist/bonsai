<template>
  <div class="w-full h-full bg-zinc-200 flex justify-center items-center m-0 p-0 antialiased selection:bg-[#CCFF33] selection:text-zinc-900">

    <div class="relative w-full max-w-md h-screen max-h-[932px] bg-[#F4F7F5] md:rounded-[40px] md:border-[8px] md:border-zinc-800 shadow-2xl flex flex-col overflow-hidden">

      <header class="pt-15 bg-gradient-to-br from-[#2D6A4F] to-[#1B4332] px-5 pt-6 pb-8 text-white rounded-b-[28px] shrink-0">
        <div class="flex justify-between items-center mb-5">
          <div class="flex items-center gap-2">
            <span class="w-2.5 height-2.5 w-2.5 h-2.5 bg-[#CCFF33] rounded-full shadow-[0_0_10px_#CCFF33]"></span>
            <h1 class="text-2xl font-extrabold tracking-tight">Bonsai</h1>
          </div>
          <div class="bg-white/10 backdrop-blur-md px-3 py-1.5 rounded-full text-xs flex gap-1">
            <span class="text-zinc-300">Pantry Status:</span>
            <span class="text-[#CCFF33] font-bold">Optimiert</span>
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

      </main>

      <transition
        enter-active-class="transition duration-350 ease-out"
        enter-from-class="opacity-0 scale-105"
        enter-to-class="opacity-100 scale-100"
        leave-active-class="transition duration-350 ease-in"
        leave-from-class="opacity-100 scale-100"
        leave-to-class="opacity-0 scale-105"
      >
        <div v-if="isScanning" class="absolute inset-0 bg-[#1A241F]/95 z-50 flex flex-col items-center justify-between px-6 py-10 text-white">
          <div class="text-center">
            <h3 class="text-xl font-bold text-[#CCFF33] mb-1.5">Bonsai Vision Engine</h3>
            <p class="text-xs text-white/70">Halte den Kassenzettel oder EAN-Code in die Kamera</p>
          </div>

          <div class="relative w-[260px] h-[380px] rounded-3xl bg-white/5 shadow-[inset_0_0_40px_rgba(0,0,0,0.5)] overflow-hidden">
            <div class="absolute left-0 right-0 h-1 bg-gradient-to-b from-transparent via-[#CCFF33] to-transparent animate-[laserMove_2.5s_infinite_linear]"></div>

            <div class="absolute w-6 h-6 border-4 border-[#CCFF33] top-4 left-4 border-r-0 border-b-0 rounded-tl-xl"></div>
            <div class="absolute w-6 h-6 border-4 border-[#CCFF33] top-4 right-4 border-left-0 border-b-0 rounded-tr-xl"></div>
            <div class="absolute w-6 h-6 border-4 border-[#CCFF33] bottom-4 left-4 border-r-0 border-t-0 rounded-bl-xl"></div>
            <div class="absolute w-6 h-6 border-4 border-[#CCFF33] bottom-4 right-4 border-left-0 border-t-0 rounded-br-xl"></div>

            <div class="absolute bottom-5 left-3 right-3 bg-black/75 border border-white/10 p-2.5 rounded-xl text-[11px] flex items-center gap-2 backdrop-blur-sm animate-[fadeIn_0.3s_ease]" v-if="scanStep > 0">
              <span class="w-2 h-2 bg-[#CCFF33] rounded-full animate-pulse shadow-[0_0_8px_#CCFF33]"></span>
              <span v-if="scanStep === 1">OpenCV: Kanten werden isoliert...</span>
              <span v-if="scanStep === 2">Ollama: Analysiere Artikel (qwen2.5vl)...</span>
              <span v-if="scanStep === 3">Erfolgreich! Importiere in Pantry...</span>
            </div>
          </div>

          <button @click="stopScanning" class="bg-white/10 hover:bg-white/20 border-0 text-white px-8 py-3 rounded-full text-sm font-semibold cursor-pointer transition-colors">Abbrechen</button>
        </div>
      </transition>

      <nav class="absolute bottom-0 left-0 right-0 h-[76px] bg-white/85 backdrop-blur-xl border-t border-black/5 grid grid-cols-[1fr_80px_1fr] items-center justify-items-center pb-[env(safe-area-inset-bottom)] z-10">
        <button @click="activeTab = 'home'" :class="['bg-none border-0 flex flex-col items-center gap-1 w-full transition-colors duration-200', activeTab === 'home' ? 'text-[#2D6A4F]' : 'text-[#6B7C74]']">
          <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="w-5 h-5"><path d="m3 9 9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z"/><polyline points="9 22 9 12 15 12 15 22"/></svg>
          <span class="text-[11px] font-semibold">Pantry</span>
        </button>

        <div class="relative w-[70px] h-[70px] -mt-9">
          <button @click="startScanning" class="w-[66px] h-[66px] rounded-full bg-gradient-to-br from-[#CCFF33] to-[#52B788] border-4 border-[#F4F7F5] shadow-[0_8px_25px_rgba(82,183,136,0.4)] flex items-center justify-center text-zinc-900 cursor-pointer active:scale-90 active:translate-y-1 transition-all duration-200" aria-label="Kassenzettel scannen">
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
const isScanning = ref(false)
const scanStep = ref(0)

const pantryItems = ref([
  { id: 1, name: 'Frische Bio-Avocado', category: 'Gemüse & Früchte', emoji: '🥑', stock: 90, stockColor: '#52B788', daysLeft: '3 Tage', status: 'urgent' },
  { id: 2, name: 'Hafermilch (Barista)', category: 'Getränke', emoji: '🥛', stock: 40, stockColor: '#D8F3DC', daysLeft: '12 Tage', status: 'good' },
  { id: 3, name: 'Cherrytomaten', category: 'Gemüse & Früchte', emoji: '🍅', stock: 15, stockColor: '#FF4D4D', daysLeft: '2 Tage', status: 'urgent' },
  { id: 4, name: 'Vollkornnudeln', category: 'Pasta & Getreide', emoji: '🍝', stock: 100, stockColor: '#52B788', daysLeft: '180 Tage', status: 'infinite' },
  { id: 5, name: 'Griechischer Joghurt', category: 'Milchprodukte', emoji: '🥣', stock: 75, stockColor: '#74C69D', daysLeft: '6 Tage', status: 'warn' },
  { id: 6, name: 'Eier (Freiland)', category: 'Frischeprodukte', emoji: '🥚', stock: 60, stockColor: '#74C69D', daysLeft: '8 Tage', status: 'good' },
])

let t1, t2, t3

const startScanning = () => {
  isScanning.value = true
  scanStep.value = 0

  t1 = setTimeout(() => { scanStep.value = 1 }, 1200)
  t2 = setTimeout(() => { scanStep.value = 2 }, 2800)
  t3 = setTimeout(() => {
    scanStep.value = 3
    pantryItems.value.unshift({
      id: Date.now(),
      name: 'Frisches Basilikum',
      category: 'Kräuter',
      emoji: '🌿',
      stock: 100,
      stockColor: '#52B788',
      daysLeft: '7 Tage',
      status: 'good'
    })
    setTimeout(() => { isScanning.value = false }, 1000)
  }, 4800)
}

const stopScanning = () => {
  isScanning.value = false
  scanStep.value = 0
  clearTimeout(t1)
  clearTimeout(t2)
  clearTimeout(t3)
}
</script>

<style>
@keyframes laserMove {
  0% { top: 0%; }
  50% { top: 100%; }
  100% { top: 0%; }
}
@keyframes fadeIn {
  from { opacity: 0; transform: translateY(8px); }
  to { opacity: 1; transform: translateY(0); }
}
</style>
