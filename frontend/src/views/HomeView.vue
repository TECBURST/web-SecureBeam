<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import { Rocket, Monitor, Shield } from 'lucide-vue-next'

const { t } = useI18n()
</script>

<template>
  <div class="flex-1 container mx-auto px-4 sm:px-6 py-12 sm:py-24 max-w-4xl">
    <!-- Coming Soon Content -->
    <div class="text-center">
      <!-- Icon -->
      <div class="w-20 h-20 sm:w-24 sm:h-24 mx-auto mb-8 rounded-full bg-neutral-100 dark:bg-neutral-800 flex items-center justify-center">
        <Rocket class="w-10 h-10 sm:w-12 sm:h-12 text-neutral-900 dark:text-white" />
      </div>

      <!-- Title -->
      <h1 class="text-3xl sm:text-5xl md:text-6xl font-semibold tracking-tight text-neutral-900 dark:text-white mb-4 sm:mb-6">
        {{ t('comingSoon.title') }}
      </h1>

      <!-- Subtitle -->
      <p class="text-base sm:text-xl text-neutral-700 dark:text-neutral-300 max-w-2xl mx-auto leading-relaxed mb-8 sm:mb-12">
        {{ t('comingSoon.subtitle') }}
      </p>

      <!-- Info Cards -->
      <div class="grid sm:grid-cols-2 gap-4 sm:gap-6 max-w-2xl mx-auto mb-12">
        <!-- Native Apps Card -->
        <div class="card !p-6 text-left">
          <div class="flex items-start gap-4">
            <div class="w-10 h-10 rounded-lg bg-neutral-100 dark:bg-neutral-700 flex items-center justify-center flex-shrink-0">
              <Monitor class="w-5 h-5 text-neutral-900 dark:text-white" />
            </div>
            <div>
              <h3 class="font-semibold text-neutral-900 dark:text-white mb-1">
                {{ t('comingSoon.nativeApps.title') }}
              </h3>
              <p class="text-sm text-neutral-600 dark:text-neutral-400">
                {{ t('comingSoon.nativeApps.description') }}
              </p>
            </div>
          </div>
        </div>

        <!-- Better Protocol Card -->
        <div class="card !p-6 text-left">
          <div class="flex items-start gap-4">
            <div class="w-10 h-10 rounded-lg bg-neutral-100 dark:bg-neutral-700 flex items-center justify-center flex-shrink-0">
              <Shield class="w-5 h-5 text-neutral-900 dark:text-white" />
            </div>
            <div>
              <h3 class="font-semibold text-neutral-900 dark:text-white mb-1">
                {{ t('comingSoon.betterProtocol.title') }}
              </h3>
              <p class="text-sm text-neutral-600 dark:text-neutral-400">
                {{ t('comingSoon.betterProtocol.description') }}
              </p>
            </div>
          </div>
        </div>
      </div>

      <!-- Stay Tuned Message -->
      <p class="text-sm text-neutral-500 dark:text-neutral-500">
        {{ t('comingSoon.stayTuned') }}
      </p>
    </div>
  </div>
</template>
