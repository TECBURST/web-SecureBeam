<script setup lang="ts">
import { RouterLink } from 'vue-router'
import { Upload, Download, Shield } from 'lucide-vue-next'
</script>

<template>
  <main class="flex-1 container mx-auto px-4 sm:px-6 py-12 sm:py-16 max-w-4xl">
    <!-- Hero Section -->
    <div class="text-center mb-12">
      <!-- Icon -->
      <div class="w-20 h-20 mx-auto mb-6 rounded-full bg-neutral-100 dark:bg-neutral-800 flex items-center justify-center">
        <Shield class="w-10 h-10 text-neutral-900 dark:text-white" />
      </div>

      <h1 class="text-3xl sm:text-4xl font-semibold tracking-tight text-neutral-900 dark:text-white mb-4">
        Secure File Transfer
      </h1>
      <p class="text-base sm:text-lg text-neutral-600 dark:text-neutral-400 max-w-xl mx-auto">
        Transfer files securely between devices using end-to-end encryption. No account required.
      </p>
    </div>

    <!-- Action Cards -->
    <div class="grid sm:grid-cols-2 gap-6 max-w-2xl mx-auto">
      <!-- Send Card -->
      <RouterLink to="/send" class="card hover:shadow-lg transition-all duration-200 group cursor-pointer">
        <div class="flex flex-col items-center text-center">
          <div class="w-14 h-14 rounded-full bg-neutral-100 dark:bg-neutral-800 flex items-center justify-center mb-4 group-hover:bg-neutral-200 dark:group-hover:bg-neutral-700 transition-colors">
            <Upload class="w-7 h-7 text-neutral-900 dark:text-white" />
          </div>
          <h2 class="text-xl font-semibold text-neutral-900 dark:text-white mb-2">
            Send Files
          </h2>
          <p class="text-sm text-neutral-600 dark:text-neutral-400">
            Select files or folders to share securely
          </p>
        </div>
      </RouterLink>

      <!-- Receive Card -->
      <RouterLink to="/receive" class="card hover:shadow-lg transition-all duration-200 group cursor-pointer">
        <div class="flex flex-col items-center text-center">
          <div class="w-14 h-14 rounded-full bg-neutral-100 dark:bg-neutral-800 flex items-center justify-center mb-4 group-hover:bg-neutral-200 dark:group-hover:bg-neutral-700 transition-colors">
            <Download class="w-7 h-7 text-neutral-900 dark:text-white" />
          </div>
          <h2 class="text-xl font-semibold text-neutral-900 dark:text-white mb-2">
            Receive Files
          </h2>
          <p class="text-sm text-neutral-600 dark:text-neutral-400">
            Enter a code to receive files
          </p>
        </div>
      </RouterLink>
    </div>

    <!-- Info Section -->
    <div class="mt-12 text-center">
      <p class="text-sm text-neutral-500 dark:text-neutral-500">
        Files are encrypted end-to-end using the Magic Wormhole protocol
      </p>
    </div>
  </main>
</template>
