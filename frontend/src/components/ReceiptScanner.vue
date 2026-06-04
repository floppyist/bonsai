<script setup>
import { ref } from 'vue';

const isLoading = ref(false);
const receiptData = ref(null);

const formatPrice = (price) => {
  if (typeof price !== 'number') return price;

  return new Intl.NumberFormat('de-DE', { style: 'currency', currency: 'EUR' }).format(price);
};

const handleFileUpload = async (event) => {
  const file = event.target.files[0];

  if (!file) return;

  isLoading.value = true;
  receiptData.value = null;

  try {
    const response = await fetch('http://192.168.188.176:3000/api/upload-receipt', {
      method: 'POST',
      headers: {
        'Content-Type': file.type,
      },
      body: file,
    });

    if (!response.ok) {
      const errorText = await response.text().catch(() => response.statusText);

      throw new Error(`Server error (${response.status}): ${errorText}`);
    }

    const data = await response.json();

    receiptData.value = data;
  } catch (error) {
    console.error('Error while uploading: ', error);

    alert(error.message);
  } finally {
    isLoading.value = false;
  }
};

const resetScanner = () => {
  receiptData.value = null;
};
</script>

<template>
  <div class="scanner-wrapper flex flex-col h-screen justify-center items-center">
    <h1 class="text-4xl text-sky-100 font-bold">Receipt Scanner</h1>

    <div v-if="!isLoading && !receiptData" class="upload-section w-full flex justify-center flex-col">
      <label class="bg-sky-700 hover:bg-sky-500 text-white font-bold m-5 rounded text-center p-3">
        Capture photo / Upload
        <input
          type="file"
          accept="image/*"
          capture="environment"
          @change="handleFileUpload"
          style="display: none;"
        />
      </label>
    </div>

    <div v-if="isLoading" class="loading-section">
      <div class="spinner"></div>
      <p class="text-white">Image is being analyzed...</p>
    </div>

    <div v-if="receiptData" class="receipt-card">
      <div class="receipt-header">
        <h2>{{ receiptData.retailer.value }}</h2>
        <h3>{{ receiptData.date.value }}</h3>
      </div>

      <div class="receipt-body">
        <table>
          <thead>
            <tr>
              <th>Quantity</th>
              <th>Article</th>
              <th class="text-right">Price</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="(item, index) in receiptData.articles" :key="index">
              <td>{{ item.quantity }}x</td>
              <td>{{ item.name }}</td>
              <td class="text-right">{{ formatPrice(item.price_total) }}</td>
            </tr>
          </tbody>
        </table>
      </div>

      <div class="receipt-footer">
        <div class="total-row">
          <span>TOTAL AMOUNT</span>
          <span>{{ formatPrice(receiptData.total_amount.value) }}</span>
        </div>
      </div>

      <button @click="resetScanner" class="reset-button">Scan new receipt</button>
    </div>
  </div>
</template>

<style scoped>
.scanner-wrapper {
  max-width: 400px;
  margin: 0 auto;
  font-family: 'Courier New', Courier, monospace;
  padding: 20px;
}

.hint {
  text-align: center;
  font-size: 0.6em;
  color: #999;
  font-family: sans-serif;
}

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
