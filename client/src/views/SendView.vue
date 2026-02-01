<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import { useRouter } from 'vue-router'
import { invoke } from '@tauri-apps/api/core'
import { listen, type UnlistenFn } from '@tauri-apps/api/event'
import { open } from '@tauri-apps/plugin-dialog'
import { ArrowLeft, Upload, File, Folder, Copy, Check, Loader2 } from 'lucide-vue-next'

const router = useRouter()

// State
const selectedFile = ref<{ name: string; size: number; path: string; isDirectory: boolean } | null>(null)
const wormholeCode = ref<string | null>(null)
const isLoading = ref(false)
const isCopied = ref(false)
const transferProgress = ref(0)
const transferSpeed = ref(0)
const transferEta = ref<number | null>(null)
const bytesTransferred = ref(0)
const totalBytes = ref(0)
const status = ref<'idle' | 'preparing' | 'waiting' | 'transferring' | 'complete' | 'error'>('idle')
const statusMessage = ref<string>('')
const errorMessage = ref<string | null>(null)
const isDragOver = ref(false)

// Event listeners
let unlistenStatus: UnlistenFn | null = null
let unlistenProgress: UnlistenFn | null = null
let unlistenComplete: UnlistenFn | null = null

// Setup event listeners
onMounted(async () => {
  unlistenStatus = await listen<string>('transfer-status', (event) => {
    statusMessage.value = event.payload
    if (event.payload.includes('Transferring')) {
      status.value = 'transferring'
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
    status.value = 'transferring'
  })

  unlistenComplete = await listen('transfer-complete', () => {
    status.value = 'complete'
    transferProgress.value = 100
    statusMessage.value = 'Transfer complete!'
  })
})

// Cleanup
onUnmounted(() => {
  unlistenStatus?.()
  unlistenProgress?.()
  unlistenComplete?.()
})

// Open file picker dialog
async function openFilePicker() {
  try {
    const selected = await open({
      multiple: false,
      directory: false,
      title: 'Select a file to send'
    })

    if (selected) {
      const path = selected as string
      const name = path.split(/[/\\]/).pop() || 'Unknown'

      try {
        const info = await invoke<{ name: string; size: number; compressed: boolean }>('prepare_file', { path })
        selectedFile.value = {
          name: info.name,
          size: info.size,
          path: path,
          isDirectory: false
        }
      } catch {
        selectedFile.value = {
          name,
          size: 0,
          path: path,
          isDirectory: false
        }
      }
    }
  } catch (error) {
    console.error('File picker error:', error)
    errorMessage.value = 'Could not open file picker'
  }
}

// Open folder picker dialog
async function openFolderPicker() {
  try {
    const selected = await open({
      multiple: false,
      directory: true,
      title: 'Select a folder to send'
    })

    if (selected) {
      const path = selected as string
      const name = path.split(/[/\\]/).pop() || 'Folder'

      try {
        const info = await invoke<{ name: string; size: number; compressed: boolean }>('prepare_directory', { path })
        selectedFile.value = {
          name: info.name,
          size: info.size,
          path: path,
          isDirectory: true
        }
      } catch {
        selectedFile.value = {
          name,
          size: 0,
          path: path,
          isDirectory: true
        }
      }
    }
  } catch (error) {
    console.error('Folder picker error:', error)
    errorMessage.value = 'Could not open folder picker'
  }
}

// Handle drag over
function handleDragOver(event: DragEvent) {
  event.preventDefault()
  isDragOver.value = true
}

// Handle drag leave
function handleDragLeave(event: DragEvent) {
  event.preventDefault()
  isDragOver.value = false
}

// Handle file drop
async function handleDrop(event: DragEvent) {
  event.preventDefault()
  isDragOver.value = false

  const files = event.dataTransfer?.files
  if (files && files.length > 0) {
    errorMessage.value = 'Please use the buttons below to select files.'
    setTimeout(() => {
      errorMessage.value = null
    }, 3000)
  }
}

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

// Start transfer
async function startTransfer() {
  if (!selectedFile.value) return

  try {
    isLoading.value = true
    status.value = 'preparing'
    errorMessage.value = null
    statusMessage.value = 'Generating code...'

    // Generate code first
    const code = await invoke<string>('generate_code')
    wormholeCode.value = code
    status.value = 'waiting'
    statusMessage.value = 'Waiting for receiver...'

    // Start the actual transfer (runs in background, listens for receiver)
    await invoke('start_send', {
      path: selectedFile.value.path,
      code: code,
      isDirectory: selectedFile.value.isDirectory
    })

  } catch (error) {
    console.error('Transfer error:', error)
    status.value = 'error'
    errorMessage.value = String(error)
  } finally {
    isLoading.value = false
  }
}

// Copy code to clipboard
async function copyCode() {
  if (!wormholeCode.value) return
  try {
    await navigator.clipboard.writeText(wormholeCode.value)
    isCopied.value = true
    setTimeout(() => {
      isCopied.value = false
    }, 2000)
  } catch (error) {
    console.error('Copy failed:', error)
  }
}

// Cancel and go back
function cancel() {
  router.push('/')
}

// Clear selection
function clearSelection() {
  selectedFile.value = null
  wormholeCode.value = null
  status.value = 'idle'
  statusMessage.value = ''
  errorMessage.value = null
  transferProgress.value = 0
  transferSpeed.value = 0
  transferEta.value = null
}

// Send another
function sendAnother() {
  clearSelection()
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
      Send Files
    </h1>

    <!-- Drop Zone (when no file selected) -->
    <div
      v-if="!selectedFile"
      class="drop-zone flex flex-col items-center justify-center"
      :class="{ 'ring-2 ring-neutral-400 dark:ring-neutral-500': isDragOver }"
      @drop="handleDrop"
      @dragover="handleDragOver"
      @dragleave="handleDragLeave"
    >
      <Upload class="w-12 h-12 text-neutral-400 dark:text-neutral-500 mb-4" />
      <p class="text-lg font-medium text-neutral-700 dark:text-neutral-300 mb-2">
        Select a file or folder to share
      </p>
      <p class="text-sm text-neutral-500 dark:text-neutral-500 mb-6">
        Click one of the buttons below
      </p>

      <!-- Browse Buttons -->
      <div class="flex gap-3">
        <button
          @click="openFilePicker"
          class="btn btn-primary flex items-center gap-2"
        >
          <File class="w-4 h-4" />
          Browse Files
        </button>
        <button
          @click="openFolderPicker"
          class="btn btn-secondary flex items-center gap-2"
        >
          <Folder class="w-4 h-4" />
          Browse Folder
        </button>
      </div>

      <!-- Error Message -->
      <div v-if="errorMessage" class="mt-4 p-3 rounded-lg bg-amber-50 dark:bg-amber-900/20 border border-amber-200 dark:border-amber-800">
        <p class="text-amber-700 dark:text-amber-400 text-sm">{{ errorMessage }}</p>
      </div>
    </div>

    <!-- File Selected -->
    <div v-else class="space-y-6">
      <!-- File Info Card -->
      <div class="card !p-6">
        <div class="flex items-start gap-4">
          <div class="w-12 h-12 rounded-xl bg-neutral-100 dark:bg-neutral-800 flex items-center justify-center flex-shrink-0">
            <Folder v-if="selectedFile.isDirectory" class="w-6 h-6 text-neutral-600 dark:text-neutral-400" />
            <File v-else class="w-6 h-6 text-neutral-600 dark:text-neutral-400" />
          </div>
          <div class="flex-1 min-w-0">
            <h3 class="font-semibold text-neutral-900 dark:text-white truncate">
              {{ selectedFile.name }}
            </h3>
            <p class="text-sm text-neutral-500 dark:text-neutral-500">
              {{ formatSize(selectedFile.size) }}
            </p>
          </div>
          <button
            v-if="status === 'idle'"
            @click="clearSelection"
            class="text-neutral-400 hover:text-neutral-600 dark:hover:text-neutral-300 text-xl"
          >
            &times;
          </button>
        </div>
      </div>

      <!-- Wormhole Code (when waiting) -->
      <div v-if="wormholeCode && status !== 'complete'" class="card !p-6 text-center">
        <p class="text-sm text-neutral-500 dark:text-neutral-500 mb-3">
          Share this code with the receiver
        </p>
        <div class="flex items-center justify-center gap-3">
          <span class="code-display">{{ wormholeCode }}</span>
          <button
            @click="copyCode"
            class="p-2 rounded-lg bg-neutral-100 dark:bg-neutral-800 hover:bg-neutral-200 dark:hover:bg-neutral-700 transition-colors"
          >
            <Check v-if="isCopied" class="w-5 h-5 text-green-600" />
            <Copy v-else class="w-5 h-5 text-neutral-600 dark:text-neutral-400" />
          </button>
        </div>
        <p class="text-sm text-neutral-500 dark:text-neutral-500 mt-4">
          {{ statusMessage || 'Waiting for receiver to connect...' }}
        </p>
      </div>

      <!-- Progress Bar (when transferring) -->
      <div v-if="status === 'transferring'" class="card !p-6">
        <div class="flex justify-between text-sm mb-2">
          <span class="text-neutral-600 dark:text-neutral-400">{{ statusMessage || 'Transferring...' }}</span>
          <span class="text-neutral-900 dark:text-white font-medium">{{ transferProgress }}%</span>
        </div>
        <div class="progress-bar">
          <div class="progress-bar-fill" :style="{ width: `${transferProgress}%` }"></div>
        </div>
        <!-- Transfer Stats -->
        <div class="flex justify-between text-xs text-neutral-500 dark:text-neutral-500 mt-3">
          <span>{{ formatSize(bytesTransferred) }} / {{ formatSize(totalBytes) }}</span>
          <span v-if="transferSpeed > 0">{{ transferSpeed.toFixed(2) }} MB/s</span>
        </div>
        <div v-if="transferEta" class="text-xs text-neutral-500 dark:text-neutral-500 mt-1 text-center">
          {{ formatEta(transferEta) }}
        </div>
      </div>

      <!-- Transfer Complete -->
      <div v-if="status === 'complete'" class="card !p-6 text-center">
        <div class="w-16 h-16 rounded-full bg-green-100 dark:bg-green-900/30 flex items-center justify-center mx-auto mb-4">
          <Check class="w-8 h-8 text-green-600 dark:text-green-400" />
        </div>
        <h3 class="text-xl font-semibold text-neutral-900 dark:text-white mb-2">
          Transfer Complete
        </h3>
        <p class="text-neutral-500 dark:text-neutral-500">
          File sent successfully
        </p>
      </div>

      <!-- Error Message -->
      <div v-if="errorMessage" class="card !p-6 border-red-200 dark:border-red-800 bg-red-50 dark:bg-red-900/20">
        <p class="text-red-600 dark:text-red-400">{{ errorMessage }}</p>
      </div>

      <!-- Action Buttons -->
      <div v-if="status === 'idle'" class="flex gap-4">
        <button @click="cancel" class="btn btn-secondary flex-1">
          Cancel
        </button>
        <button
          @click="startTransfer"
          class="btn btn-primary flex-1"
          :disabled="isLoading"
        >
          <Loader2 v-if="isLoading" class="w-4 h-4 mr-2 animate-spin" />
          Generate Code
        </button>
      </div>

      <!-- Send Another Button -->
      <div v-if="status === 'complete'" class="flex gap-4">
        <button @click="sendAnother" class="btn btn-primary w-full">
          Send Another File
        </button>
      </div>
    </div>
  </main>
</template>
