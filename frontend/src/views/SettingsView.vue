<script setup>
import { version } from '../../package.json'
import SettingsAuthCard from '../components/SettingsAuthCard.vue'
import SettingsSystemStatus from '../components/SettingsSystemStatus.vue'
</script>

<template>
  <div class="animate-[fadeIn_0.3s_ease-out] space-y-4">
    <div>
      <h2 class="text-lg font-bold text-[#2D3A34]">Optionen</h2>
      <p class="text-xs text-[#6B7C74] font-medium">Konfiguration & Systemstatus</p>
    </div>

    <SettingsAuthCard />

    <div class="bg-white rounded-[20px] p-5 shadow-sm border border-black/5">
      <h3 class="text-xs font-bold text-[#2D6A4F] uppercase tracking-wider mb-3">Erscheinungsbild</h3>
      <div class="space-y-1">
        <div class="flex items-center justify-between py-3 border-b border-zinc-50 opacity-40">
          <span class="text-sm font-medium text-[#2D3A34]">Dunkelmodus <span class="text-[10px] text-zinc-400 font-normal">(Bald)</span></span>
          <span class="w-10 h-6 bg-zinc-200 rounded-full block"></span>
        </div>
        <div class="flex items-center justify-between py-3 opacity-40">
          <span class="text-sm font-medium text-[#2D3A34]">Kompakt-Modus <span class="text-[10px] text-zinc-400 font-normal">(Bald)</span></span>
          <span class="w-10 h-6 bg-zinc-200 rounded-full block"></span>
        </div>
      </div>
    </div>

    <SettingsSystemStatus />

    <div class="text-center pt-4">
      <p class="text-[10px] text-zinc-400 font-semibold tracking-wide">BONSAI v{{ version }}</p>
      <p class="text-[9px] text-zinc-300 italic mt-1">Lokal & Sicher. Keine Cloud-Anbindung.</p>
    </div>
  </div>
</template>
