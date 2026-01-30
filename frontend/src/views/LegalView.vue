<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import { RouterLink } from 'vue-router'
import { Building2, Shield, ArrowRight, Clock } from 'lucide-vue-next'

const { t } = useI18n()
</script>

<template>
  <div class="min-h-screen bg-neutral-50 dark:bg-neutral-950 py-12 sm:py-16">
    <div class="container mx-auto px-4 sm:px-6 max-w-4xl">
      <!-- Header -->
      <div class="text-center mb-12">
        <h1 class="text-3xl sm:text-4xl font-bold text-neutral-900 dark:text-white mb-3">
          {{ t('legal.overview.title') }}
        </h1>
        <p class="text-neutral-500 dark:text-neutral-400">
          {{ t('legal.overview.subtitle') }}
        </p>
      </div>

      <!-- Cards Grid -->
      <div class="grid md:grid-cols-2 gap-6 mb-12">
        <!-- Impressum Card -->
        <RouterLink
          to="/legal/impressum"
          class="bg-white dark:bg-neutral-900 rounded-2xl border border-neutral-200 dark:border-neutral-800 p-6 hover:border-neutral-300 dark:hover:border-neutral-700 hover:shadow-sm transition-all group"
        >
          <div class="flex items-start justify-between mb-4">
            <div class="w-12 h-12 bg-neutral-100 dark:bg-neutral-800 rounded-xl flex items-center justify-center">
              <Building2 class="w-6 h-6 text-neutral-700 dark:text-neutral-300" />
            </div>
            <span class="text-xs text-neutral-400 dark:text-neutral-500 bg-neutral-100 dark:bg-neutral-800 px-2 py-1 rounded-full">v1.0</span>
          </div>

          <h2 class="text-xl font-semibold text-neutral-900 dark:text-white mb-2">
            {{ t('legal.impressum.title') }}
          </h2>
          <p class="text-neutral-600 dark:text-neutral-400 text-sm mb-4">
            {{ t('legal.overview.impressumDesc') }}
          </p>

          <div class="flex items-center gap-2 text-sm font-medium text-neutral-900 dark:text-white group-hover:gap-3 transition-all">
            <span>{{ t('legal.overview.readMore') }}</span>
            <ArrowRight class="w-4 h-4" />
          </div>
        </RouterLink>

        <!-- Privacy Card -->
        <RouterLink
          to="/legal/privacy"
          class="bg-white dark:bg-neutral-900 rounded-2xl border border-neutral-200 dark:border-neutral-800 p-6 hover:border-neutral-300 dark:hover:border-neutral-700 hover:shadow-sm transition-all group"
        >
          <div class="flex items-start justify-between mb-4">
            <div class="w-12 h-12 bg-neutral-100 dark:bg-neutral-800 rounded-xl flex items-center justify-center">
              <Shield class="w-6 h-6 text-neutral-700 dark:text-neutral-300" />
            </div>
            <span class="text-xs text-neutral-400 dark:text-neutral-500 bg-neutral-100 dark:bg-neutral-800 px-2 py-1 rounded-full">v1.0</span>
          </div>

          <h2 class="text-xl font-semibold text-neutral-900 dark:text-white mb-2">
            {{ t('legal.privacy.title') }}
          </h2>
          <p class="text-neutral-600 dark:text-neutral-400 text-sm mb-4">
            {{ t('legal.overview.privacyDesc') }}
          </p>

          <div class="flex items-center gap-2 text-sm font-medium text-neutral-900 dark:text-white group-hover:gap-3 transition-all">
            <span>{{ t('legal.overview.readMore') }}</span>
            <ArrowRight class="w-4 h-4" />
          </div>
        </RouterLink>
      </div>

      <!-- Last Updated -->
      <div class="flex items-center justify-center gap-2 text-sm text-neutral-400 dark:text-neutral-500">
        <Clock class="w-4 h-4" />
        <span>{{ t('legal.privacy.lastUpdate') }}: 29. Januar 2026</span>
      </div>
    </div>
  </div>
</template>
