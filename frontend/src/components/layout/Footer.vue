<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import { RouterLink } from 'vue-router'
import { Lock, Zap, Globe } from 'lucide-vue-next'

const { t } = useI18n()
</script>

<template>
  <footer class="bg-neutral-50 dark:bg-neutral-900 border-t border-neutral-100 dark:border-neutral-800 py-5 sm:py-6">
    <div class="container mx-auto px-4 sm:px-6 max-w-5xl">
      <!-- Mobile Layout -->
      <div class="flex flex-col items-center gap-3 text-center md:hidden">
        <div class="flex items-center gap-2">
          <svg viewBox="0 0 32 32" class="w-4 h-4">
            <circle cx="16" cy="16" r="12" fill="none" stroke="currentColor" stroke-width="3" class="text-neutral-900 dark:text-white"/>
          </svg>
          <span class="text-sm font-medium text-neutral-900 dark:text-white">SecureBeam</span>
          <span class="text-neutral-300 dark:text-neutral-600">·</span>
          <span class="text-xs text-neutral-500 dark:text-neutral-400">securebeam.eu</span>
        </div>

        <div class="flex items-center gap-2.5 text-xs text-neutral-500 dark:text-neutral-400">
          <div class="flex items-center gap-1">
            <Lock class="w-3 h-3" />
            <span>{{ t('footer.encrypted') }}</span>
          </div>
          <span class="text-neutral-300 dark:text-neutral-600">·</span>
          <div class="flex items-center gap-1">
            <Zap class="w-3 h-3" />
            <span>{{ t('footer.noLimit') }}</span>
          </div>
          <span class="text-neutral-300 dark:text-neutral-600">·</span>
          <div class="flex items-center gap-1">
            <Globe class="w-3 h-3" />
            <span>{{ t('footer.p2p') }}</span>
          </div>
        </div>

        <RouterLink to="/legal" class="text-xs text-neutral-400 hover:text-neutral-600 dark:hover:text-neutral-300 transition-colors">
          {{ t('footer.legal') }}
        </RouterLink>
      </div>

      <!-- Desktop Layout -->
      <div class="hidden md:flex flex-col items-center gap-4">
        <div class="flex items-center justify-between w-full">
          <div class="flex items-center gap-3">
            <svg viewBox="0 0 32 32" class="w-5 h-5">
              <circle cx="16" cy="16" r="12" fill="none" stroke="currentColor" stroke-width="3" class="text-neutral-900 dark:text-white"/>
            </svg>
            <span class="text-sm font-medium text-neutral-900 dark:text-white">SecureBeam</span>
          </div>

          <div class="flex items-center gap-5 text-sm text-neutral-500 dark:text-neutral-400">
            <div class="flex items-center gap-1.5">
              <Lock class="w-3.5 h-3.5" />
              <span>{{ t('footer.encrypted') }}</span>
            </div>
            <span class="text-neutral-300 dark:text-neutral-600">|</span>
            <div class="flex items-center gap-1.5">
              <Zap class="w-3.5 h-3.5" />
              <span>{{ t('footer.noLimit') }}</span>
            </div>
            <span class="text-neutral-300 dark:text-neutral-600">|</span>
            <div class="flex items-center gap-1.5">
              <Globe class="w-3.5 h-3.5" />
              <span>{{ t('footer.p2p') }}</span>
            </div>
          </div>

          <span class="text-sm text-neutral-500 dark:text-neutral-400">securebeam.eu</span>
        </div>

        <RouterLink to="/legal" class="text-sm text-neutral-400 hover:text-neutral-600 dark:hover:text-neutral-300 transition-colors">
          {{ t('footer.legal') }}
        </RouterLink>
      </div>
    </div>
  </footer>
</template>
