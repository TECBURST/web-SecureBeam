<script setup lang="ts">
import Header from '@/components/layout/Header.vue'
import Footer from '@/components/layout/Footer.vue'
import { useThemeStore } from '@/stores/theme'

// Initialize theme store to apply theme on app load
useThemeStore()
</script>

<template>
  <div class="min-h-screen flex flex-col bg-white dark:bg-neutral-950 transition-colors duration-200">
    <Header />

    <main class="flex-1 flex flex-col">
      <RouterView />
    </main>

    <Footer />
  </div>
</template>
