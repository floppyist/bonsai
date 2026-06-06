<script setup>
defineProps({
  isProcessingInBackground: Boolean
})
</script>

<template>
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
</template>
