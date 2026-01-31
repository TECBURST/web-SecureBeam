<script setup lang="ts">
import { RouterLink } from 'vue-router'
import { Sun, Moon } from 'lucide-vue-next'
import { useThemeStore } from '@/stores/theme'

const themeStore = useThemeStore()
</script>

<template>
  <header class="bg-white/90 dark:bg-neutral-900/90 backdrop-blur-md border-b border-neutral-100 dark:border-neutral-800 sticky top-0 z-50">
    <div class="container mx-auto px-4 sm:px-6 py-3 sm:py-4 flex items-center justify-between max-w-4xl">
      <!-- Logo -->
      <RouterLink to="/" class="flex items-center gap-2 sm:gap-3 group">
        <!-- Ring Logo -->
        <div class="w-7 h-7 sm:w-8 sm:h-8 flex items-center justify-center">
          <svg viewBox="0 0 32 32" class="w-7 h-7 sm:w-8 sm:h-8">
            <circle cx="16" cy="16" r="12" fill="none" stroke="currentColor" stroke-width="4" class="text-neutral-900 dark:text-white"/>
          </svg>
        </div>
        <span class="text-base sm:text-lg font-semibold tracking-tight text-neutral-900 dark:text-white">SecureBeam</span>
      </RouterLink>

      <!-- Theme Toggle -->
      <button
        @click="themeStore.toggleTheme()"
        class="p-2 rounded-lg text-neutral-600 dark:text-neutral-400 hover:text-neutral-900 dark:hover:text-white hover:bg-neutral-100 dark:hover:bg-neutral-800 transition-colors"
        :title="themeStore.theme === 'dark' ? 'Switch to light mode' : 'Switch to dark mode'"
      >
        <Moon v-if="themeStore.theme === 'light'" class="w-5 h-5" />
        <Sun v-else class="w-5 h-5" />
      </button>
    </div>
  </header>
</template>
