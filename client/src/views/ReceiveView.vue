<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import { useRouter } from 'vue-router'
import { invoke } from '@tauri-apps/api/core'
import { listen, type UnlistenFn } from '@tauri-apps/api/event'
import { open } from '@tauri-apps/plugin-dialog'
import { ArrowLeft, Loader2, Check, File, Folder, Download } from 'lucide-vue-next'

const router = useRouter()

// State
const codeInput = ref('')
const saveFolder = ref<string | null>(null)
const isLoading = ref(false)
const status = ref<'idle' | 'connecting' | 'receiving' | 'complete' | 'error'>('idle')
const statusMessage = ref<string>('')
const errorMessage = ref<string | null>(null)
const fileOffer = ref<{ name: string; size: number; is_directory: boolean } | null>(null)
const transferProgress = ref(0)
const transferSpeed = ref(0)
const transferEta = ref<number | null>(null)
const bytesTransferred = ref(0)
const totalBytes = ref(0)

// Event listeners
let unlistenStatus: UnlistenFn | null = null
let unlistenProgress: UnlistenFn | null = null
let unlistenComplete: UnlistenFn | null = null
let unlistenOffer: UnlistenFn | null = null

// Setup event listeners
onMounted(async () => {
  unlistenStatus = await listen<string>('transfer-status', (event) => {
    statusMessage.value = event.payload
    if (event.payload.includes('Receiving')) {
      status.value = 'receiving'
    }
  })

  unlistenProgress = await listen<{
    bytes_transferred: number
    total_bytes: number
    percentage: number
    speed_mbps: number
    eta_seconds: number | null
    status: string
  }>('transfer-progress', (event) => {
    transferProgress.value = Math.round(event.payload.percentage)
    transferSpeed.value = event.payload.speed_mbps
    transferEta.value = event.payload.eta_seconds
    bytesTransferred.value = event.payload.bytes_transferred
    totalBytes.value = event.payload.total_bytes
    status.value = 'receiving'
  })

  unlistenComplete = await listen('transfer-complete', () => {
    status.value = 'complete'
    transferProgress.value = 100
    statusMessage.value = 'Transfer complete!'
  })

  unlistenOffer = await listen<{
    name: string
    size: number
    compressed: boolean
    is_directory: boolean
  }>('file-offer', (event) => {
    fileOffer.value = {
      name: event.payload.name,
      size: event.payload.size,
      is_directory: event.payload.is_directory
    }
    totalBytes.value = event.payload.size
  })
})

// Cleanup
onUnmounted(() => {
  unlistenStatus?.()
  unlistenProgress?.()
  unlistenComplete?.()
  unlistenOffer?.()
})

// Format file size
function formatSize(bytes: number): string {
  if (bytes === 0) return 'Unknown size'
  const units = ['B', 'KB', 'MB', 'GB']
  let size = bytes
  let unitIndex = 0
  while (size >= 1024 && unitIndex < units.length - 1) {
    size /= 1024
    unitIndex++
  }
  return `${size.toFixed(unitIndex > 0 ? 2 : 0)} ${units[unitIndex]}`
}

// Format ETA
function formatEta(seconds: number | null): string {
  if (seconds === null || seconds <= 0) return ''
  if (seconds < 60) return `${Math.round(seconds)}s remaining`
  if (seconds < 3600) return `${Math.round(seconds / 60)}m remaining`
  return `${Math.round(seconds / 3600)}h remaining`
}

// Select save folder
async function selectSaveFolder() {
  try {
    const selected = await open({
      multiple: false,
      directory: true,
      title: 'Select folder to save file'
    })

    if (selected) {
      saveFolder.value = selected as string
    }
  } catch (error) {
    console.error('Folder picker error:', error)
    errorMessage.value = 'Could not open folder picker'
  }
}

// Connect with code and start receiving
async function connect() {
  if (!codeInput.value.trim()) return

  // Require save folder
  if (!saveFolder.value) {
    errorMessage.value = 'Please select a folder to save the file first'
    return
  }

  try {
    isLoading.value = true
    status.value = 'connecting'
    statusMessage.value = 'Connecting...'
    errorMessage.value = null

    // Parse and validate code first
    await invoke<[string, string]>('parse_code', {
      code: codeInput.value.trim().toLowerCase(),
    })

    // Start receiving
    await invoke('start_receive', {
      code: codeInput.value.trim().toLowerCase(),
      savePath: saveFolder.value
    })

  } catch (error) {
    console.error('Connection error:', error)
    status.value = 'error'
    errorMessage.value = String(error)
    isLoading.value = false
  }
}

// Cancel and go back
function cancel() {
  router.push('/')
}

// Reset
function reset() {
  codeInput.value = ''
  saveFolder.value = null
  status.value = 'idle'
  statusMessage.value = ''
  errorMessage.value = null
  fileOffer.value = null
  transferProgress.value = 0
  transferSpeed.value = 0
  transferEta.value = null
  bytesTransferred.value = 0
  totalBytes.value = 0
  isLoading.value = false
}
</script>

<template>
  <main class="flex-1 container mx-auto px-4 sm:px-6 py-8 sm:py-12 max-w-2xl">
    <!-- Back Button -->
    <button
      @click="cancel"
      class="flex items-center gap-2 text-neutral-600 dark:text-neutral-400 hover:text-neutral-900 dark:hover:text-white mb-8 transition-colors"
    >
      <ArrowLeft class="w-4 h-4" />
      <span>Back</span>
    </button>

    <!-- Title -->
    <h1 class="text-2xl sm:text-3xl font-semibold text-neutral-900 dark:text-white mb-8">
      Receive Files
    </h1>

    <!-- Code Input and Setup (initial state) -->
    <div v-if="status === 'idle' || status === 'error'" class="space-y-6">
      <!-- Code Input -->
      <div class="card !p-6">
        <label class="block text-sm font-medium text-neutral-700 dark:text-neutral-300 mb-3">
          Enter the code from the sender
        </label>
        <input
          v-model="codeInput"
          type="text"
          placeholder="123-purple-sausages"
          class="input text-center font-mono text-lg"
          :disabled="isLoading"
          @keyup.enter="connect"
        />
      </div>

      <!-- Save Location -->
      <div class="card !p-6">
        <label class="block text-sm font-medium text-neutral-700 dark:text-neutral-300 mb-3">
          Save location
        </label>
        <div class="flex items-center gap-3">
          <div class="flex-1 px-4 py-3 rounded-lg bg-neutral-100 dark:bg-neutral-800 text-sm truncate">
            <span v-if="saveFolder" class="text-neutral-900 dark:text-white">{{ saveFolder }}</span>
            <span v-else class="text-neutral-500">No folder selected</span>
          </div>
          <button
            @click="selectSaveFolder"
            class="btn btn-secondary flex items-center gap-2"
            :disabled="isLoading"
          >
            <Folder class="w-4 h-4" />
            Browse
          </button>
        </div>
      </div>

      <!-- Error Message -->
      <div v-if="errorMessage" class="card !p-6 border-red-200 dark:border-red-800 bg-red-50 dark:bg-red-900/20">
        <p class="text-red-600 dark:text-red-400">{{ errorMessage }}</p>
      </div>

      <!-- Action Buttons -->
      <div class="flex gap-4">
        <button @click="cancel" class="btn btn-secondary flex-1">
          Cancel
        </button>
        <button
          @click="connect"
          class="btn btn-primary flex-1"
          :disabled="isLoading || !codeInput.trim() || !saveFolder"
        >
          <Loader2 v-if="isLoading" class="w-4 h-4 mr-2 animate-spin" />
          <Download v-else class="w-4 h-4 mr-2" />
          Connect & Receive
        </button>
      </div>
    </div>

    <!-- Connecting / Receiving -->
    <div v-if="status === 'connecting' || status === 'receiving'" class="space-y-6">
      <!-- File Info (if offer received) -->
      <div v-if="fileOffer" class="card !p-6">
        <div class="flex items-start gap-4">
          <div class="w-12 h-12 rounded-xl bg-neutral-100 dark:bg-neutral-800 flex items-center justify-center flex-shrink-0">
            <Folder v-if="fileOffer.is_directory" class="w-6 h-6 text-neutral-600 dark:text-neutral-400" />
            <File v-else class="w-6 h-6 text-neutral-600 dark:text-neutral-400" />
          </div>
          <div class="flex-1 min-w-0">
            <h3 class="font-semibold text-neutral-900 dark:text-white truncate">
              {{ fileOffer.name }}
            </h3>
            <p class="text-sm text-neutral-500 dark:text-neutral-500">
              {{ formatSize(fileOffer.size) }}
            </p>
          </div>
        </div>
      </div>

      <!-- Progress -->
      <div class="card !p-6">
        <div class="flex justify-between text-sm mb-2">
          <span class="text-neutral-600 dark:text-neutral-400">{{ statusMessage || 'Connecting...' }}</span>
          <span v-if="status === 'receiving'" class="text-neutral-900 dark:text-white font-medium">{{ transferProgress }}%</span>
        </div>
        <div class="progress-bar">
          <div class="progress-bar-fill" :style="{ width: `${transferProgress}%` }"></div>
        </div>
        <!-- Transfer Stats -->
        <div v-if="status === 'receiving'" class="flex justify-between text-xs text-neutral-500 dark:text-neutral-500 mt-3">
          <span>{{ formatSize(bytesTransferred) }} / {{ formatSize(totalBytes) }}</span>
          <span v-if="transferSpeed > 0">{{ transferSpeed.toFixed(2) }} MB/s</span>
        </div>
        <div v-if="transferEta && status === 'receiving'" class="text-xs text-neutral-500 dark:text-neutral-500 mt-1 text-center">
          {{ formatEta(transferEta) }}
        </div>
      </div>

      <!-- Spinner for connecting state -->
      <div v-if="status === 'connecting' && !fileOffer" class="flex justify-center">
        <Loader2 class="w-8 h-8 text-neutral-400 animate-spin" />
      </div>
    </div>

    <!-- Complete -->
    <div v-if="status === 'complete'" class="space-y-6">
      <div class="card !p-6 text-center">
        <div class="w-16 h-16 rounded-full bg-green-100 dark:bg-green-900/30 flex items-center justify-center mx-auto mb-4">
          <Check class="w-8 h-8 text-green-600 dark:text-green-400" />
        </div>
        <h3 class="text-xl font-semibold text-neutral-900 dark:text-white mb-2">
          Transfer Complete
        </h3>
        <p class="text-neutral-500 dark:text-neutral-500 mb-2">
          File saved successfully
        </p>
        <p v-if="saveFolder && fileOffer" class="text-sm text-neutral-400 dark:text-neutral-600 truncate">
          {{ saveFolder }}/{{ fileOffer.name }}
        </p>
      </div>

      <button @click="reset" class="btn btn-primary w-full">
        Receive Another File
      </button>
    </div>
  </main>
</template>
