<script setup lang="ts">
import { computed } from 'vue'

const props = withDefaults(defineProps<{
  code: string
  size?: 'sm' | 'md' | 'lg'
}>(), {
  size: 'sm'
})

const sizeClass = computed(() => {
  switch (props.size) {
    case 'lg': return 'w-8 h-5'
    case 'md': return 'w-6 h-4'
    default: return 'w-5 h-3'
  }
})
</script>

<template>
  <!-- US Flag -->
  <svg v-if="code === 'en'" viewBox="0 0 190 100" :class="sizeClass" class="rounded-sm overflow-hidden ring-1 ring-black/20">
    <!-- Stripes -->
    <rect width="190" height="100" fill="#B22234"/>
    <g fill="#fff">
      <rect y="7.69" width="190" height="7.69"/>
      <rect y="23.08" width="190" height="7.69"/>
      <rect y="38.46" width="190" height="7.69"/>
      <rect y="53.85" width="190" height="7.69"/>
      <rect y="69.23" width="190" height="7.69"/>
      <rect y="84.62" width="190" height="7.69"/>
    </g>
    <!-- Blue canton -->
    <rect width="76" height="53.85" fill="#3C3B6E"/>
    <!-- Stars (simplified 5x4 grid for clarity) -->
    <g fill="#fff">
      <circle cx="9.5" cy="6.7" r="2.5"/>
      <circle cx="28.5" cy="6.7" r="2.5"/>
      <circle cx="47.5" cy="6.7" r="2.5"/>
      <circle cx="66.5" cy="6.7" r="2.5"/>
      <circle cx="19" cy="13.5" r="2.5"/>
      <circle cx="38" cy="13.5" r="2.5"/>
      <circle cx="57" cy="13.5" r="2.5"/>
      <circle cx="9.5" cy="20.2" r="2.5"/>
      <circle cx="28.5" cy="20.2" r="2.5"/>
      <circle cx="47.5" cy="20.2" r="2.5"/>
      <circle cx="66.5" cy="20.2" r="2.5"/>
      <circle cx="19" cy="27" r="2.5"/>
      <circle cx="38" cy="27" r="2.5"/>
      <circle cx="57" cy="27" r="2.5"/>
      <circle cx="9.5" cy="33.7" r="2.5"/>
      <circle cx="28.5" cy="33.7" r="2.5"/>
      <circle cx="47.5" cy="33.7" r="2.5"/>
      <circle cx="66.5" cy="33.7" r="2.5"/>
      <circle cx="19" cy="40.5" r="2.5"/>
      <circle cx="38" cy="40.5" r="2.5"/>
      <circle cx="57" cy="40.5" r="2.5"/>
      <circle cx="9.5" cy="47.2" r="2.5"/>
      <circle cx="28.5" cy="47.2" r="2.5"/>
      <circle cx="47.5" cy="47.2" r="2.5"/>
      <circle cx="66.5" cy="47.2" r="2.5"/>
    </g>
  </svg>

  <!-- German Flag -->
  <svg v-else-if="code === 'de'" viewBox="0 0 5 3" :class="sizeClass" class="rounded-sm overflow-hidden ring-1 ring-black/20">
    <rect width="5" height="1" fill="#000"/>
    <rect y="1" width="5" height="1" fill="#DD0000"/>
    <rect y="2" width="5" height="1" fill="#FFCE00"/>
  </svg>

  <!-- French Flag -->
  <svg v-else-if="code === 'fr'" viewBox="0 0 3 2" :class="sizeClass" class="rounded-sm overflow-hidden ring-1 ring-black/20">
    <rect width="1" height="2" fill="#002395"/>
    <rect x="1" width="1" height="2" fill="#fff"/>
    <rect x="2" width="1" height="2" fill="#ED2939"/>
  </svg>

  <!-- Spanish Flag -->
  <svg v-else-if="code === 'es'" viewBox="0 0 750 500" :class="sizeClass" class="rounded-sm overflow-hidden ring-1 ring-black/20">
    <rect width="750" height="500" fill="#c60b1e"/>
    <rect y="125" width="750" height="250" fill="#ffc400"/>
  </svg>

  <!-- Italian Flag -->
  <svg v-else-if="code === 'it'" viewBox="0 0 3 2" :class="sizeClass" class="rounded-sm overflow-hidden ring-1 ring-black/20">
    <rect width="1" height="2" fill="#009246"/>
    <rect x="1" width="1" height="2" fill="#fff"/>
    <rect x="2" width="1" height="2" fill="#ce2b37"/>
  </svg>

  <!-- Russian Flag -->
  <svg v-else-if="code === 'ru'" viewBox="0 0 9 6" :class="sizeClass" class="rounded-sm overflow-hidden ring-1 ring-black/20">
    <rect width="9" height="2" fill="#fff"/>
    <rect y="2" width="9" height="2" fill="#0039A6"/>
    <rect y="4" width="9" height="2" fill="#D52B1E"/>
  </svg>

  <!-- Dutch Flag -->
  <svg v-else-if="code === 'nl'" viewBox="0 0 9 6" :class="sizeClass" class="rounded-sm overflow-hidden ring-1 ring-black/20">
    <rect width="9" height="2" fill="#AE1C28"/>
    <rect y="2" width="9" height="2" fill="#fff"/>
    <rect y="4" width="9" height="2" fill="#21468B"/>
  </svg>

  <!-- Polish Flag -->
  <svg v-else-if="code === 'pl'" viewBox="0 0 8 5" :class="sizeClass" class="rounded-sm overflow-hidden ring-1 ring-black/20">
    <rect width="8" height="2.5" fill="#fff"/>
    <rect y="2.5" width="8" height="2.5" fill="#DC143C"/>
  </svg>

  <!-- Chinese Flag -->
  <svg v-else-if="code === 'zh'" viewBox="0 0 30 20" :class="sizeClass" class="rounded-sm overflow-hidden ring-1 ring-black/20">
    <rect width="30" height="20" fill="#DE2910"/>
    <g fill="#FFDE00">
      <polygon points="5,4 5.9,6.5 3.2,4.8 6.8,4.8 4.1,6.5"/>
      <polygon points="10,1 10.4,2.1 9.2,1.5 10.8,1.5 9.6,2.1" transform="rotate(23.5 10 2)"/>
      <polygon points="12,3 12.4,4.1 11.2,3.5 12.8,3.5 11.6,4.1" transform="rotate(-10 12 4)"/>
      <polygon points="12,6 12.4,7.1 11.2,6.5 12.8,6.5 11.6,7.1" transform="rotate(20 12 7)"/>
      <polygon points="10,8 10.4,9.1 9.2,8.5 10.8,8.5 9.6,9.1" transform="rotate(40 10 9)"/>
    </g>
  </svg>

  <!-- Fallback -->
  <div v-else :class="sizeClass" class="bg-neutral-300 rounded-sm ring-1 ring-black/20"/>
</template>
