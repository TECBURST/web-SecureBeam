<script setup lang="ts">
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'
import { Monitor, Shield, Apple, Terminal } from 'lucide-vue-next'

const { t } = useI18n()

// Detect user's OS
const userOS = ref<'windows' | 'macos' | 'linux'>('windows')
if (typeof navigator !== 'undefined') {
  const platform = navigator.platform.toLowerCase()
  if (platform.includes('mac')) {
    userOS.value = 'macos'
  } else if (platform.includes('linux')) {
    userOS.value = 'linux'
  }
}

// Download links
const downloadLinks = {
  windows: 'https://github.com/TECBURST/web-SecureBeam/releases/latest/download/SecureBeam_1.0.0_x64-setup.exe',
  macos: 'https://github.com/TECBURST/web-SecureBeam/releases/latest/download/SecureBeam_1.0.0_aarch64.dmg',
  linux: {
    appimage: 'https://github.com/TECBURST/web-SecureBeam/releases/latest/download/SecureBeam_1.0.0_amd64.AppImage',
    deb: 'https://github.com/TECBURST/web-SecureBeam/releases/latest/download/SecureBeam_1.0.0_amd64.deb',
    rpm: 'https://github.com/TECBURST/web-SecureBeam/releases/latest/download/SecureBeam-1.0.0-1.x86_64.rpm'
  }
}

const showLinuxOptions = ref(false)
</script>

<template>
  <div class="flex-1 container mx-auto px-4 sm:px-6 py-12 sm:py-20 max-w-4xl">
    <!-- Hero Section with CTA -->
    <div class="text-center mb-12 sm:mb-16">
      <!-- Badge -->
      <div class="inline-flex items-center gap-2 px-4 py-2 rounded-full bg-green-100 dark:bg-green-900/30 text-green-700 dark:text-green-400 text-sm font-medium mb-6">
        <span class="w-2 h-2 rounded-full bg-green-500 animate-pulse"></span>
        {{ t('download.available') }}
      </div>

      <!-- Main Headline -->
      <h1 class="text-3xl sm:text-5xl md:text-6xl font-bold tracking-tight text-neutral-900 dark:text-white mb-4 sm:mb-6">
        {{ t('download.headline') }}
      </h1>

      <!-- CTA Subline -->
      <p class="text-lg sm:text-xl text-neutral-600 dark:text-neutral-400 max-w-2xl mx-auto mb-8 sm:mb-10">
        {{ t('download.subline') }}
      </p>

      <!-- Primary Download Button -->
      <div class="flex flex-col items-center gap-4 mb-8">
        <!-- Windows -->
        <a
          v-if="userOS === 'windows'"
          :href="downloadLinks.windows"
          class="btn btn-primary !px-8 !py-4 text-lg flex items-center gap-3 shadow-lg hover:shadow-xl transition-all"
        >
          <Monitor class="w-6 h-6" />
          {{ t('download.forWindows') }}
        </a>

        <!-- macOS -->
        <a
          v-else-if="userOS === 'macos'"
          :href="downloadLinks.macos"
          class="btn btn-primary !px-8 !py-4 text-lg flex items-center gap-3 shadow-lg hover:shadow-xl transition-all"
        >
          <Apple class="w-6 h-6" />
          {{ t('download.forMac') }}
        </a>

        <!-- Linux -->
        <div v-else class="flex flex-col items-center gap-3">
          <button
            @click="showLinuxOptions = !showLinuxOptions"
            class="btn btn-primary !px-8 !py-4 text-lg flex items-center gap-3 shadow-lg hover:shadow-xl transition-all"
          >
            <Terminal class="w-6 h-6" />
            {{ t('download.forLinux') }}
          </button>
          <div v-if="showLinuxOptions" class="flex flex-wrap justify-center gap-3 animate-in fade-in">
            <a :href="downloadLinks.linux.appimage" class="btn btn-secondary">AppImage</a>
            <a :href="downloadLinks.linux.deb" class="btn btn-secondary">.deb</a>
            <a :href="downloadLinks.linux.rpm" class="btn btn-secondary">.rpm</a>
          </div>
        </div>

        <!-- Version info -->
        <p class="text-sm text-neutral-500 dark:text-neutral-500">
          v1.0.0 · {{ t('download.freeForever') }}
        </p>
      </div>

      <!-- Other Platforms -->
      <div class="flex flex-wrap justify-center gap-6 text-sm">
        <span class="text-neutral-500 dark:text-neutral-500">{{ t('download.otherPlatforms') }}:</span>
        <a
          v-if="userOS !== 'windows'"
          :href="downloadLinks.windows"
          class="text-neutral-700 dark:text-neutral-300 hover:text-neutral-900 dark:hover:text-white underline underline-offset-4"
        >
          Windows
        </a>
        <a
          v-if="userOS !== 'macos'"
          :href="downloadLinks.macos"
          class="text-neutral-700 dark:text-neutral-300 hover:text-neutral-900 dark:hover:text-white underline underline-offset-4"
        >
          macOS
        </a>
        <a
          v-if="userOS !== 'linux'"
          :href="downloadLinks.linux.appimage"
          class="text-neutral-700 dark:text-neutral-300 hover:text-neutral-900 dark:hover:text-white underline underline-offset-4"
        >
          Linux
        </a>
      </div>
    </div>

    <!-- Feature Cards -->
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

    <!-- How it works teaser -->
    <div class="text-center">
      <p class="text-sm text-neutral-500 dark:text-neutral-500">
        {{ t('download.encrypted') }} · {{ t('download.noCloud') }} · {{ t('download.openSource') }}
      </p>
    </div>
  </div>
</template>
