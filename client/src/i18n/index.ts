import { createI18n } from 'vue-i18n'

import en from './locales/en'
import de from './locales/de'
import fr from './locales/fr'
import es from './locales/es'
import it from './locales/it'
import ru from './locales/ru'
import nl from './locales/nl'
import pl from './locales/pl'
import zh from './locales/zh'

export type Locale = 'en' | 'de' | 'fr' | 'es' | 'it' | 'ru' | 'nl' | 'pl' | 'zh'

export const languages = [
  { code: 'en' as Locale, name: 'English' },
  { code: 'de' as Locale, name: 'Deutsch' },
  { code: 'fr' as Locale, name: 'Francais' },
  { code: 'es' as Locale, name: 'Espanol' },
  { code: 'it' as Locale, name: 'Italiano' },
  { code: 'ru' as Locale, name: 'Russkiy' },
  { code: 'nl' as Locale, name: 'Nederlands' },
  { code: 'pl' as Locale, name: 'Polski' },
  { code: 'zh' as Locale, name: '中文' }
]

export const SUPPORTED_LOCALES: Locale[] = languages.map(l => l.code)

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
    it,
    ru,
    nl,
    pl,
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
