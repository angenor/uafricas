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
    // URL utilisee cote serveur (SSR) — en production Docker, pointe vers le service backend interne
    ssrApiBaseUrl: process.env.NUXT_SSR_API_BASE_URL || '',
    public: {
      apiBaseUrl: process.env.NUXT_PUBLIC_API_BASE_URL || 'http://localhost:8080',
      // Visioconférence P2P (feature 001-rendez-vous-visio) — surchargeable via NUXT_PUBLIC_*.
      // Hôte de signalisation PeerJS : vide = cloud public 0.peerjs.com.
      peerjsHost: process.env.NUXT_PUBLIC_PEERJS_HOST || '',
      peerjsPort: Number(process.env.NUXT_PUBLIC_PEERJS_PORT) || 443,
      peerjsPath: process.env.NUXT_PUBLIC_PEERJS_PATH || '/',
      peerjsSecure: process.env.NUXT_PUBLIC_PEERJS_SECURE !== 'false',
      // Liste ICE (JSON) ; défaut STUN Google. Brancher un TURN ultérieurement si besoin.
      iceServers: process.env.NUXT_PUBLIC_ICE_SERVERS
        ? JSON.parse(process.env.NUXT_PUBLIC_ICE_SERVERS)
        : [{ urls: 'stun:stun.l.google.com:19302' }],
    },
  },

  nitro: {
    prerender: {
      failOnError: false,
      crawlLinks: false,
    },
  },

  // Redirection permanente 301 (feature 001-centres-reorganisation, Décision 1 research.md).
  // Les redirections paramétrées `/site/:id` et `/site/:siteId/programmation/:programmationId`
  // sont traitées par `server/middleware/redirect-legacy-site.ts` car Nitro ne substitue
  // pas les segments `:param` dans le champ `redirect.to` des routeRules.
  routeRules: {
    '/africain-afro-americain': {
      redirect: { to: '/centres', statusCode: 301 },
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
