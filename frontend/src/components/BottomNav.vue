<script setup>
import { computed } from 'vue'
import { useRouter, useRoute } from 'vue-router'

defineEmits(['triggerCamera'])

const router = useRouter()
const route = useRoute()

const currentRouteName = computed(() => route.name)

const navigateTo = (routeName) => {
  router.push({ name: routeName })
}
</script>

<template>
  <nav class="absolute bottom-0 left-0 right-0 h-[76px] bg-white/85 backdrop-blur-xl border-t border-black/5 grid grid-cols-[1fr_80px_1fr] items-center justify-items-center pb-[env(safe-area-inset-bottom)] z-10">

    <button @click="navigateTo('pantry')" :class="['bg-none border-0 flex flex-col items-center gap-1 w-full transition-colors duration-200', currentRouteName === 'pantry' ? 'text-[#2D6A4F]' : 'text-[#6B7C74]']">
      <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="w-5 h-5"><path d="m3 9 9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z"/><polyline points="9 22 9 12 15 12 15 22"/></svg>
      <span class="text-[11px] font-semibold">Pantry</span>
    </button>

    <div class="relative w-[70px] h-[70px] -mt-9">
      <button @click="$emit('triggerCamera')" class="w-[66px] h-[66px] rounded-full bg-gradient-to-br from-[#CCFF33] to-[#52B788] border-4 border-[#F4F7F5] shadow-[0_8px_25px_rgba(82,183,136,0.4)] flex items-center justify-center text-zinc-900 cursor-pointer active:scale-90 active:translate-y-1 transition-all duration-200" aria-label="Kassenzettel scannen">
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="w-6 h-6"><path d="M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z"/><polyline points="3.27 6.96 12 12.01 20.73 6.96"/><line x1="12" y1="22.08" x2="12" y2="12"/></svg>
      </button>
    </div>

    <button @click="navigateTo('budget')" :class="['bg-none border-0 flex flex-col items-center gap-1 w-full transition-colors duration-200', currentRouteName === 'budget' ? 'text-[#2D6A4F]' : 'text-[#6B7C74]']">
      <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="w-5 h-5"><line x1="18" y1="20" x2="18" y2="10"/><line x1="12" y1="20" x2="12" y2="4"/><line x1="6" y1="20" x2="6" y2="14"/></svg>
      <span class="text-[11px] font-semibold">Budget</span>
    </button>
  </nav>
</template>
