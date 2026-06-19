import { fileURLToPath, URL } from 'node:url'
import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import eslintPlugin from 'vite-plugin-eslint'
import { VitePWA } from 'vite-plugin-pwa'
import tailwindcss from '@tailwindcss/vite'
import fs from 'node:fs'
import path from 'node:path'

const __dirname = fileURLToPath(new URL('.', import.meta.url))

const certPath = path.resolve(__dirname, '../backend/certs/server.crt')
const keyPath = path.resolve(__dirname, '../backend/certs/server.key')

export default defineConfig({
  plugins: [
    vue(),
    eslintPlugin({ cache: false }),
    VitePWA({
      registerType: 'autoUpdate',
      devOptions: {
        enabled: false,
      },
      includeAssets: ['logo.png', 'icon.gif'],
      manifest: {
        name: 'Bonsai - Pantry & Budget',
        short_name: 'Bonsai',
        description: 'Keep your pantry and purchases perfectly trimmed.',
        theme_color: '#2e7d32',
        background_color: '#000000',
        display: 'standalone',
        icons: [
          {
            src: 'logo.png',
            sizes: '512x512',
            type: 'image/png',
            purpose: 'any'
          }
        ]
      }
    }),
    tailwindcss(),
  ],
  server: {
    host: true,
    https: {
      key: fs.readFileSync(keyPath),
      cert: fs.readFileSync(certPath),
    },
    hmr: {
      host: process.env.VITE_HOST || 'localhost',
      protocol: 'wss',
    },
    proxy: {
      '/api': {
        target: process.env.VITE_API_TARGET || 'https://localhost:3000',
        changeOrigin: true,
        secure: false
      },
      '/avatars': {
        target: process.env.VITE_API_TARGET || 'https://localhost:3000',
        changeOrigin: true,
        secure: false,
      }
    }
  },
  preview: {
    host: true,
    https: {
      key: fs.readFileSync(keyPath),
      cert: fs.readFileSync(certPath),
    }
  },
  resolve: {
    alias: {
      '@': fileURLToPath(new URL('./src', import.meta.url))
    }
  }
})
