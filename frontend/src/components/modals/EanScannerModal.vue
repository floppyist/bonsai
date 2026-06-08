<script setup>
import { ref, watch, nextTick } from 'vue'

const props = defineProps({
  showEanModal: Boolean,
  currentTargetEanName: String
})

const emit = defineEmits(['stopEanScanner', 'videoReady'])
const videoElement = ref(null)

watch(() => props.showEanModal, async (newVal) => {
  if (newVal) {
    await nextTick()
    emit('videoReady', videoElement.value)
  }
})
</script>

<template>
  <transition
    enter-active-class="transition duration-300 ease-out"
    enter-from-class="opacity-0"
    enter-to-class="opacity-100"
    leave-active-class="transition duration-200 ease-in"
    leave-from-class="opacity-100"
    leave-to-class="opacity-0"
  >
    <div v-if="showEanModal" class="absolute inset-0 bg-[#1A241F] z-50 flex flex-col justify-between p-6 text-white">
      <div class="flex justify-between items-center mt-8">
        <div>
          <h3 class="text-lg font-bold text-[#CCFF33]">EAN Scanner</h3>
          <p class="text-xs text-zinc-400 mt-0.5 truncate max-w-[240px]">{{ currentTargetEanName }}</p>
        </div>
        <button @click="$emit('stopEanScanner')" class="bg-white/10 w-8 h-8 rounded-full flex items-center justify-center text-white font-bold text-sm active:scale-90 transition-transform">
          ✕
        </button>
      </div>

      <div class="relative w-full aspect-square max-w-[320px] mx-auto bg-black rounded-2xl overflow-hidden border-2 border-white/10 flex items-center justify-center">
        <video ref="videoElement" class="w-full h-full object-cover" autoplay playsinline muted></video>
        <div class="absolute inset-x-6 h-[2px] bg-[#CCFF33] shadow-[0_0_12px_#CCFF33] animate-[scanLine_2s_infinite_ease-in-out]"></div>
        <div class="absolute inset-6 border-2 border-dashed border-white/20 pointer-events-none rounded-xl"></div>
      </div>

      <div class="mb-10 text-center px-4">
        <p class="text-xs text-zinc-400 leading-relaxed">Positioniere den Barcode im markierten Bereich.</p>
      </div>
    </div>
  </transition>
</template>
