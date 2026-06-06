<script setup>
import { ref } from 'vue'
import QrScanner from 'qr-scanner'

import PantryHeader from './components/PantryHeader.vue'
import PantryTab from './components/PantryTab.vue'
import BudgetTab from './components/BudgetTab.vue'
import ReceiptTab from './components/ReceiptTab.vue'
import VisionOverlay from './components/VisionOverlay.vue'
import EanScannerModal from './components/modals/EanScannerModal.vue'
import BottomNav from './components/BottomNav.vue'

const activeTab = ref('home')
const scanStep = ref(0)
const receiptData = ref(null)
const showOverlay = ref(false)
const isProcessingInBackground = ref(false)
const fileInput = ref(null)

const showEanModal = ref(false)
const currentTargetEanIndex = ref(null)
const currentTargetEanName = ref('')
let qrScannerInstance = null

const pantryItems = ref([
  { id: 1, name: 'Frische Bio-Avocado', category: 'Gemüse & Früchte', emoji: '🥑', stock: 90, stockColor: '#52B788', daysLeft: '3 Tage', status: 'urgent' },
  { id: 2, name: 'Hafermilch (Barista)', category: 'Getränke', emoji: '🥛', stock: 40, stockColor: '#D8F3DC', daysLeft: '12 Tage', status: 'good' },
  { id: 3, name: 'Cherrytomaten', category: 'Gemüse & Früchte', emoji: '🍅', stock: 15, stockColor: '#FF4D4D', daysLeft: '2 Tage', status: 'urgent' },
  { id: 4, name: 'Vollkornnudeln', category: 'Pasta & Getreide', emoji: '🍝', stock: 100, stockColor: '#52B788', daysLeft: '180 Tage', status: 'infinite' },
  { id: 5, name: 'Griechischer Joghurt', category: 'Milchprodukte', emoji: '🥣', stock: 75, stockColor: '#74C69D', daysLeft: '6 Tage', status: 'warn' },
  { id: 6, name: 'Eier (Freiland)', category: 'Frischeprodukte', emoji: '🥚', stock: 60, stockColor: '#74C69D', daysLeft: '8 Tage', status: 'good' },
])

const formatPrice = (price) => {
  if (typeof price !== 'number') return price;
  return new Intl.NumberFormat('de-DE', { style: 'currency', currency: 'EUR' }).format(price);
};

const triggerCamera = () => {
  if (fileInput.value) fileInput.value.click()
}

const handleFileUpload = (event) => {
  const file = event.target.files[0];
  if (!file) return;

  showOverlay.value = true;
  isProcessingInBackground.value = true;
  scanStep.value = 1;
  receiptData.value = null;
  scanStep.value = 2;

  const worker = new Worker(new URL('./workers/receiptWorker.js', import.meta.url), { type: 'module' });

  worker.postMessage({
    file: file,
    url: 'https://bonsai.pi:3000/api/upload-receipt'
  });

  worker.onmessage = (e) => {
    const { payload, error } = e.data;

    if (error) {
      console.error('Error while uploading receipt: ', error);
      alert(error);
      showOverlay.value = false;
      isProcessingInBackground.value = false;
    } else {
      receiptData.value = payload;
      scanStep.value = 3;

      setTimeout(() => {
        showOverlay.value = false;
        isProcessingInBackground.value = false;
        activeTab.value = 'receipt';
      }, 800);
    }
    worker.terminate();
  };

  event.target.value = '';
};

const startEanScanner = (index) => {
  currentTargetEanIndex.value = index;
  currentTargetEanName.value = receiptData.value.articles[index].name;
  showEanModal.value = true;
};

const initializeQrScanner = async (videoElement) => {
  let detailsText = "";

  try {
    detailsText += `[1/4] Prüfe Verfügbarkeit von navigator.mediaDevices...\n`;
    if (!navigator.mediaDevices) {
      throw new Error("navigator.mediaDevices ist undefined! (Unsicherer HTTP-Kontext auf iOS?)");
    }

    detailsText += `[2/4] Instanziiere QrScanner...\n`;
    qrScannerInstance = new QrScanner(
      videoElement,
      (result) => {
        if (result && result.data) {
          receiptData.value.articles[currentTargetEanIndex.value].ean = result.data;
          stopEanScanner();
        }
      },
      {
        onDecodeError: () => {},
        maxScansPerSecond: 20,
        highlightScanRegion: false,
        highlightCodeOutline: false,
        returnDetailedScanResult: true,
        constraints: {
          video: { facingMode: { exact: "environment" } }
        }
      }
    );

    detailsText += `[3/4] Starte QrScanner (Rückkamera)...\n`;
    await qrScannerInstance.start();

  } catch (err) {
    detailsText += `❌ Fehler im ersten Versuch: Name=${err.name}, Message=${err.message}\n`;
    detailsText += `[4/4] Weiche auf Fallback-Kamera-Konfiguration aus...\n`;

    try {
      if (qrScannerInstance) qrScannerInstance.destroy();

      qrScannerInstance = new QrScanner(
        videoElement,
        (result) => {
          if (result && result.data) {
            receiptData.value.articles[currentTargetEanIndex.value].ean = result.data;
            stopEanScanner();
          }
        },
        {
          onDecodeError: () => {},
          maxScansPerSecond: 20
        }
      );
      await qrScannerInstance.start();
    } catch (fallbackErr) {
      detailsText += `❌ Fataler Fallback-Fehler: Name=${fallbackErr.name}, Message=${fallbackErr.message}\n`;
      detailsText += `Stack: ${fallbackErr.stack || 'Kein Stacktrace'}`;

      console.error("Kompletter Kamera-Fehler-Log:", detailsText);
      alert(`DIAGNOSE-LOG:\n\n${detailsText}`);
      showEanModal.value = false;
    }
  }
};

const stopEanScanner = () => {
  showEanModal.value = false;
  currentTargetEanIndex.value = null;
  currentTargetEanName.value = '';

  if (qrScannerInstance) {
    qrScannerInstance.destroy();
    qrScannerInstance = null;
  }
};

const saveAndCloseReceipt = () => {
  receiptData.value = null;
  activeTab.value = 'home';
};
</script>

<style>
@keyframes fadeIn {
from { opacity: 0; transform: translateY(8px); }
to { opacity: 1; transform: translateY(0); }
}

@keyframes scanLine {
0% { top: 0%; opacity: 0.3; }
50% { top: 100%; opacity: 1; }
100% { top: 0%; opacity: 0.3; }
}
</style>

<template>
  <div class="w-full h-full bg-zinc-200 flex justify-center items-center m-0 p-0 antialiased selection:bg-[#CCFF33] selection:text-zinc-900">
    <div class="relative w-full h-screen bg-[#F4F7F5] shadow-2xl flex flex-col overflow-hidden">

      <PantryHeader :isProcessingInBackground="isProcessingInBackground" />

      <main class="flex-1 overflow-y-auto p-5 pb-28 scrollbar-thin scrollbar-thumb-zinc-300">
        <PantryTab v-if="activeTab === 'home'" :pantryItems="pantryItems" />
        <BudgetTab v-if="activeTab === 'analytics'" />
        <ReceiptTab
          v-if="activeTab === 'receipt' && receiptData"
          :receiptData="receiptData"
          :formatPrice="formatPrice"
          @startEanScanner="startEanScanner"
          @saveAndCloseReceipt="saveAndCloseReceipt"
        />
      </main>

      <input type="file" accept="image/*" ref="fileInput" @change="handleFileUpload" class="hidden" />

      <VisionOverlay :showOverlay="showOverlay" :scanStep="scanStep" @closeOverlay="showOverlay = false" />

      <EanScannerModal
        :showEanModal="showEanModal"
        :currentTargetEanName="currentTargetEanName"
        @stopEanScanner="stopEanScanner"
        @videoReady="initializeQrScanner"
      />

      <BottomNav :activeTab="activeTab" @updateTab="activeTab = $event" @triggerCamera="triggerCamera" />

    </div>
  </div>
</template>
