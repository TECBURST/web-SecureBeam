<script setup lang="ts">
import { ref } from 'vue'
import { RouterLink } from 'vue-router'
import { useI18n } from 'vue-i18n'
import { Sun, Moon, Settings, Globe, ChevronDown } from 'lucide-vue-next'
import { useThemeStore } from '@/stores/theme'
import { SUPPORTED_LOCALES, LOCALE_NAMES, setLocale, getCurrentLocale, type Locale } from '@/i18n'

const { t } = useI18n()
const themeStore = useThemeStore()

const showLanguageMenu = ref(false)
const currentLocale = ref<Locale>(getCurrentLocale())

function changeLanguage(locale: Locale) {
  currentLocale.value = locale
  setLocale(locale)
  showLanguageMenu.value = false
}

function toggleLanguageMenu() {
  showLanguageMenu.value = !showLanguageMenu.value
}

// Close menu when clicking outside
function closeLanguageMenu() {
  showLanguageMenu.value = false
}
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

      <!-- Right Side Actions -->
      <div class="flex items-center gap-1">
        <!-- Language Selector -->
        <div class="relative">
          <button
            @click="toggleLanguageMenu"
            @blur="closeLanguageMenu"
            class="flex items-center gap-1 px-2 py-2 rounded-lg text-neutral-600 dark:text-neutral-400 hover:text-neutral-900 dark:hover:text-white hover:bg-neutral-100 dark:hover:bg-neutral-800 transition-colors"
            :title="t('settings.language')"
          >
            <Globe class="w-5 h-5" />
            <span class="text-xs font-medium uppercase hidden sm:inline">{{ currentLocale }}</span>
            <ChevronDown class="w-3 h-3" />
          </button>

          <!-- Language Dropdown -->
          <div
            v-if="showLanguageMenu"
            class="absolute right-0 mt-1 py-1 w-36 bg-white dark:bg-neutral-800 rounded-lg shadow-lg border border-neutral-200 dark:border-neutral-700 z-50"
          >
            <button
              v-for="locale in SUPPORTED_LOCALES"
              :key="locale"
              @mousedown.prevent="changeLanguage(locale)"
              class="w-full px-3 py-2 text-left text-sm hover:bg-neutral-100 dark:hover:bg-neutral-700 transition-colors"
              :class="currentLocale === locale
                ? 'text-neutral-900 dark:text-white font-medium'
                : 'text-neutral-600 dark:text-neutral-400'"
            >
              {{ LOCALE_NAMES[locale] }}
            </button>
          </div>
        </div>

        <!-- Settings -->
        <RouterLink
          to="/settings"
          class="p-2 rounded-lg text-neutral-600 dark:text-neutral-400 hover:text-neutral-900 dark:hover:text-white hover:bg-neutral-100 dark:hover:bg-neutral-800 transition-colors"
          :title="t('nav.settings')"
        >
          <Settings class="w-5 h-5" />
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
    </div>
  </header>
</template>
