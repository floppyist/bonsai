<script setup>
import { inject } from 'vue'

const isDevMode = inject('isDevMode')
</script>

<template>
  <div class="animate-[fadeIn_0.3s_ease-out]">
    <div class="mb-4">
      <h2 class="text-lg font-bold text-[#2D3A34]">Optionen</h2>
      <p class="text-xs text-[#6B7C74] font-medium">Konfiguration & Systemstatus</p>
    </div>

    <div class="space-y-4">
      <div class="bg-white rounded-[20px] p-5 shadow-sm border border-black/5 flex items-center gap-4">
        <div class="w-12 h-12 rounded-full bg-gradient-to-br from-[#B7E4C7] to-[#74C69D] flex items-center justify-center text-xl shadow-inner">👤</div>
        <div>
          <h3 class="text-sm font-bold text-[#2D3A34]">Lokaler Haushalt</h3>
          <p class="text-[11px] text-[#6B7C74]">Daten werden nur lokal gespeichert</p>
        </div>
      </div>

      <div class="bg-white rounded-[20px] p-5 shadow-sm border border-black/5">
        <h3 class="text-xs font-bold text-[#2D6A4F] uppercase tracking-wider mb-3">Erscheinungsbild</h3>
        <div class="space-y-1">
          <div class="flex items-center justify-between py-3 border-b border-zinc-50">
            <span class="text-sm font-medium text-[#2D3A34]">Dunkelmodus</span>
            <button class="w-10 h-6 bg-zinc-200 rounded-full relative"><div class="absolute left-1 top-1 w-4 h-4 bg-white rounded-full shadow-sm"></div></button>
          </div>
          <div class="flex items-center justify-between py-3">
            <span class="text-sm font-medium text-[#2D3A34]">Kompakt-Modus</span>
            <button class="w-10 h-6 bg-[#2D6A4F] rounded-full relative"><div class="absolute right-1 top-1 w-4 h-4 bg-white rounded-full shadow-sm"></div></button>
          </div>
        </div>
      </div>

      <div class="bg-white rounded-[20px] p-5 shadow-sm border border-black/5">
        <h3 class="text-xs font-bold text-[#2D6A4F] uppercase tracking-wider mb-3">KI-Engine Status</h3>
        <div class="space-y-3">
          <div class="flex justify-between items-center"><span class="text-xs text-[#6B7C74]">Vision Modell</span><span class="text-xs font-mono bg-zinc-100 px-2 py-0.5 rounded text-[#2D3A34]">qwen2.5vl:3b</span></div>
          <div class="flex justify-between items-center">
            <span class="text-xs text-[#6B7C74]">Ollama Status</span>
            <span class="flex items-center gap-1.5 text-xs text-emerald-600 font-bold"><span class="w-1.5 h-1.5 bg-emerald-500 rounded-full animate-pulse"></span>Bereit</span>
          </div>
          <div class="flex justify-between items-center"><span class="text-xs text-[#6B7C74]">Backend</span><span class="text-xs text-zinc-400 font-medium">bonsai.pi:3000 (Axum)</span></div>
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

      <div class="text-center pt-4">
        <p class="text-[10px] text-zinc-400 font-semibold tracking-wide">BONSAI v1.0.0-ALPHA</p>
        <p class="text-[9px] text-zinc-300 italic mt-1">Lokal & Sicher. Keine Cloud-Anbindung.</p>
      </div>
    </div>
  </div>
</template>
