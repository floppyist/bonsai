<script setup>
import Footer from './components/Footer.vue';
import Navbar from './components/Navbar.vue';
import ReceiptScanner from './components/ReceiptScanner.vue';
</script>

<template>
  <main class="flex flex-col h-dvh bg-black">
    <Navbar></Navbar>
    <ReceiptScanner></ReceiptScanner>
    <Footer></Footer>
  </main>
</template>
