<script setup>
import { ref, inject, onMounted } from 'vue'

const isDevMode = inject('isDevMode')
const ollamaStatus = ref('Prüfe…')
const ollamaColor = ref('text-zinc-400')
const ollamaDot = ref('bg-zinc-400')

const apiTarget = import.meta.env.VITE_API_TARGET || 'https://localhost:3000'

onMounted(async () => {
  try {
    const res = await fetch('/api/health')

    if (res.ok) {
      ollamaStatus.value = 'Bereit'
      ollamaColor.value = 'text-emerald-600'
      ollamaDot.value = 'bg-emerald-500'
    } else throw new Error()
  } catch {
    ollamaStatus.value = 'Nicht verfügbar'
    ollamaColor.value = 'text-red-500'
    ollamaDot.value = 'bg-red-500'
  }
})
</script>

<template>
  <div class="space-y-4">
    <div class="bg-white rounded-[20px] p-5 shadow-sm border border-black/5">
      <h3 class="text-xs font-bold text-[#2D6A4F] uppercase tracking-wider mb-3">KI-Engine Status</h3>
      <div class="space-y-3">
        <div class="flex justify-between items-center">
          <span class="text-xs text-[#6B7C74]">Vision Modell</span>
          <span class="text-xs font-mono bg-zinc-100 px-2 py-0.5 rounded text-[#2D3A34]">qwen2.5vl:3b</span>
        </div>
        <div class="flex justify-between items-center">
          <span class="text-xs text-[#6B7C74]">Ollama Status</span>
          <span :class="['flex items-center gap-1.5 text-xs font-bold', ollamaColor]">
            <span :class="['w-1.5 h-1.5 rounded-full', ollamaDot]"></span>{{ ollamaStatus }}
          </span>
        </div>
        <div class="flex justify-between items-center">
          <span class="text-xs text-[#6B7C74]">Backend</span>
          <span v-if="isDevMode" class="text-xs text-zinc-400 font-medium">
            {{ apiTarget }} (Axum)
          </span>
        </div>
      </div>
    </div>

    <div class="bg-white rounded-[20px] p-5 shadow-sm border border-black/5">
      <h3 class="text-xs font-bold text-amber-600 uppercase tracking-wider mb-3">Developer Tools</h3>
      <div class="flex items-center justify-between py-1">
        <div class="flex flex-col">
          <span class="text-sm font-medium text-[#2D3A34]">Dev-Mode</span>
          <span class="text-[11px] text-[#6B7C74]">Leitet Foto-Uploads direkt auf Test-Dummies um</span>
        </div>
        <button
          @click="isDevMode = !isDevMode"
          :class="['w-10 h-6 rounded-full relative transition-colors duration-200 cursor-pointer', isDevMode ? 'bg-amber-500' : 'bg-zinc-200']"
        >
          <div :class="['absolute top-1 w-4 h-4 bg-white rounded-full shadow-sm transition-all duration-200', isDevMode ? 'right-1' : 'left-1']"></div>
        </button>
      </div>
    </div>
  </div>
</template>
