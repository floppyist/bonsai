<script setup>
import { RouterLink, RouterView } from 'vue-router'
</script>

<template>
  <header>
    <div class="wrapper">
      <div class="brand">
        <h2>🌿 Bonsai</h2>
      </div>

      <nav>
        <RouterLink to="/">Home</RouterLink>
        <RouterLink to="/scan">📸 Bon scannen</RouterLink>
        <RouterLink to="/about">About</RouterLink>
      </nav>
    </div>
  </header>

  <main class="content-container">
    <RouterView />
  </main>
</template>

<style scoped>
header {
  line-height: 1.5;
  max-height: 100vh;
  padding: 1rem;
  border-bottom: 1px solid var(--color-border);
}

.brand h2 {
  margin: 0;
  text-align: center;
}

nav {
  width: 100%;
  font-size: 14px;
  text-align: center;
  margin-top: 1rem;
}

nav a.router-link-exact-active {
  color: #2e7d32; /* Passend zu deiner PWA theme_color */
  font-weight: bold;
}

nav a {
  display: inline-block;
  padding: 0 1rem;
  border-left: 1px solid var(--color-border);
  text-decoration: none;
  color: var(--color-text);
}

nav a:first-of-type {
  border: 0;
}

.content-container {
  padding: 20px;
  min-height: calc(100vh - 100px);
}

@media (min-width: 1024px) {
  header {
    display: flex;
    place-items: center;
    justify-content: space-between;
    padding: 1rem 2rem;
  }

  .brand h2 {
    text-align: left;
  }

  nav {
    text-align: right;
    margin-top: 0;
    font-size: 1rem;
  }
}
</style>
