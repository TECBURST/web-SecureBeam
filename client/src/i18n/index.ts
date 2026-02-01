import { createI18n } from 'vue-i18n'

import en from './locales/en'
import de from './locales/de'
import fr from './locales/fr'
import es from './locales/es'
import zh from './locales/zh'

export type Locale = 'en' | 'de' | 'fr' | 'es' | 'zh'

export const SUPPORTED_LOCALES: Locale[] = ['en', 'de', 'fr', 'es', 'zh']

export const LOCALE_NAMES: Record<Locale, string> = {
  en: 'English',
  de: 'Deutsch',
  fr: 'Francais',
  es: 'Espanol',
  zh: '中文'
}

function getInitialLocale(): Locale {
  // Check localStorage first
  const stored = localStorage.getItem('securebeam-locale')
  if (stored && SUPPORTED_LOCALES.includes(stored as Locale)) {
    return stored as Locale
  }

  // Try to detect from browser
  const browserLang = navigator.language.split('-')[0]
  if (SUPPORTED_LOCALES.includes(browserLang as Locale)) {
    return browserLang as Locale
  }

  return 'en'
}

const i18n = createI18n({
  legacy: false,
  locale: getInitialLocale(),
  fallbackLocale: 'en',
  messages: {
    en,
    de,
    fr,
    es,
    zh
  }
})

export function setLocale(locale: Locale) {
  i18n.global.locale.value = locale
  localStorage.setItem('securebeam-locale', locale)
  document.documentElement.lang = locale
}

export function getCurrentLocale(): Locale {
  return i18n.global.locale.value as Locale
}

export default i18n
