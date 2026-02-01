<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useI18n } from 'vue-i18n'
import { open } from '@tauri-apps/plugin-dialog'
import { invoke } from '@tauri-apps/api/core'
import { ArrowLeft, Folder, Wifi, CheckCircle, XCircle, Loader2, Globe } from 'lucide-vue-next'
import { RouterLink } from 'vue-router'
import { SUPPORTED_LOCALES, LOCALE_NAMES, setLocale, getCurrentLocale, type Locale } from '../i18n'

const { t } = useI18n()

// Download location
const downloadPath = ref('')
const isLoadingPath = ref(true)

// Connection test
const isTestingConnection = ref(false)
const signalingStatus = ref<'unknown' | 'online' | 'offline'>('unknown')
const relayStatus = ref<'unknown' | 'online' | 'offline'>('unknown')
const signalingLatency = ref<number | null>(null)
const relayLatency = ref<number | null>(null)

// Language
const currentLocale = ref<Locale>(getCurrentLocale())

// App version
const appVersion = ref('1.0.0')

async function loadSettings() {
  try {
    // Try to get saved download path from Tauri
    const savedPath = await invoke<string | null>('get_download_path')
    if (savedPath) {
      downloadPath.value = savedPath
    } else {
      // Default to user's Downloads folder
      downloadPath.value = await invoke<string>('get_default_download_path')
    }
  } catch (e) {
    console.error('Failed to load settings:', e)
    downloadPath.value = '~/Downloads'
  } finally {
    isLoadingPath.value = false
  }
}

async function selectDownloadFolder() {
  try {
    const selected = await open({
      directory: true,
      multiple: false,
      title: t('settings.selectFolder')
    })

    if (selected && typeof selected === 'string') {
      downloadPath.value = selected
      // Save to Tauri backend
      await invoke('set_download_path', { path: selected })
    }
  } catch (e) {
    console.error('Failed to select folder:', e)
  }
}

async function testConnection() {
  isTestingConnection.value = true
  signalingStatus.value = 'unknown'
  relayStatus.value = 'unknown'
  signalingLatency.value = null
  relayLatency.value = null

  try {
    // Test signaling server
    const signalingStart = performance.now()
    const signalingResult = await invoke<boolean>('test_signaling_connection')
    signalingLatency.value = Math.round(performance.now() - signalingStart)
    signalingStatus.value = signalingResult ? 'online' : 'offline'
  } catch {
    signalingStatus.value = 'offline'
  }

  try {
    // Test relay server
    const relayStart = performance.now()
    const relayResult = await invoke<boolean>('test_relay_connection')
    relayLatency.value = Math.round(performance.now() - relayStart)
    relayStatus.value = relayResult ? 'online' : 'offline'
  } catch {
    relayStatus.value = 'offline'
  }

  isTestingConnection.value = false
}

function changeLanguage(locale: Locale) {
  currentLocale.value = locale
  setLocale(locale)
}

onMounted(() => {
  loadSettings()
})
</script>

<template>
  <main class="flex-1 container mx-auto px-4 sm:px-6 py-8 max-w-2xl">
    <!-- Header -->
    <div class="flex items-center gap-4 mb-8">
      <RouterLink
        to="/"
        class="p-2 rounded-lg hover:bg-neutral-100 dark:hover:bg-neutral-800 transition-colors"
      >
        <ArrowLeft class="w-5 h-5 text-neutral-600 dark:text-neutral-400" />
      </RouterLink>
      <h1 class="text-2xl font-semibold text-neutral-900 dark:text-white">
        {{ t('settings.title') }}
      </h1>
    </div>

    <div class="space-y-6">
      <!-- Language Section -->
      <section class="card">
        <div class="flex items-center gap-3 mb-4">
          <Globe class="w-5 h-5 text-neutral-600 dark:text-neutral-400" />
          <h2 class="text-lg font-medium text-neutral-900 dark:text-white">
            {{ t('settings.language') }}
          </h2>
        </div>

        <div class="grid grid-cols-2 sm:grid-cols-3 gap-2">
          <button
            v-for="locale in SUPPORTED_LOCALES"
            :key="locale"
            @click="changeLanguage(locale)"
            class="px-4 py-2 rounded-lg text-sm font-medium transition-colors"
            :class="currentLocale === locale
              ? 'bg-neutral-900 dark:bg-white text-white dark:text-neutral-900'
              : 'bg-neutral-100 dark:bg-neutral-800 text-neutral-700 dark:text-neutral-300 hover:bg-neutral-200 dark:hover:bg-neutral-700'"
          >
            {{ LOCALE_NAMES[locale] }}
          </button>
        </div>
      </section>

      <!-- Download Location Section -->
      <section class="card">
        <div class="flex items-center gap-3 mb-4">
          <Folder class="w-5 h-5 text-neutral-600 dark:text-neutral-400" />
          <h2 class="text-lg font-medium text-neutral-900 dark:text-white">
            {{ t('settings.downloadLocation') }}
          </h2>
        </div>

        <div class="flex flex-col sm:flex-row gap-3">
          <div class="flex-1 px-4 py-3 bg-neutral-100 dark:bg-neutral-800 rounded-lg">
            <p class="text-xs text-neutral-500 dark:text-neutral-500 mb-1">
              {{ t('settings.currentFolder') }}
            </p>
            <p class="text-sm text-neutral-900 dark:text-white font-mono truncate">
              {{ isLoadingPath ? '...' : downloadPath }}
            </p>
          </div>
          <button
            @click="selectDownloadFolder"
            class="btn-primary whitespace-nowrap"
          >
            {{ t('settings.selectFolder') }}
          </button>
        </div>
      </section>

      <!-- Connection Test Section -->
      <section class="card">
        <div class="flex items-center justify-between mb-4">
          <div class="flex items-center gap-3">
            <Wifi class="w-5 h-5 text-neutral-600 dark:text-neutral-400" />
            <h2 class="text-lg font-medium text-neutral-900 dark:text-white">
              {{ t('settings.connectionTest') }}
            </h2>
          </div>
          <button
            @click="testConnection"
            :disabled="isTestingConnection"
            class="btn-secondary flex items-center gap-2"
          >
            <Loader2 v-if="isTestingConnection" class="w-4 h-4 animate-spin" />
            {{ isTestingConnection ? t('settings.testing') : t('settings.testConnection') }}
          </button>
        </div>

        <div class="space-y-3">
          <!-- Signaling Server -->
          <div class="flex items-center justify-between p-3 bg-neutral-100 dark:bg-neutral-800 rounded-lg">
            <div class="flex items-center gap-3">
              <div
                class="w-2 h-2 rounded-full"
                :class="{
                  'bg-neutral-400': signalingStatus === 'unknown',
                  'bg-green-500': signalingStatus === 'online',
                  'bg-red-500': signalingStatus === 'offline'
                }"
              />
              <span class="text-sm text-neutral-700 dark:text-neutral-300">
                {{ t('settings.signaling') }}
              </span>
            </div>
            <div class="flex items-center gap-2">
              <span v-if="signalingLatency !== null" class="text-xs text-neutral-500">
                {{ signalingLatency }}ms
              </span>
              <CheckCircle v-if="signalingStatus === 'online'" class="w-4 h-4 text-green-500" />
              <XCircle v-else-if="signalingStatus === 'offline'" class="w-4 h-4 text-red-500" />
            </div>
          </div>

          <!-- Relay Server -->
          <div class="flex items-center justify-between p-3 bg-neutral-100 dark:bg-neutral-800 rounded-lg">
            <div class="flex items-center gap-3">
              <div
                class="w-2 h-2 rounded-full"
                :class="{
                  'bg-neutral-400': relayStatus === 'unknown',
                  'bg-green-500': relayStatus === 'online',
                  'bg-red-500': relayStatus === 'offline'
                }"
              />
              <span class="text-sm text-neutral-700 dark:text-neutral-300">
                {{ t('settings.relay') }}
              </span>
            </div>
            <div class="flex items-center gap-2">
              <span v-if="relayLatency !== null" class="text-xs text-neutral-500">
                {{ relayLatency }}ms
              </span>
              <CheckCircle v-if="relayStatus === 'online'" class="w-4 h-4 text-green-500" />
              <XCircle v-else-if="relayStatus === 'offline'" class="w-4 h-4 text-red-500" />
            </div>
          </div>
        </div>
      </section>

      <!-- About Section -->
      <section class="card">
        <h2 class="text-lg font-medium text-neutral-900 dark:text-white mb-4">
          {{ t('settings.about') }}
        </h2>
        <div class="text-sm text-neutral-600 dark:text-neutral-400">
          <p>SecureBeam - {{ t('settings.version') }} {{ appVersion }}</p>
          <p class="mt-2">
            End-to-end encrypted file transfer using the Magic Wormhole protocol.
          </p>
        </div>
      </section>
    </div>
  </main>
</template>
