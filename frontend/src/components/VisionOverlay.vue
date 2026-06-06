<script setup>
defineProps({
  showOverlay: Boolean,
  scanStep: Number
})

defineEmits(['closeOverlay'])
</script>

<template>
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
      <button @click="$emit('closeOverlay')" class="bg-white/10 hover:bg-white/15 border border-white/20 text-white px-6 py-2.5 rounded-xl text-xs font-semibold tracking-wide transition-colors">
        Im Hintergrund weiterlaufen lassen
      </button>
    </div>
  </transition>
</template>
