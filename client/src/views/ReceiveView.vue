<script setup lang="ts">
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import { invoke } from '@tauri-apps/api/core'
import { ArrowLeft, Download, Loader2, Check, X, File } from 'lucide-vue-next'

const router = useRouter()

// State
const codeInput = ref('')
const isLoading = ref(false)
const status = ref<'idle' | 'connecting' | 'receiving' | 'complete' | 'error'>('idle')
const errorMessage = ref<string | null>(null)
const fileOffer = ref<{ name: string; size: number } | null>(null)
const transferProgress = ref(0)

// Format file size
function formatSize(bytes: number): string {
  const units = ['B', 'KB', 'MB', 'GB']
  let size = bytes
  let unitIndex = 0
  while (size >= 1024 && unitIndex < units.length - 1) {
    size /= 1024
    unitIndex++
  }
  return `${size.toFixed(unitIndex > 0 ? 2 : 0)} ${units[unitIndex]}`
}

// Connect with code
async function connect() {
  if (!codeInput.value.trim()) return

  try {
    isLoading.value = true
    status.value = 'connecting'
    errorMessage.value = null

    // Parse and validate code
    const result = await invoke<[string, string]>('parse_code', {
      code: codeInput.value.trim().toLowerCase(),
    })

    // TODO: Connect to sender via securebeam-core
    // For now, simulate receiving an offer
    await new Promise((resolve) => setTimeout(resolve, 1500))

    // Simulated file offer
    fileOffer.value = {
      name: 'example-file.txt',
      size: 1024 * 1024 * 5, // 5 MB
    }

  } catch (error) {
    console.error('Connection error:', error)
    status.value = 'error'
    errorMessage.value = String(error)
  } finally {
    isLoading.value = false
  }
}

// Accept the transfer
async function acceptTransfer() {
  try {
    status.value = 'receiving'

    // TODO: Accept and receive file via securebeam-core
    // Simulate progress
    for (let i = 0; i <= 100; i += 10) {
      await new Promise((resolve) => setTimeout(resolve, 200))
      transferProgress.value = i
    }

    status.value = 'complete'
  } catch (error) {
    console.error('Transfer error:', error)
    status.value = 'error'
    errorMessage.value = String(error)
  }
}

// Reject the transfer
function rejectTransfer() {
  fileOffer.value = null
  status.value = 'idle'
  codeInput.value = ''
}

// Cancel and go back
function cancel() {
  router.push('/')
}

// Reset
function reset() {
  codeInput.value = ''
  status.value = 'idle'
  errorMessage.value = null
  fileOffer.value = null
  transferProgress.value = 0
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

    <!-- Code Input (initial state) -->
    <div v-if="!fileOffer && status !== 'complete'" class="space-y-6">
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
          :disabled="isLoading || !codeInput.trim()"
        >
          <Loader2 v-if="isLoading" class="w-4 h-4 mr-2 animate-spin" />
          Connect
        </button>
      </div>
    </div>

    <!-- File Offer -->
    <div v-if="fileOffer && status !== 'complete' && status !== 'receiving'" class="space-y-6">
      <div class="card !p-6">
        <p class="text-sm text-neutral-500 dark:text-neutral-500 mb-4">
          Incoming file transfer
        </p>
        <div class="flex items-start gap-4">
          <div class="w-12 h-12 rounded-xl bg-neutral-100 dark:bg-neutral-800 flex items-center justify-center flex-shrink-0">
            <File class="w-6 h-6 text-neutral-600 dark:text-neutral-400" />
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

      <!-- Accept/Reject Buttons -->
      <div class="flex gap-4">
        <button @click="rejectTransfer" class="btn btn-secondary flex-1">
          <X class="w-4 h-4 mr-2" />
          Reject
        </button>
        <button @click="acceptTransfer" class="btn btn-primary flex-1">
          <Check class="w-4 h-4 mr-2" />
          Accept
        </button>
      </div>
    </div>

    <!-- Progress (receiving) -->
    <div v-if="status === 'receiving'" class="space-y-6">
      <div class="card !p-6">
        <div class="flex justify-between text-sm mb-2">
          <span class="text-neutral-600 dark:text-neutral-400">Receiving...</span>
          <span class="text-neutral-900 dark:text-white font-medium">{{ transferProgress }}%</span>
        </div>
        <div class="progress-bar">
          <div class="progress-bar-fill" :style="{ width: `${transferProgress}%` }"></div>
        </div>
        <p class="text-sm text-neutral-500 dark:text-neutral-500 mt-3">
          {{ fileOffer?.name }}
        </p>
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
        <p class="text-neutral-500 dark:text-neutral-500">
          File saved successfully
        </p>
      </div>

      <button @click="reset" class="btn btn-primary w-full">
        Receive Another File
      </button>
    </div>
  </main>
</template>
