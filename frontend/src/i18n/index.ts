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

export const languages = [
  { code: 'en', name: 'English' },
  { code: 'de', name: 'Deutsch' },
  { code: 'fr', name: 'Français' },
  { code: 'es', name: 'Español' },
  { code: 'it', name: 'Italiano' },
  { code: 'ru', name: 'Русский' },
  { code: 'nl', name: 'Nederlands' },
  { code: 'pl', name: 'Polski' },
  { code: 'zh', name: '中文' }
]

// Get saved language or detect from browser
function getDefaultLocale(): string {
  const saved = localStorage.getItem('locale')
  if (saved && languages.some(l => l.code === saved)) {
    return saved
  }

  const browserLang = navigator.language.split('-')[0]
  if (languages.some(l => l.code === browserLang)) {
    return browserLang
  }

  return 'en'
}

export const i18n = createI18n({
  legacy: false,
  locale: getDefaultLocale(),
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

type LocaleCode = 'en' | 'de' | 'fr' | 'es' | 'it' | 'ru' | 'nl' | 'pl' | 'zh'

export function setLocale(locale: string) {
  i18n.global.locale.value = locale as LocaleCode
  localStorage.setItem('locale', locale)
  document.documentElement.lang = locale
}
