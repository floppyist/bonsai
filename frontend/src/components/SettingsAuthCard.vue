<script setup>
import { ref } from 'vue'
import { useBonsaiStore } from '../stores/BonsaiStore'

const store = useBonsaiStore()

const isLoginMode = ref(true)
const username = ref('')
const password = ref('')
const errorMessage = ref('')
const successMessage = ref('')
const isSubmitting = ref(false)

const handleSubmit = async () => {
  if (!username.value.trim() || !password.value.trim()) {
    errorMessage.value = 'Bitte alle Felder ausfüllen.'
    return
  }

  errorMessage.value = ''
  successMessage.value = ''
  isSubmitting.value = true

  try {
    if (isLoginMode.value) {
      const success = await store.login(username.value, password.value)
      if (success) {
        username.value = ''
        password.value = ''
      } else {
        errorMessage.value = 'Login fehlgeschlagen. Zugangsdaten prüfen.'
      }
    } else {
      const success = await store.register(username.value, password.value)
      if (success) {
        successMessage.value = 'Registrierung erfolgreich! Du kannst dich jetzt einloggen.'
        isLoginMode.value = true
        password.value = ''
      } else {
        errorMessage.value = 'Registrierung fehlgeschlagen. Name evtl. vergeben.'
      }
    }
  } catch (err) {
    errorMessage.value = 'Ein Fehler ist aufgetreten.'
  } finally {
    isSubmitting.value = false
  }
}
</script>

<template>
  <div class="bg-white rounded-[20px] p-5 shadow-sm border border-black/5 space-y-4">
    <div class="flex items-center gap-4">
      <div class="w-12 h-12 rounded-full bg-gradient-to-br from-[#B7E4C7] to-[#74C69D] flex items-center justify-center text-xl shadow-inner">
        👤
      </div>
      <div class="flex-1">
        <h3 class="text-sm font-bold text-[#2D3A34]">
          {{ store.user ? `Haushalt: ${store.user.username}` : 'Nicht angemeldet' }}
        </h3>
        <p class="text-[11px] text-[#6B7C74]">
          {{ store.user ? 'Daten sind an dein Konto gebunden' : 'Melde dich an, um deine Vorräte zu verwalten' }}
        </p>
      </div>

      <button
        v-if="store.user"
        @click="store.logout()"
        class="text-xs font-bold text-red-500 hover:text-red-600 bg-red-50 px-3 py-1.5 rounded-xl transition-colors cursor-pointer"
      >
        Abmelden
      </button>
    </div>

    <div v-if="!store.user" class="pt-2 border-t border-zinc-50 space-y-3">
      <div class="flex gap-4 border-b border-zinc-100 pb-2">
        <button
          @click="isLoginMode = true; errorMessage = ''; successMessage = ''"
          :class="['text-xs font-bold pb-1 cursor-pointer', isLoginMode ? 'text-[#2D6A4F] border-b-2 border-[#2D6A4F]' : 'text-zinc-400']"
        >
          Anmelden
        </button>
        <button
          @click="isLoginMode = false; errorMessage = ''; successMessage = ''"
          :class="['text-xs font-bold pb-1 cursor-pointer', !isLoginMode ? 'text-[#2D6A4F] border-b-2 border-[#2D6A4F]' : 'text-zinc-400']"
        >
          Registrieren
        </button>
      </div>

      <form @submit.prevent="handleSubmit" class="space-y-2.5">
        <div>
          <input
            v-model="username"
            type="text"
            placeholder="Nutzername"
            class="w-full text-xs px-3 py-2 rounded-xl bg-zinc-50 border border-zinc-100 focus:outline-none focus:border-[#74C69D] transition-colors"
            :disabled="isSubmitting"
          />
        </div>
        <div>
          <input
            v-model="password"
            type="password"
            placeholder="Passwort"
            class="w-full text-xs px-3 py-2 rounded-xl bg-zinc-50 border border-zinc-100 focus:outline-none focus:border-[#74C69D] transition-colors"
            :disabled="isSubmitting"
          />
        </div>

        <p v-if="errorMessage" class="text-[11px] text-red-500 font-medium">{{ errorMessage }}</p>
        <p v-if="successMessage" class="text-[11px] text-emerald-600 font-medium">{{ successMessage }}</p>

        <button
          type="submit"
          class="w-full bg-[#2D6A4F] text-white text-xs font-bold py-2 rounded-xl hover:bg-[#1B4332] transition-colors shadow-sm disabled:opacity-50 cursor-pointer"
          :disabled="isSubmitting"
        >
          {{ isSubmitting ? 'Bitte warten…' : (isLoginMode ? 'Einloggen' : 'Konto erstellen') }}
        </button>
      </form>
    </div>
  </div>
</template>
