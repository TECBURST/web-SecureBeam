<script setup lang="ts">
import { ref } from 'vue'
import { RouterLink } from 'vue-router'
import { useI18n } from 'vue-i18n'
import { languages, setLocale } from '@/i18n'
import { ChevronDown, Sun, Moon } from 'lucide-vue-next'
import FlagIcon from '@/components/ui/FlagIcon.vue'
import { useThemeStore } from '@/stores/theme'

const { locale } = useI18n()
const themeStore = useThemeStore()

const showLanguageMenu = ref(false)

function selectLanguage(code: string) {
  setLocale(code)
  showLanguageMenu.value = false
}
</script>

<template>
  <header class="bg-white/90 dark:bg-neutral-900/90 backdrop-blur-md border-b border-neutral-100 dark:border-neutral-800 sticky top-0 z-50">
    <div class="container mx-auto px-4 sm:px-6 py-3 sm:py-4 flex items-center justify-between max-w-5xl">
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

      <!-- Controls -->
      <div class="flex items-center gap-2">
        <!-- Theme Toggle -->
        <button
          @click="themeStore.toggleTheme()"
          class="p-2 rounded-lg text-neutral-600 dark:text-neutral-400 hover:text-neutral-900 dark:hover:text-white hover:bg-neutral-100 dark:hover:bg-neutral-800 transition-colors"
          :title="themeStore.theme === 'dark' ? 'Switch to light mode' : 'Switch to dark mode'"
        >
          <Moon v-if="themeStore.theme === 'light'" class="w-5 h-5" />
          <Sun v-else class="w-5 h-5" />
        </button>

        <!-- Language Dropdown -->
        <div class="relative" @click.stop>
          <button
            @click="showLanguageMenu = !showLanguageMenu"
            class="flex items-center gap-1.5 sm:gap-2 text-sm font-medium text-neutral-600 dark:text-neutral-400 hover:text-neutral-900 dark:hover:text-white transition-colors px-2.5 sm:px-3 py-2 rounded-lg hover:bg-neutral-100 dark:hover:bg-neutral-800 border border-neutral-200 dark:border-neutral-700"
          >
            <FlagIcon :code="locale" size="sm" />
            <ChevronDown class="w-3 h-3 transition-transform" :class="{ 'rotate-180': showLanguageMenu }" />
          </button>

          <!-- Dropdown Menu -->
          <Transition
            enter-active-class="transition ease-out duration-100"
            enter-from-class="transform opacity-0 scale-95"
            enter-to-class="transform opacity-100 scale-100"
            leave-active-class="transition ease-in duration-75"
            leave-from-class="transform opacity-100 scale-100"
            leave-to-class="transform opacity-0 scale-95"
          >
            <div
              v-if="showLanguageMenu"
              class="absolute right-0 mt-2 w-40 sm:w-44 bg-white dark:bg-neutral-800 rounded-xl shadow-lg border border-neutral-200 dark:border-neutral-700 py-1 z-50"
            >
              <button
                v-for="lang in languages"
                :key="lang.code"
                @click="selectLanguage(lang.code)"
                class="w-full px-3 sm:px-4 py-2.5 text-sm text-left flex items-center gap-2.5 sm:gap-3 hover:bg-neutral-100 dark:hover:bg-neutral-700 transition-colors text-neutral-700 dark:text-neutral-300 hover:text-neutral-900 dark:hover:text-white"
                :class="{ 'bg-neutral-100 dark:bg-neutral-700 font-semibold text-neutral-900 dark:text-white': locale === lang.code }"
              >
                <FlagIcon :code="lang.code" size="sm" />
                <span>{{ lang.name }}</span>
              </button>
            </div>
          </Transition>
        </div>
      </div>
    </div>
  </header>

  <!-- Click outside to close dropdown -->
  <Teleport to="body">
    <div
      v-if="showLanguageMenu"
      class="fixed inset-0 z-40"
      @click="showLanguageMenu = false"
    ></div>
  </Teleport>
</template>
