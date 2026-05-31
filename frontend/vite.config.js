import { fileURLToPath, URL } from 'node:url'
import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import { VitePWA } from 'vite-plugin-pwa'
import basicSsl from '@vitejs/plugin-basic-ssl'
import tailwindcss from '@tailwindcss/vite'

export default defineConfig({
  plugins: [
    vue(),
    // basicSsl(), // Prevents iOS from using the ICON (this is fixed if a real LetsEncrypt Cert is given later)
    VitePWA({
      registerType: 'autoUpdate',
      manifest: {
        name: 'Bonsai - Pantry & Budget',
        short_name: 'Bonsai',
        description: 'Keep your pantry and purchases perfectly trimmed.',
        theme_color: '#2e7d32',
        background_color: '#000000',
        display: 'standalone',
        icons: [
          {
            src: 'icon.png',
            sizes: '512x512',
            type: 'image/png',
            purpose: 'any'
          }
        ]
      }
    }),
    tailwindcss(),
  ],
  resolve: {
    alias: {
      '@': fileURLToPath(new URL('./src', import.meta.url))
    }
  }
})
