import { defineStore } from 'pinia'
import { ref } from 'vue'

export const useThemeStore = defineStore('theme', () => {
  // Check for saved preference or system preference
  const getInitialTheme = (): 'light' | 'dark' => {
    const saved = localStorage.getItem('theme')
    if (saved === 'dark' || saved === 'light') {
      return saved
    }
    // Check system preference
    if (window.matchMedia('(prefers-color-scheme: dark)').matches) {
      return 'dark'
    }
    return 'light'
  }

  const theme = ref<'light' | 'dark'>(getInitialTheme())

  const isDark = () => theme.value === 'dark'

  function setTheme(newTheme: 'light' | 'dark') {
    theme.value = newTheme
    localStorage.setItem('theme', newTheme)
    applyTheme(newTheme)
  }

  function toggleTheme() {
    setTheme(theme.value === 'dark' ? 'light' : 'dark')
  }

  function applyTheme(t: 'light' | 'dark') {
    if (t === 'dark') {
      document.documentElement.classList.add('dark')
    } else {
      document.documentElement.classList.remove('dark')
    }
  }

  // Apply theme on initialization
  applyTheme(theme.value)

  // Watch for system preference changes
  window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', (e) => {
    // Only auto-switch if user hasn't manually set a preference
    if (!localStorage.getItem('theme')) {
      setTheme(e.matches ? 'dark' : 'light')
    }
  })

  return {
    theme,
    isDark,
    setTheme,
    toggleTheme
  }
})
