<script setup>
defineProps({
  totalAmount: Number,
  scannedTotalAmount: Number,
  itemCount: Number,
  hasDuplicateEan: Boolean,
  hasAmountMismatch: Boolean,
  isFormInvalid: Boolean,
  isSubmitting: Boolean,
  isMissingEan: Boolean,
  isPriceInvalid: Boolean,
  isStoreInvalid: Boolean,
  isDateInvalid: Boolean,
  formatPrice: Function
})

const emit = defineEmits(['save'])
</script>

<template>
  <div class="bg-gradient-to-br from-[#2D6A4F] to-[#1B4332] rounded-[24px] p-5 text-white space-y-4 shadow-md">
    <div class="flex justify-between items-center">
      <div>
        <span class="text-[10px] text-white/70 uppercase tracking-wider block font-medium">Berechnete Summe</span>
        <span class="text-2xl font-black font-mono tracking-tight">{{ formatPrice(totalAmount) }}</span>
      </div>
      <span class="text-xs bg-white/10 px-3 py-1 rounded-full text-[#CCFF33] font-bold">
        {{ itemCount }} Positionen
      </span>
    </div>

    <div v-if="hasDuplicateEan" class="bg-red-500/20 text-red-300 border border-red-500/30 rounded-xl p-3 text-xs font-semibold flex items-start gap-2.5 animate-[fadeIn_0.2s_ease-out]">
      <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round" class="text-red-400 shrink-0 mt-0.5">
        <circle cx="12" cy="12" r="10"/><line x1="12" y1="8" x2="12" y2="12"/><line x1="12" y1="16" x2="12.01" y2="16"/>
      </svg>
      <div>
        <span class="font-bold block text-white">Doppelte Barcodes!</span>
        <span class="opacity-90">Einige Artikel besitzen dieselbe EAN. Jeder Artikel muss eindeutig sein (gelb markiert).</span>
      </div>
    </div>

    <div v-if="hasAmountMismatch && !hasDuplicateEan" class="bg-amber-500/20 text-amber-300 border border-amber-500/30 rounded-xl p-3 text-xs font-semibold flex items-start gap-2.5 animate-[fadeIn_0.2s_ease-out]">
      <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5" stroke-linecap="round" stroke-linejoin="round" class="text-amber-400 shrink-0 mt-0.5">
        <path d="m21.73 18-8-14a2 2 0 0 0-3.48 0l-8 14A2 2 0 0 0 4 21h16a2 2 0 0 0 1.73-3Z"/><line x1="12" y1="9" x2="12" y2="13"/><line x1="12" y1="17" x2="12.01" y2="17"/>
      </svg>
      <div>
        <span class="font-bold block text-white">Betrag weicht ab!</span>
        <span class="opacity-90">Gescanntes Total vom Beleg: {{ formatPrice(scannedTotalAmount) }}</span>
      </div>
    </div>

    <button
      type="button"
      @click="emit('save')"
      :disabled="isFormInvalid || isSubmitting"
      :class="[
        'w-full font-bold py-4 px-4 rounded-xl text-sm shadow-lg transition-all duration-200 min-h-[48px]',
        (isFormInvalid || isSubmitting)
          ? 'bg-zinc-700/50 text-zinc-400 border border-zinc-600/30 cursor-not-allowed opacity-60'
          : 'bg-[#CCFF33] hover:bg-[#b2eb2a] text-zinc-900 cursor-pointer active:scale-[0.99]'
      ]"
    >
      <template v-if="isSubmitting">Wird gespeichert...</template>
      <template v-else-if="isMissingEan">Bitte erst alle Barcodes scannen</template>
      <template v-else-if="hasDuplicateEan">Doppelte Barcodes korrigieren</template>
      <template v-else-if="isPriceInvalid">Preis/Summe darf nicht 0 € sein</template>
      <template v-else-if="isStoreInvalid">Händler-Namen korrigieren</template>
      <template v-else-if="isDateInvalid">Gültiges Belegdatum angeben</template>
      <template v-else>Daten bestätigen & Einbuchen</template>
    </button>
  </div>
</template>
