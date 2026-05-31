<template>
  <div class="scanner-wrapper">
    <h1>Kassenbon Scanner</h1>

    <div v-if="!isLoading && !receiptData" class="upload-section">
      <label class="camera-button">
        📸 Foto aufnehmen / hochladen
        <input
          type="file"
          accept="image/*"
          capture="environment"
          @change="handleFileUpload"
          style="display: none;"
        />
      </label>
      <p class="hint">Die Verarbeitung durch KI kann einen Moment dauern.</p>
    </div>

    <div v-if="isLoading" class="loading-section">
      <div class="spinner"></div>
      <p>Bild wird analysiert... (Das kann auf der CPU dauern ☕)</p>
    </div>

    <div v-if="receiptData" class="receipt-card">
      <div class="receipt-header">
        <h2>{{ receiptData.haendler || 'Unbekannter Händler' }}</h2>
      </div>

      <div class="receipt-body">
        <table>
          <thead>
            <tr>
              <th>Menge</th>
              <th>Artikel</th>
              <th class="text-right">Preis</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="(item, index) in receiptData.artikel" :key="index">
              <td>{{ item.menge }}x</td>
              <td>{{ item.name }}</td>
              <td class="text-right">{{ formatPrice(item.preis_gesamt) }}</td>
            </tr>
          </tbody>
        </table>
      </div>

      <div class="receipt-footer">
        <div class="total-row">
          <span>GESAMTBETRAG</span>
          <span>{{ formatPrice(receiptData.gesamtbetrag) }}</span>
        </div>
      </div>

      <button @click="resetScanner" class="reset-button">Neuen Bon scannen</button>
    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue';

const isLoading = ref(false);
const receiptData = ref(null);

// Preis-Formatierung (z.B. 1.99 -> 1,99 €)
const formatPrice = (price) => {
  if (typeof price !== 'number') return price;
  return new Intl.NumberFormat('de-DE', { style: 'currency', currency: 'EUR' }).format(price);
};

// Datei-Upload ans Rust-Backend
const handleFileUpload = async (event) => {
  const file = event.target.files[0];
  if (!file) return;

  isLoading.value = true;
  receiptData.value = null;

  const formData = new FormData();
  formData.append('image', file);

  try {
    const response = await fetch('http://192.168.188.176:3000/api/upload-receipt', {
      method: 'POST',
      body: formData,
    });

    if (!response.ok) {
      // Versucht den exakten Fehlertext aus der main.rs zu lesen (z.B. Python-Fehler)
      const errorText = await response.text().catch(() => response.statusText);
      throw new Error(`Serverfehler (${response.status}): ${errorText}`);
    }

    const data = await response.json();
    receiptData.value = data;

  } catch (error) {
    console.error('Fehler beim Upload:', error);
    // Zeigt den detaillierten Fehler als Popup auf dem Smartphone an
    alert(error.message);
  } finally {
    isLoading.value = false;
  }
};

const resetScanner = () => {
  receiptData.value = null;
};
</script>

<style scoped>
.scanner-wrapper {
  max-width: 400px;
  margin: 0 auto;
  font-family: 'Courier New', Courier, monospace; /* Typischer Kassenbon-Look */
  padding: 20px;
}

h1 {
  text-align: center;
  font-family: sans-serif;
}

/* Button Styling */
.camera-button {
  display: block;
  background-color: #4CAF50;
  color: white;
  padding: 15px 20px;
  text-align: center;
  border-radius: 8px;
  font-weight: bold;
  font-family: sans-serif;
  cursor: pointer;
  margin-top: 20px;
  box-shadow: 0 4px 6px rgba(0,0,0,0.1);
}

.hint {
  text-align: center;
  font-size: 0.8em;
  color: #666;
  font-family: sans-serif;
}

/* Spinner Animation */
.spinner {
  border: 4px solid #f3f3f3;
  border-top: 4px solid #3498db;
  border-radius: 50%;
  width: 40px;
  height: 40px;
  animation: spin 1s linear infinite;
  margin: 20px auto;
}
@keyframes spin {
  0% { transform: rotate(0deg); }
  100% { transform: rotate(360deg); }
}

.loading-section {
  text-align: center;
  font-family: sans-serif;
}

/* Receipt Card (Thermopapier-Look) */
.receipt-card {
  background-color: #fff;
  border: 1px solid #ddd;
  padding: 20px;
  box-shadow: 0px 5px 15px rgba(0,0,0,0.05);
  position: relative;
  margin-top: 20px;
  color: #333;
}

.receipt-header {
  text-align: center;
  border-bottom: 2px dashed #ccc;
  padding-bottom: 10px;
  margin-bottom: 10px;
}

.receipt-body table {
  width: 100%;
  border-collapse: collapse;
}

.receipt-body th, .receipt-body td {
  padding: 8px 0;
  text-align: left;
  border-bottom: 1px dotted #eee;
}

.text-right {
  text-align: right !important;
}

.receipt-footer {
  border-top: 2px dashed #ccc;
  margin-top: 15px;
  padding-top: 10px;
  font-weight: bold;
  font-size: 1.2em;
}

.total-row {
  display: flex;
  justify-content: space-between;
}

.reset-button {
  width: 100%;
  margin-top: 20px;
  padding: 10px;
  background-color: #333;
  color: white;
  border: none;
  cursor: pointer;
  font-family: sans-serif;
  border-radius: 4px;
}
</style>
