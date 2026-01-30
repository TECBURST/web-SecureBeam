<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import { RouterLink } from 'vue-router'
import { Shield, ArrowLeft, Clock } from 'lucide-vue-next'

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
            <Shield class="w-6 h-6 text-neutral-700 dark:text-neutral-300" />
          </div>
          <div>
            <h1 class="text-2xl font-bold text-neutral-900 dark:text-white">
              {{ t('legal.privacy.title') }}
            </h1>
            <span class="text-xs text-neutral-400 dark:text-neutral-500">v1.0</span>
          </div>
        </div>

        <section>
          <h2 class="text-lg font-semibold text-neutral-900 dark:text-white mb-3">
            {{ t('legal.privacy.intro.title') }}
          </h2>
          <p class="text-neutral-700 dark:text-neutral-300">
            {{ t('legal.privacy.intro.text') }}
          </p>
        </section>

        <section>
          <h2 class="text-lg font-semibold text-neutral-900 dark:text-white mb-3">
            {{ t('legal.privacy.controller.title') }}
          </h2>
          <div class="text-neutral-700 dark:text-neutral-300 space-y-1">
            <p class="font-medium">Jan Kärger</p>
            <p>Dannenbergstraße 33</p>
            <p>30459 Hannover</p>
            <p>{{ t('legal.impressum.email') }}: kontakt@securebeam.eu</p>
          </div>
        </section>

        <section>
          <h2 class="text-lg font-semibold text-neutral-900 dark:text-white mb-3">
            {{ t('legal.privacy.data.title') }}
          </h2>

          <h3 class="text-base font-medium text-neutral-800 dark:text-neutral-200 mt-5 mb-2">
            {{ t('legal.privacy.data.server.title') }}
          </h3>
          <p class="text-neutral-700 dark:text-neutral-300 mb-2">
            {{ t('legal.privacy.data.server.text') }}
          </p>
          <ul class="list-disc list-inside text-neutral-700 dark:text-neutral-300 space-y-1 ml-2">
            <li>{{ t('legal.privacy.data.server.ip') }}</li>
            <li>{{ t('legal.privacy.data.server.time') }}</li>
            <li>{{ t('legal.privacy.data.server.browser') }}</li>
          </ul>
          <p class="text-neutral-700 dark:text-neutral-300 mt-2">
            {{ t('legal.privacy.data.server.basis') }}
          </p>

          <h3 class="text-base font-medium text-neutral-800 dark:text-neutral-200 mt-5 mb-2">
            {{ t('legal.privacy.data.webrtc.title') }}
          </h3>
          <p class="text-neutral-700 dark:text-neutral-300">
            {{ t('legal.privacy.data.webrtc.text') }}
          </p>

          <h3 class="text-base font-medium text-neutral-800 dark:text-neutral-200 mt-5 mb-2">
            {{ t('legal.privacy.data.stats.title') }}
          </h3>
          <p class="text-neutral-700 dark:text-neutral-300">
            {{ t('legal.privacy.data.stats.text') }}
          </p>
        </section>

        <section>
          <h2 class="text-lg font-semibold text-neutral-900 dark:text-white mb-3">
            {{ t('legal.privacy.cloudflare.title') }}
          </h2>
          <p class="text-neutral-700 dark:text-neutral-300 mb-2">
            {{ t('legal.privacy.cloudflare.text') }}
          </p>
          <p class="text-neutral-700 dark:text-neutral-300">
            {{ t('legal.privacy.cloudflare.more') }}
            <a
              href="https://www.cloudflare.com/privacypolicy/"
              target="_blank"
              rel="noopener noreferrer"
              class="text-neutral-900 dark:text-white underline hover:no-underline"
            >
              https://www.cloudflare.com/privacypolicy/
            </a>
          </p>
        </section>

        <section>
          <h2 class="text-lg font-semibold text-neutral-900 dark:text-white mb-3">
            {{ t('legal.privacy.cookies.title') }}
          </h2>
          <p class="text-neutral-700 dark:text-neutral-300">
            {{ t('legal.privacy.cookies.text') }}
          </p>
        </section>

        <section>
          <h2 class="text-lg font-semibold text-neutral-900 dark:text-white mb-3">
            {{ t('legal.privacy.rights.title') }}
          </h2>
          <p class="text-neutral-700 dark:text-neutral-300 mb-2">
            {{ t('legal.privacy.rights.intro') }}
          </p>
          <ul class="list-disc list-inside text-neutral-700 dark:text-neutral-300 space-y-1 ml-2">
            <li>{{ t('legal.privacy.rights.access') }}</li>
            <li>{{ t('legal.privacy.rights.rectification') }}</li>
            <li>{{ t('legal.privacy.rights.erasure') }}</li>
            <li>{{ t('legal.privacy.rights.restriction') }}</li>
            <li>{{ t('legal.privacy.rights.portability') }}</li>
            <li>{{ t('legal.privacy.rights.objection') }}</li>
          </ul>
          <p class="text-neutral-700 dark:text-neutral-300 mt-3">
            {{ t('legal.privacy.rights.complaint') }}
          </p>
        </section>

        <section>
          <h2 class="text-lg font-semibold text-neutral-900 dark:text-white mb-3">
            {{ t('legal.privacy.security.title') }}
          </h2>
          <p class="text-neutral-700 dark:text-neutral-300">
            {{ t('legal.privacy.security.text') }}
          </p>
        </section>

        <section>
          <h2 class="text-lg font-semibold text-neutral-900 dark:text-white mb-3">
            {{ t('legal.privacy.changes.title') }}
          </h2>
          <p class="text-neutral-700 dark:text-neutral-300">
            {{ t('legal.privacy.changes.text') }}
          </p>
        </section>

        <!-- Last Updated -->
        <div class="flex items-center gap-2 pt-4 border-t border-neutral-100 dark:border-neutral-800 text-sm text-neutral-400 dark:text-neutral-500">
          <Clock class="w-4 h-4" />
          <span>{{ t('legal.privacy.lastUpdate') }}: 29. Januar 2026</span>
        </div>
      </div>
    </div>
  </div>
</template>
