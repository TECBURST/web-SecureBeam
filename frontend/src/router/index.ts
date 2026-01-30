import { createRouter, createWebHistory } from 'vue-router'
import HomeView from '@/views/HomeView.vue'
import LegalView from '@/views/LegalView.vue'
import ImpressumView from '@/views/ImpressumView.vue'
import PrivacyView from '@/views/PrivacyView.vue'

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: '/',
      name: 'home',
      component: HomeView
    },
    {
      path: '/legal',
      name: 'legal',
      component: LegalView
    },
    {
      path: '/legal/impressum',
      name: 'impressum',
      component: ImpressumView
    },
    {
      path: '/legal/privacy',
      name: 'privacy',
      component: PrivacyView
    },
    {
      path: '/impressum',
      redirect: '/legal/impressum'
    },
    {
      path: '/privacy',
      redirect: '/legal/privacy'
    }
  ],
  scrollBehavior() {
    return { top: 0 }
  }
})

export default router
