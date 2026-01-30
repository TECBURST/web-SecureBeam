<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import { RouterLink } from 'vue-router'
import { Building2, ArrowLeft } from 'lucide-vue-next'

const { t } = useI18n()
</script>

<template>
  <div class="min-h-screen bg-neutral-50 dark:bg-neutral-950 py-12 sm:py-16">
    <div class="container mx-auto px-4 sm:px-6 max-w-3xl">
      <!-- Back Link -->
      <RouterLink
        to="/legal"
        class="inline-flex items-center gap-2 text-sm text-neutral-500 dark:text-neutral-400 hover:text-neutral-700 dark:hover:text-neutral-300 transition-colors mb-8"
      >
        <ArrowLeft class="w-4 h-4" />
        <span>{{ t('legal.overview.title') }}</span>
      </RouterLink>

      <!-- Content -->
      <div class="bg-white dark:bg-neutral-900 rounded-2xl border border-neutral-200 dark:border-neutral-800 p-6 sm:p-8 space-y-8">
        <div class="flex items-center gap-4 pb-4 border-b border-neutral-100 dark:border-neutral-800">
          <div class="w-12 h-12 bg-neutral-100 dark:bg-neutral-800 rounded-xl flex items-center justify-center">
            <Building2 class="w-6 h-6 text-neutral-700 dark:text-neutral-300" />
          </div>
          <div>
            <h1 class="text-2xl font-bold text-neutral-900 dark:text-white">
              {{ t('legal.impressum.title') }}
            </h1>
            <span class="text-xs text-neutral-400 dark:text-neutral-500">v1.0</span>
          </div>
        </div>

        <section>
          <h2 class="text-lg font-semibold text-neutral-900 dark:text-white mb-3">
            {{ t('legal.impressum.according') }}
          </h2>
          <div class="text-neutral-700 dark:text-neutral-300 space-y-1">
            <p class="font-medium">Jan Kärger</p>
            <p>Dannenbergstraße 33</p>
            <p>30459 Hannover</p>
            <p>{{ t('legal.impressum.country') }}</p>
          </div>
        </section>

        <section>
          <h2 class="text-lg font-semibold text-neutral-900 dark:text-white mb-3">
            {{ t('legal.impressum.contact') }}
          </h2>
          <div class="text-neutral-700 dark:text-neutral-300">
            <p>{{ t('legal.impressum.email') }}: kontakt@securebeam.eu</p>
          </div>
        </section>

        <section>
          <h2 class="text-lg font-semibold text-neutral-900 dark:text-white mb-3">
            {{ t('legal.impressum.responsible') }}
          </h2>
          <div class="text-neutral-700 dark:text-neutral-300 space-y-1">
            <p>Jan Kärger</p>
            <p>Dannenbergstraße 33</p>
            <p>30459 Hannover</p>
          </div>
        </section>

        <section>
          <h2 class="text-lg font-semibold text-neutral-900 dark:text-white mb-3">
            {{ t('legal.impressum.dispute') }}
          </h2>
          <p class="text-neutral-700 dark:text-neutral-300">
            {{ t('legal.impressum.disputeText') }}
            <a
              href="https://ec.europa.eu/consumers/odr/"
              target="_blank"
              rel="noopener noreferrer"
              class="text-neutral-900 dark:text-white underline hover:no-underline"
            >
              https://ec.europa.eu/consumers/odr/
            </a>
          </p>
          <p class="text-neutral-700 dark:text-neutral-300 mt-3">
            {{ t('legal.impressum.disputeNote') }}
          </p>
        </section>

        <section>
          <h2 class="text-lg font-semibold text-neutral-900 dark:text-white mb-3">
            {{ t('legal.impressum.disclaimer') }}
          </h2>
          <p class="text-neutral-700 dark:text-neutral-300">
            {{ t('legal.impressum.disclaimerText') }}
          </p>
        </section>
      </div>
    </div>
  </div>
</template>
