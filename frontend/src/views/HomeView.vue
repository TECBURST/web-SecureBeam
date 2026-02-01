<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useI18n } from 'vue-i18n'
import { Monitor, Shield, Apple, Terminal, Loader2 } from 'lucide-vue-next'

const { t } = useI18n()

// Dynamic download links from GitHub API
const isLoading = ref(true)
const version = ref('')
const downloadLinks = ref({
  windows: '',
  macos: '',
  linux: {
    appimage: '',
    deb: '',
    rpm: ''
  }
})

const showLinuxOptions = ref(false)

// Fetch latest release from GitHub
async function fetchLatestRelease() {
  try {
    const response = await fetch('https://api.github.com/repos/TECBURST/web-SecureBeam/releases/latest')
    if (!response.ok) throw new Error('Failed to fetch release')

    const release = await response.json()
    version.value = release.tag_name.replace('v', '')

    // Find assets by file extension/pattern
    for (const asset of release.assets) {
      const name = asset.name.toLowerCase()
      const url = asset.browser_download_url

      if (name.endsWith('.exe') || name.endsWith('.msi')) {
        if (name.includes('setup')) {
          downloadLinks.value.windows = url
        }
      } else if (name.endsWith('.dmg')) {
        downloadLinks.value.macos = url
      } else if (name.endsWith('.appimage')) {
        downloadLinks.value.linux.appimage = url
      } else if (name.endsWith('.deb')) {
        downloadLinks.value.linux.deb = url
      } else if (name.endsWith('.rpm')) {
        downloadLinks.value.linux.rpm = url
      }
    }
  } catch (error) {
    console.error('Failed to fetch latest release:', error)
    // Fallback to GitHub releases page
    const fallback = 'https://github.com/TECBURST/web-SecureBeam/releases/latest'
    downloadLinks.value.windows = fallback
    downloadLinks.value.macos = fallback
    downloadLinks.value.linux.appimage = fallback
    downloadLinks.value.linux.deb = fallback
    downloadLinks.value.linux.rpm = fallback
    version.value = 'latest'
  } finally {
    isLoading.value = false
  }
}

onMounted(() => {
  fetchLatestRelease()
})
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

      <!-- Loading State -->
      <div v-if="isLoading" class="flex justify-center gap-4 mb-8">
        <div class="btn btn-primary !px-6 !py-3 flex items-center gap-2 opacity-70">
          <Loader2 class="w-5 h-5 animate-spin" />
        </div>
      </div>

      <!-- Download Buttons - All 3 side by side -->
      <div v-else class="flex flex-col items-center gap-6 mb-8">
        <div class="grid grid-cols-1 sm:grid-cols-3 gap-4 w-full max-w-2xl">
          <!-- Windows -->
          <a
            :href="downloadLinks.windows"
            class="btn btn-primary !py-4 flex flex-col items-center gap-2 shadow-lg hover:shadow-xl transition-all"
          >
            <Monitor class="w-8 h-8" />
            <span class="font-medium">Windows</span>
          </a>

          <!-- macOS -->
          <a
            :href="downloadLinks.macos"
            class="btn btn-primary !py-4 flex flex-col items-center gap-2 shadow-lg hover:shadow-xl transition-all"
          >
            <Apple class="w-8 h-8" />
            <span class="font-medium">macOS</span>
          </a>

          <!-- Linux -->
          <div class="flex flex-col">
            <button
              @click="showLinuxOptions = !showLinuxOptions"
              class="btn btn-primary !py-4 flex flex-col items-center gap-2 shadow-lg hover:shadow-xl transition-all w-full"
            >
              <Terminal class="w-8 h-8" />
              <span class="font-medium">Linux</span>
            </button>
            <!-- Linux Format Options -->
            <div v-if="showLinuxOptions" class="flex flex-col gap-2 mt-2">
              <a :href="downloadLinks.linux.appimage" class="btn btn-secondary text-sm">AppImage</a>
              <a :href="downloadLinks.linux.deb" class="btn btn-secondary text-sm">.deb (Debian/Ubuntu)</a>
              <a :href="downloadLinks.linux.rpm" class="btn btn-secondary text-sm">.rpm (Fedora/RHEL)</a>
            </div>
          </div>
        </div>

        <!-- Version info -->
        <p class="text-sm text-neutral-500 dark:text-neutral-500">
          v{{ version }}
        </p>
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
