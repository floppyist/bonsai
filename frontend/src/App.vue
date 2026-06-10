<script setup>
import { ref, provide } from 'vue'
import { useRouter } from 'vue-router'
import { useBonsaiStore } from './stores/BonsaiStore.js'

import { BrowserMultiFormatReader, BarcodeFormat, DecodeHintType } from '@zxing/library'

import PantryHeader from './components/PantryHeader.vue'
import VisionOverlay from './components/VisionOverlay.vue'
import EanScannerModal from './components/modals/EanScannerModal.vue'
import BottomNav from './components/BottomNav.vue'

const router = useRouter();
const store = useBonsaiStore();

const scanStep = ref(0)
const receiptData = ref(null)
const showOverlay = ref(false)
const isProcessingInBackground = ref(false)
const fileInput = ref(null)

const showEanModal = ref(false)
const currentTargetEanIndex = ref(null)
const currentTargetEanName = ref('')

let codeReaderInstance = null
let codeReaderControls = null

const isDevMode = ref(false)
provide('isDevMode', isDevMode)

const pantryItems = ref([
  { id: 1, name: 'Frische Bio-Avocado', category: 'Gemüse & Früchte', emoji: '🥑', stock: 90, stockColor: '#52B788', daysLeft: '3 Tage', status: 'urgent' },
  { id: 2, name: 'Hafermilch (Barista)', category: 'Getränke', emoji: '🥛', stock: 40, stockColor: '#D8F3DC', daysLeft: '12 Tage', status: 'good' },
  { id: 3, name: 'Cherrytomaten', category: 'Gemüse & Früchte', emoji: '🍅', stock: 15, stockColor: '#FF4D4D', daysLeft: '2 Tage', status: 'urgent' },
  { id: 4, name: 'Vollkornnudeln', category: 'Pasta & Getreide', emoji: '🍝', stock: 100, stockColor: '#52B788', daysLeft: '180 Tage', status: 'infinite' },
  { id: 5, name: 'Griechischer Joghurt', category: 'Milchprodukte', emoji: '🥣', stock: 75, stockColor: '#74C69D', daysLeft: '6 Tage', status: 'warn' },
  { id: 6, name: 'Eier (Freiland)', category: 'Frischeprodukte', emoji: '🥚', stock: 60, stockColor: '#74C69D', daysLeft: '8 Tage', status: 'good' },
])

const MOCK_RECEIPT_DATA = {
  id: 'mock-dev-123',
  store_name: 'Bio-Markt Bonsai (Mock)',
  date: new Date().toISOString().split('T')[0],
  total_amount: 24.95,
  items: [
    { name: 'Bio-Hafermilch 1L', quantity: 2, price: 1.99 },
    { name: 'Avocado Premium', quantity: 1, price: 2.49 },
    { name: 'Dinkelbrot 500g', quantity: 1, price: 3.89 },
    { name: 'Kaffeebohnen Arabica 1kg', quantity: 1, price: 14.59 }
  ]
}

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

if (isDevMode.value) {
  setTimeout(() => {
    store.setReceiptData({
      articles: MOCK_RECEIPT_DATA.items.map(item => ({
        name: item.name,
        raw_ocr_name: item.name, // <-- Neu: Als Startwert mitschleifen
        quantity: item.quantity,
        price: item.price,
        ean: null
      }))
    });
    scanStep.value = 3;

      setTimeout(() => {
        showOverlay.value = false;
        isProcessingInBackground.value = false;
        router.push({ name: 'receipt' });
      }, 800);
    }, 1000);

    event.target.value = '';
    return;
  }

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
      store.setReceiptData(payload);
      scanStep.value = 3;

      setTimeout(() => {
        showOverlay.value = false;
        isProcessingInBackground.value = false;
        router.push({ name: 'receipt' });
      }, 800);
    }
    worker.terminate();
  };

  event.target.value = '';
};

const startEanScanner = (index) => {
  currentTargetEanIndex.value = index;
  currentTargetEanName.value = store.currentReceipt.articles[index]?.name || '';
  showEanModal.value = true;
};

const initializeQrScanner = async (videoElement) => {
  try {
    const hints = new Map();
    hints.set(DecodeHintType.POSSIBLE_FORMATS, [BarcodeFormat.EAN_13, BarcodeFormat.EAN_8]);

    codeReaderInstance = new BrowserMultiFormatReader(hints);
    codeReaderInstance.timeBetweenDecodingAttempts = 100;

    const constraints = {
      video: {
        facingMode: "environment",
        width: { ideal: 640 },
        height: { ideal: 480 }
      }
    };

    codeReaderControls = await codeReaderInstance.decodeFromConstraints(
      constraints,
      videoElement,
      (result, err) => {
        if (result) {
          const scannedEan = result.getText();
          if (!scannedEan) return;

          const activeIndex = currentTargetEanIndex.value;

          stopEanScanner();

          setTimeout(() => {
            store.lookupEanForArticle(activeIndex, scannedEan);
          }, 50);
        }
      }
    );
  } catch (err) {
    console.error("Kamera-Setup fehlgeschlagen:", err);
    alert(`Fehler beim Starten der Kamera: ${err.message}`);
    showEanModal.value = false;
  }
};

const stopEanScanner = () => {
  showEanModal.value = false;
  currentTargetEanIndex.value = null;
  currentTargetEanName.value = '';

  if (codeReaderControls) {
    codeReaderControls.stop();
    codeReaderControls = null;
  }

  if (codeReaderInstance) {
    try {
      codeReaderInstance.reset();
    } catch (e) {
      console.warn("ZXing reset fehler:", e);
    }
    codeReaderInstance = null;
  }

  const videoElement = document.querySelector('video');
  if (videoElement && videoElement.srcObject) {
    const stream = videoElement.srcObject;
    const tracks = stream.getTracks();
    tracks.forEach(track => track.stop());
    videoElement.srcObject = null;
  }
};

const saveAndCloseReceipt = () => {
  receiptData.value = null;
  router.push({ name: 'pantry' });
};
</script>

<template>
  <div class="w-full h-screen bg-zinc-200 flex justify-center items-center m-0 p-0 antialiased selection:bg-[#CCFF33] selection:text-zinc-900">
    <div class="relative w-full h-full bg-[#F4F7F5] shadow-2xl flex flex-col overflow-hidden">

      <PantryHeader
        :isProcessingInBackground="isProcessingInBackground"
        @bringOverlayToForeground="showOverlay = true"
      />

      <main class="flex-1 overflow-y-auto p-5 pb-28 scrollbar-thin scrollbar-thumb-zinc-300">
        <router-view v-slot="{ Component }">
          <component
            :is="Component"
            ref="receiptViewRef"
            :pantryItems="pantryItems"
            :formatPrice="formatPrice"
            @startEanScanner="startEanScanner"
            @saveAndCloseReceipt="saveAndCloseReceipt"
          />
        </router-view>
      </main>

      <input type="file" accept="image/*" ref="fileInput" @change="handleFileUpload" class="hidden" />
      <VisionOverlay :showOverlay="showOverlay" :scanStep="scanStep" @closeOverlay="showOverlay = false" />

      <EanScannerModal
        :showEanModal="showEanModal"
        :currentTargetEanName="currentTargetEanName"
        @stopEanScanner="stopEanScanner"
        @videoReady="initializeQrScanner"
      />

      <BottomNav @triggerCamera="triggerCamera" />

    </div>
  </div>
</template>

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
