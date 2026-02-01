<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useI18n } from 'vue-i18n'
import { Shield, Monitor, Download, Loader2 } from 'lucide-vue-next'

const { t } = useI18n()

// Dynamic download links from GitHub API
const isLoading = ref(true)
const version = ref('')

interface DownloadAsset {
  name: string
  url: string
  size: string
}

const downloads = ref({
  windows: [] as DownloadAsset[],
  macos: [] as DownloadAsset[],
  linux: [] as DownloadAsset[]
})

// Format file size
function formatSize(bytes: number): string {
  if (bytes >= 1024 * 1024 * 1024) {
    return (bytes / (1024 * 1024 * 1024)).toFixed(1) + ' GB'
  } else if (bytes >= 1024 * 1024) {
    return (bytes / (1024 * 1024)).toFixed(1) + ' MB'
  } else if (bytes >= 1024) {
    return (bytes / 1024).toFixed(1) + ' KB'
  }
  return bytes + ' B'
}

// Get friendly name for file
function getFriendlyName(filename: string): string {
  if (filename.endsWith('.exe') || filename.endsWith('.msi')) {
    if (filename.includes('setup') || filename.includes('Setup')) {
      return 'Installer (.exe)'
    }
    return filename.endsWith('.msi') ? 'Installer (.msi)' : 'Portable (.exe)'
  } else if (filename.endsWith('.dmg')) {
    return 'Disk Image (.dmg)'
  } else if (filename.endsWith('.app.tar.gz')) {
    return 'App Bundle (.app.tar.gz)'
  } else if (filename.endsWith('.AppImage')) {
    return 'AppImage (Universal)'
  } else if (filename.endsWith('.deb')) {
    return 'Debian/Ubuntu (.deb)'
  } else if (filename.endsWith('.rpm')) {
    return 'Fedora/RHEL (.rpm)'
  }
  return filename
}

// Fetch latest release from GitHub
async function fetchLatestRelease() {
  try {
    const response = await fetch('https://api.github.com/repos/TECBURST/web-SecureBeam/releases/latest')
    if (!response.ok) throw new Error('Failed to fetch release')

    const release = await response.json()
    version.value = release.tag_name.replace('v', '')

    // Categorize assets by OS
    for (const asset of release.assets) {
      const name = asset.name
      const lowerName = name.toLowerCase()
      const url = asset.browser_download_url
      const size = formatSize(asset.size)

      const downloadAsset: DownloadAsset = {
        name: getFriendlyName(name),
        url,
        size
      }

      if (lowerName.endsWith('.exe') || lowerName.endsWith('.msi')) {
        downloads.value.windows.push(downloadAsset)
      } else if (lowerName.endsWith('.dmg') || lowerName.includes('.app.tar.gz')) {
        downloads.value.macos.push(downloadAsset)
      } else if (lowerName.endsWith('.appimage') || lowerName.endsWith('.deb') || lowerName.endsWith('.rpm')) {
        downloads.value.linux.push(downloadAsset)
      }
    }
  } catch (error) {
    console.error('Failed to fetch latest release:', error)
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
  <div class="flex-1 container mx-auto px-4 sm:px-6 py-12 sm:py-20 max-w-5xl">
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
      <p class="text-lg sm:text-xl text-neutral-600 dark:text-neutral-400 max-w-2xl mx-auto mb-10 sm:mb-12">
        {{ t('download.subline') }}
      </p>

      <!-- Loading State -->
      <div v-if="isLoading" class="flex justify-center items-center py-20">
        <Loader2 class="w-8 h-8 animate-spin text-neutral-400" />
      </div>

      <!-- Download Cards -->
      <div v-else class="grid grid-cols-1 md:grid-cols-3 gap-6 mb-8">
        <!-- Windows Card -->
        <div class="bg-neutral-100 dark:bg-neutral-800 rounded-2xl p-6 border border-neutral-200 dark:border-neutral-700">
          <!-- Windows Icon -->
          <div class="mb-4 flex justify-center">
            <svg class="w-14 h-14 text-neutral-900 dark:text-white" viewBox="0 0 24 24" fill="currentColor">
              <path d="M0 3.449L9.75 2.1v9.451H0m10.949-9.602L24 0v11.4H10.949M0 12.6h9.75v9.451L0 20.699M10.949 12.6H24V24l-12.9-1.801"/>
            </svg>
          </div>

          <h3 class="text-xl font-bold text-neutral-900 dark:text-white mb-4">Windows</h3>

          <!-- Download Links -->
          <div class="space-y-2">
            <a
              v-for="asset in downloads.windows"
              :key="asset.url"
              :href="asset.url"
              target="_blank"
              rel="noopener noreferrer"
              class="flex items-center justify-between gap-2 bg-neutral-900 dark:bg-white text-white dark:text-neutral-900 hover:bg-neutral-700 dark:hover:bg-neutral-200 rounded-lg px-4 py-3 text-sm font-medium transition-colors cursor-pointer"
            >
              <span class="flex items-center gap-2">
                <Download class="w-4 h-4" />
                {{ asset.name }}
              </span>
              <span class="text-neutral-400 dark:text-neutral-500 text-xs">{{ asset.size }}</span>
            </a>
            <p v-if="downloads.windows.length === 0" class="text-neutral-500 text-sm py-2">
              Keine Downloads verfügbar
            </p>
          </div>
        </div>

        <!-- macOS Card -->
        <div class="bg-neutral-100 dark:bg-neutral-800 rounded-2xl p-6 border border-neutral-200 dark:border-neutral-700">
          <!-- Apple Icon -->
          <div class="mb-4 flex justify-center">
            <svg class="w-14 h-14 text-neutral-900 dark:text-white" viewBox="0 0 24 24" fill="currentColor">
              <path d="M18.71 19.5c-.83 1.24-1.71 2.45-3.05 2.47-1.34.03-1.77-.79-3.29-.79-1.53 0-2 .77-3.27.82-1.31.05-2.3-1.32-3.14-2.53C4.25 17 2.94 12.45 4.7 9.39c.87-1.52 2.43-2.48 4.12-2.51 1.28-.02 2.5.87 3.29.87.78 0 2.26-1.07 3.81-.91.65.03 2.47.26 3.64 1.98-.09.06-2.17 1.28-2.15 3.81.03 3.02 2.65 4.03 2.68 4.04-.03.07-.42 1.44-1.38 2.83M13 3.5c.73-.83 1.94-1.46 2.94-1.5.13 1.17-.34 2.35-1.04 3.19-.69.85-1.83 1.51-2.95 1.42-.15-1.15.41-2.35 1.05-3.11z"/>
            </svg>
          </div>

          <h3 class="text-xl font-bold text-neutral-900 dark:text-white mb-4">macOS</h3>

          <!-- Download Links -->
          <div class="space-y-2">
            <a
              v-for="asset in downloads.macos"
              :key="asset.url"
              :href="asset.url"
              target="_blank"
              rel="noopener noreferrer"
              class="flex items-center justify-between gap-2 bg-neutral-900 dark:bg-white text-white dark:text-neutral-900 hover:bg-neutral-700 dark:hover:bg-neutral-200 rounded-lg px-4 py-3 text-sm font-medium transition-colors cursor-pointer"
            >
              <span class="flex items-center gap-2">
                <Download class="w-4 h-4" />
                {{ asset.name }}
              </span>
              <span class="text-neutral-400 dark:text-neutral-500 text-xs">{{ asset.size }}</span>
            </a>
            <p v-if="downloads.macos.length === 0" class="text-neutral-500 text-sm py-2">
              Keine Downloads verfügbar
            </p>
          </div>
        </div>

        <!-- Linux Card -->
        <div class="bg-neutral-100 dark:bg-neutral-800 rounded-2xl p-6 border border-neutral-200 dark:border-neutral-700">
          <!-- Tux Icon -->
          <div class="mb-4 flex justify-center">
            <svg class="w-14 h-14 text-neutral-900 dark:text-white" viewBox="0 0 24 24" fill="currentColor">
              <path d="M12.504 0c-.155 0-.315.008-.48.021-4.226.333-3.105 4.807-3.17 6.298-.076 1.092-.3 1.953-1.05 3.02-.885 1.051-2.127 2.75-2.716 4.521-.278.832-.41 1.684-.287 2.489a.424.424 0 00-.11.135c-.26.268-.45.6-.663.839-.199.199-.485.267-.797.4-.313.136-.658.269-.864.68-.09.189-.136.394-.132.602 0 .199.027.4.055.536.058.399.116.728.04.97-.249.68-.28 1.145-.106 1.484.174.334.535.47.94.601.81.2 1.91.135 2.774.6.926.466 1.866.67 2.616.47.526-.116.97-.464 1.208-.946.587-.003 1.23-.269 2.26-.334.699-.058 1.574.267 2.577.2.025.134.063.198.114.333l.003.003c.391.778 1.113 1.132 1.884 1.071.771-.06 1.592-.536 2.257-1.306.631-.765 1.683-1.084 2.378-1.503.348-.199.629-.469.649-.853.023-.4-.2-.811-.714-1.376v-.097l-.003-.003c-.17-.2-.25-.535-.338-.926-.085-.401-.182-.786-.492-1.046h-.003c-.059-.054-.123-.067-.188-.135a.357.357 0 00-.19-.064c.431-1.278.264-2.55-.173-3.694-.533-1.41-1.465-2.638-2.175-3.483-.796-1.005-1.576-1.957-1.56-3.368.026-2.152.236-6.133-3.544-6.139zm.529 3.405h.013c.213 0 .396.062.584.198.19.135.33.332.438.533.105.259.158.459.166.724 0-.02.006-.04.006-.06v.105a.086.086 0 01-.004-.021l-.004-.024a1.807 1.807 0 01-.15.706.953.953 0 01-.213.335.71.71 0 00-.088-.042c-.104-.045-.198-.064-.284-.133a1.312 1.312 0 00-.22-.066c.05-.06.146-.133.183-.198.053-.128.082-.264.088-.402v-.02a1.21 1.21 0 00-.061-.4c-.045-.134-.101-.2-.183-.333-.084-.066-.167-.132-.267-.132h-.016c-.093 0-.176.03-.262.132a.8.8 0 00-.205.334 1.18 1.18 0 00-.09.4v.019c.002.089.008.179.02.267-.193-.067-.438-.135-.607-.202a1.635 1.635 0 01-.018-.2v-.02a1.772 1.772 0 01.15-.768c.082-.22.232-.406.43-.533a.985.985 0 01.594-.2zm-2.962.059h.036c.142 0 .27.048.399.135.146.129.264.288.344.465.09.199.14.4.153.667v.004c.007.134.006.2-.002.266v.08c-.03.007-.056.018-.083.024-.152.055-.274.135-.393.2.012-.09.013-.18.003-.267v-.015c-.012-.133-.04-.2-.082-.333a.613.613 0 00-.166-.267.248.248 0 00-.183-.064h-.021c-.071.006-.13.04-.186.132a.552.552 0 00-.12.27.944.944 0 00-.023.33v.015c.012.135.037.2.08.334.046.134.098.2.166.268.01.009.02.018.034.024-.07.057-.117.07-.176.136a.304.304 0 01-.131.068 2.62 2.62 0 01-.275-.402 1.772 1.772 0 01-.155-.667 1.759 1.759 0 01.08-.668 1.43 1.43 0 01.283-.535c.128-.133.26-.2.418-.2zm1.37 1.706c.332 0 .733.065 1.216.399.293.2.523.269 1.052.468h.003c.255.136.405.266.478.399v-.131a.571.571 0 01.016.47c-.123.31-.516.643-1.063.842v.002c-.268.135-.501.333-.775.465-.276.135-.588.292-1.012.267a1.139 1.139 0 01-.448-.067 3.566 3.566 0 01-.322-.198c-.195-.135-.363-.332-.612-.465v-.005h-.005c-.4-.246-.616-.512-.686-.71-.07-.268-.005-.47.193-.6.224-.135.38-.271.483-.336.104-.074.143-.102.176-.131h.002v-.003c.169-.202.436-.47.839-.601.139-.036.294-.065.466-.065zm2.8 2.142c.358 1.417 1.196 3.475 1.735 4.473.286.534.855 1.659 1.102 3.024.156-.005.33.018.513.064.646-1.671-.546-3.467-1.089-3.966-.22-.2-.232-.335-.123-.335.59.534 1.365 1.572 1.646 2.757.13.535.16 1.104.021 1.67.067.028.135.06.205.067 1.032.534 1.413.938 1.23 1.537v-.002c-.06-.003-.12 0-.18 0h-.016c.151-.467-.182-.825-1.065-1.224-.915-.4-1.646-.336-1.77.465-.008.043-.013.066-.018.135-.068.023-.139.053-.209.064-.43.268-.662.669-.793 1.187-.13.533-.17 1.156-.205 1.869v.003c-.02.334-.17.838-.319 1.35-1.5 1.072-3.58 1.538-5.348.334a2.645 2.645 0 00-.402-.533 1.45 1.45 0 00-.275-.333c.182 0 .338-.03.465-.067a.615.615 0 00.314-.334c.108-.267 0-.697-.345-1.163-.345-.467-.931-.995-1.788-1.521-.63-.4-.986-.87-1.15-1.396-.165-.534-.143-1.085-.015-1.645.245-1.07.873-2.11 1.274-2.763.107-.065.037.135-.408.974-.396.751-1.14 2.497-.122 3.854a8.123 8.123 0 01.647-2.876c.564-1.278 1.743-3.504 1.836-5.268.048.036.217.135.289.202.218.133.38.333.59.465.21.201.477.335.876.335.039.003.075.006.11.006.412 0 .73-.134.997-.268.29-.134.52-.334.74-.4h.005c.467-.135.835-.402 1.044-.7zm2.185 8.958c.037.6.343 1.245.882 1.377.588.134 1.434-.333 1.791-.765l.211-.01c.315-.007.577.01.847.268l.003.003c.208.199.305.53.391.876.085.4.154.78.409 1.066.486.527.645.906.636 1.14l.003-.007v.018l-.003-.012c-.015.262-.185.396-.498.574-.63.328-1.559.503-2.276 1.153-.718.65-1.057 1.238-1.657 1.377-.6.14-1.432-.197-1.773-.834l-.003.005-.002-.013c-.182-.26-.323-.532-.504-.663-.181-.135-.384-.2-.724-.2-.34 0-.643.065-.824.2-.182.135-.322.334-.505.599-.037.065-.074.132-.111.2a3.592 3.592 0 01-.315-.266 1.12 1.12 0 01-.303-.465c-.06-.2-.04-.47.175-.87.386-.719.97-.931 1.862-1.405.9-.466 1.648-.801 1.906-1.405.158-.401.093-.67-.078-.98-.178-.34-.515-.665-.974-1.096v-.001c.375-.33.696-.468 1.031-.601.006 0 .013 0 .019.003z"/>
            </svg>
          </div>

          <h3 class="text-xl font-bold text-neutral-900 dark:text-white mb-4">Linux</h3>

          <!-- Download Links -->
          <div class="space-y-2">
            <a
              v-for="asset in downloads.linux"
              :key="asset.url"
              :href="asset.url"
              target="_blank"
              rel="noopener noreferrer"
              class="flex items-center justify-between gap-2 bg-neutral-900 dark:bg-white text-white dark:text-neutral-900 hover:bg-neutral-700 dark:hover:bg-neutral-200 rounded-lg px-4 py-3 text-sm font-medium transition-colors cursor-pointer"
            >
              <span class="flex items-center gap-2">
                <Download class="w-4 h-4" />
                {{ asset.name }}
              </span>
              <span class="text-neutral-400 dark:text-neutral-500 text-xs">{{ asset.size }}</span>
            </a>
            <p v-if="downloads.linux.length === 0" class="text-neutral-500 text-sm py-2">
              Keine Downloads verfügbar
            </p>
          </div>
        </div>
      </div>

      <!-- Version info -->
      <p v-if="!isLoading" class="text-sm text-neutral-500 dark:text-neutral-500 mb-12">
        Version {{ version }}
      </p>
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
