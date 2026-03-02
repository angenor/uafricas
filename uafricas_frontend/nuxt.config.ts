// https://nuxt.com/docs/api/configuration/nuxt-config
import tailwindcss from '@tailwindcss/vite'

export default defineNuxtConfig({
  compatibilityDate: '2025-07-15',
  devtools: { enabled: true },
  modules: ['@pinia/nuxt'],
  css: [
    '~/assets/css/main.css',
    'aos/dist/aos.css',
    '@fortawesome/fontawesome-svg-core/styles.css',
  ],
  vite: {
    plugins: [
      tailwindcss(),
    ],
  },

  // Configuration d'execution pour l'URL de l'API backend
  runtimeConfig: {
    public: {
      apiBaseUrl: process.env.NUXT_PUBLIC_API_BASE_URL || 'http://localhost:8080',
    },
  },

  nitro: {
    prerender: {
      failOnError: false,
      crawlLinks: false,
    },
  },

  app: {
    head: {
      title: 'UAfricas - United Africa for Sustainable Development',
      meta: [
        { charset: 'utf-8' },
        { name: 'viewport', content: 'width=device-width, initial-scale=1' },
        { name: 'description', content: 'Plateforme panafricaine pour le développement durable' },
      ],
      link: [
        { rel: 'preconnect', href: 'https://fonts.googleapis.com' },
        { rel: 'preconnect', href: 'https://fonts.gstatic.com', crossorigin: '' },
        { rel: 'stylesheet', href: 'https://fonts.googleapis.com/css2?family=Open+Sans:wght@400;500;600;700&family=Oswald:wght@400;500;600;700&display=swap' },
      ],
    },
  },
})
