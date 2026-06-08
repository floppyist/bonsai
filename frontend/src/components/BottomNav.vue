<script setup>
import { computed } from 'vue'
import { useRouter, useRoute } from 'vue-router'

defineEmits(['triggerCamera'])

const router = useRouter()
const route = useRoute()

const currentRouteName = computed(() => route.name)
const navigateTo = (routeName) => router.push({ name: routeName })
</script>

<template>
  <nav class="absolute bottom-0 left-0 right-0 h-[76px] bg-white/85 backdrop-blur-xl border-t border-black/5 grid grid-cols-[1fr_1fr_80px_1fr_1fr] items-center justify-items-center pb-[env(safe-area-inset-bottom)] z-10">

    <button @click="navigateTo('pantry')" :class="['bg-none border-0 flex flex-col items-center justify-center gap-1 w-full h-full transition-colors duration-200', currentRouteName === 'pantry' ? 'text-[#2D6A4F]' : 'text-[#6B7C74]']">
      <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="w-5 h-5"><path d="m3 9 9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z"/><polyline points="9 22 9 12 15 12 15 22"/></svg>
      <span class="text-[10px] font-semibold">Pantry</span>
    </button>

    <button @click="navigateTo('receipts')" :class="['bg-none border-0 flex flex-col items-center justify-center gap-1 w-full h-full transition-colors duration-200', currentRouteName === 'receipts' ? 'text-[#2D6A4F]' : 'text-[#6B7C74]']">
      <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="w-5 h-5"><path d="M4 2v20l2-1 2-1 2-1 2-1 2-1 2-1 2-1 2-1V2l-2 1-2-1-2 1-2-1-2 1-2-1-2 1Z"/><path d="M16 8h-6"/><path d="M16 12h-6"/><path d="M16 16h-6"/></svg>
      <span class="text-[10px] font-semibold">Belege</span>
    </button>

    <div class="relative w-[60px] h-[60px] flex items-center justify-center -mt-8">
      <button @click="$emit('triggerCamera')" class="w-[60px] h-[60px] rounded-full bg-gradient-to-br from-[#CCFF33] to-[#52B788] border-4 border-[#F4F7F5] shadow-[0_8px_25px_rgba(82,183,136,0.3)] flex items-center justify-center text-zinc-900 cursor-pointer active:scale-90 active:translate-y-1 transition-all duration-200" aria-label="Scan">
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round" class="w-6 h-6"><path d="M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z"/><polyline points="3.27 6.96 12 12.01 20.73 6.96"/><line x1="12" y1="22.08" x2="12" y2="12"/></svg>
      </button>
    </div>

    <button @click="navigateTo('budget')" :class="['bg-none border-0 flex flex-col items-center justify-center gap-1 w-full h-full transition-colors duration-200', currentRouteName === 'budget' ? 'text-[#2D6A4F]' : 'text-[#6B7C74]']">
      <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="w-5 h-5"><line x1="18" y1="20" x2="18" y2="10"/><line x1="12" y1="20" x2="12" y2="4"/><line x1="6" y1="20" x2="6" y2="14"/></svg>
      <span class="text-[10px] font-semibold">Budget</span>
    </button>

    <button @click="navigateTo('settings')" :class="['bg-none border-0 flex flex-col items-center justify-center gap-1 w-full h-full transition-colors duration-200', currentRouteName === 'settings' ? 'text-[#2D6A4F]' : 'text-[#6B7C74]']">
      <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="w-5 h-5"><path d="M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.08a2 2 0 0 1-1-1.74v-.5a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2Z"/><circle cx="12" cy="12" r="3"/></svg>
      <span class="text-[10px] font-semibold">Optionen</span>
    </button>
  </nav>
</template>
