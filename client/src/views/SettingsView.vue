<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useI18n } from 'vue-i18n'
import { open } from '@tauri-apps/plugin-dialog'
import { invoke } from '@tauri-apps/api/core'
import { ArrowLeft, Folder, Wifi, CheckCircle, XCircle, Loader2, FolderOpen, RefreshCw } from 'lucide-vue-next'
import { RouterLink } from 'vue-router'

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
      <!-- Download Location Section -->
      <section class="card">
        <div class="flex items-center gap-3 mb-5">
          <div class="w-10 h-10 rounded-xl bg-neutral-100 dark:bg-neutral-800 flex items-center justify-center">
            <Folder class="w-5 h-5 text-neutral-600 dark:text-neutral-400" />
          </div>
          <div>
            <h2 class="text-lg font-medium text-neutral-900 dark:text-white">
              {{ t('settings.downloadLocation') }}
            </h2>
            <p class="text-xs text-neutral-500">{{ t('settings.currentFolder') }}</p>
          </div>
        </div>

        <div class="bg-neutral-50 dark:bg-neutral-800/50 rounded-xl p-4 mb-4">
          <p class="text-sm text-neutral-900 dark:text-white font-mono truncate">
            {{ isLoadingPath ? '...' : downloadPath }}
          </p>
        </div>

        <button
          @click="selectDownloadFolder"
          class="w-full flex items-center justify-center gap-2 px-4 py-3 rounded-xl bg-neutral-900 dark:bg-white text-white dark:text-neutral-900 font-medium hover:bg-neutral-800 dark:hover:bg-neutral-100 transition-colors"
        >
          <FolderOpen class="w-5 h-5" />
          {{ t('settings.selectFolder') }}
        </button>
      </section>

      <!-- Connection Test Section -->
      <section class="card">
        <div class="flex items-center gap-3 mb-5">
          <div class="w-10 h-10 rounded-xl bg-neutral-100 dark:bg-neutral-800 flex items-center justify-center">
            <Wifi class="w-5 h-5 text-neutral-600 dark:text-neutral-400" />
          </div>
          <div>
            <h2 class="text-lg font-medium text-neutral-900 dark:text-white">
              {{ t('settings.connectionTest') }}
            </h2>
            <p class="text-xs text-neutral-500">{{ t('settings.serverStatus') }}</p>
          </div>
        </div>

        <div class="space-y-3 mb-4">
          <!-- Signaling Server -->
          <div class="flex items-center justify-between p-4 bg-neutral-50 dark:bg-neutral-800/50 rounded-xl">
            <div class="flex items-center gap-3">
              <div
                class="w-3 h-3 rounded-full"
                :class="{
                  'bg-neutral-300 dark:bg-neutral-600': signalingStatus === 'unknown',
                  'bg-green-500': signalingStatus === 'online',
                  'bg-red-500': signalingStatus === 'offline'
                }"
              />
              <span class="text-sm font-medium text-neutral-700 dark:text-neutral-300">
                {{ t('settings.signaling') }}
              </span>
            </div>
            <div class="flex items-center gap-3">
              <span v-if="signalingLatency !== null" class="text-xs text-neutral-500 font-mono">
                {{ signalingLatency }}ms
              </span>
              <CheckCircle v-if="signalingStatus === 'online'" class="w-5 h-5 text-green-500" />
              <XCircle v-else-if="signalingStatus === 'offline'" class="w-5 h-5 text-red-500" />
            </div>
          </div>

          <!-- Relay Server -->
          <div class="flex items-center justify-between p-4 bg-neutral-50 dark:bg-neutral-800/50 rounded-xl">
            <div class="flex items-center gap-3">
              <div
                class="w-3 h-3 rounded-full"
                :class="{
                  'bg-neutral-300 dark:bg-neutral-600': relayStatus === 'unknown',
                  'bg-green-500': relayStatus === 'online',
                  'bg-red-500': relayStatus === 'offline'
                }"
              />
              <span class="text-sm font-medium text-neutral-700 dark:text-neutral-300">
                {{ t('settings.relay') }}
              </span>
            </div>
            <div class="flex items-center gap-3">
              <span v-if="relayLatency !== null" class="text-xs text-neutral-500 font-mono">
                {{ relayLatency }}ms
              </span>
              <CheckCircle v-if="relayStatus === 'online'" class="w-5 h-5 text-green-500" />
              <XCircle v-else-if="relayStatus === 'offline'" class="w-5 h-5 text-red-500" />
            </div>
          </div>
        </div>

        <button
          @click="testConnection"
          :disabled="isTestingConnection"
          class="w-full flex items-center justify-center gap-2 px-4 py-3 rounded-xl border-2 border-neutral-200 dark:border-neutral-700 text-neutral-700 dark:text-neutral-300 font-medium hover:bg-neutral-50 dark:hover:bg-neutral-800 transition-colors disabled:opacity-50"
        >
          <Loader2 v-if="isTestingConnection" class="w-5 h-5 animate-spin" />
          <RefreshCw v-else class="w-5 h-5" />
          {{ isTestingConnection ? t('settings.testing') : t('settings.testConnection') }}
        </button>
      </section>

      <!-- About Section -->
      <section class="card">
        <h2 class="text-lg font-medium text-neutral-900 dark:text-white mb-4">
          {{ t('settings.about') }}
        </h2>
        <div class="text-sm text-neutral-600 dark:text-neutral-400 space-y-2">
          <p class="font-medium">SecureBeam v{{ appVersion }}</p>
          <p>
            End-to-end encrypted file transfer using the Magic Wormhole protocol.
          </p>
        </div>
      </section>
    </div>
  </main>
</template>
