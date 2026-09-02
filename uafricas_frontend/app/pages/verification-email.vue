<template>
  <div
    class="min-h-screen w-screen bg-font-login bg-cover bg-center relative flex items-center justify-center"
  >
    <div class="absolute inset-0 bg-black/30"></div>

    <div
      class="relative z-10 w-full max-w-md mx-4"
      data-aos="fade-up"
      data-aos-duration="1200"
    >
      <div
        class="backdrop-blur-lg bg-white/95 rounded-2xl shadow-2xl border border-white/20 overflow-hidden"
      >
        <!-- Header -->
        <div
          class="bg-gradient-to-br from-gray-800 via-gray-900 to-black p-6 text-center relative overflow-hidden"
        >
          <div
            class="absolute inset-0 bg-gradient-to-r from-custom-green/10 to-custom-chocolat/10"
          ></div>
          <div class="relative z-10">
            <NuxtLink
              to="/"
              class="inline-block transform hover:scale-105 transition-transform duration-300"
            >
              <img
                class="h-14 mx-auto filter drop-shadow-2xl"
                src="/logos/logo_uafracas.png"
                alt="AfricanS"
              />
            </NuxtLink>
            <h1 class="text-white text-xl font-bold mt-3 tracking-wide">
              Vérification de votre email
            </h1>
          </div>
        </div>

        <!-- Contenu -->
        <div class="p-8 text-center">
          <!-- Etat : Chargement -->
          <div v-if="etat === 'chargement'">
            <div class="mx-auto w-20 h-20 bg-gradient-to-br from-custom-green/10 to-custom-chocolat/10 rounded-full flex items-center justify-center mb-6">
              <svg class="animate-spin w-10 h-10 text-custom-green" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
              </svg>
            </div>
            <p class="text-gray-700 text-base mb-2">Vérification en cours...</p>
            <p class="text-gray-500 text-sm">Veuillez patienter quelques instants.</p>
          </div>

          <!-- Etat : Succes -->
          <div v-else-if="etat === 'succes'">
            <div class="mx-auto w-20 h-20 bg-green-100 rounded-full flex items-center justify-center mb-6">
              <svg class="w-10 h-10 text-green-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
              </svg>
            </div>
            <h2 class="text-xl font-bold text-gray-800 mb-2">Email vérifié !</h2>
            <p class="text-gray-600 text-sm mb-6">
              Votre compte a été activé avec succès. Vous allez être redirigé vers la page d'accueil...
            </p>
            <div class="bg-green-50 border border-green-200 text-green-700 px-4 py-3 rounded-lg text-sm">
              <div class="flex items-center justify-center">
                <svg class="h-4 w-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
                </svg>
                Redirection dans {{ compteurRedirection }}s...
              </div>
            </div>
          </div>

          <!-- Etat : Erreur -->
          <div v-else-if="etat === 'erreur'">
            <div class="mx-auto w-20 h-20 bg-red-100 rounded-full flex items-center justify-center mb-6">
              <svg class="w-10 h-10 text-red-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
              </svg>
            </div>
            <h2 class="text-xl font-bold text-gray-800 mb-2">Échec de la vérification</h2>
            <p class="text-red-600 text-sm mb-6">{{ messageErreur }}</p>

            <!-- Renvoyer un nouveau lien -->
            <div class="space-y-4">
              <div class="space-y-2">
                <label class="text-sm font-medium text-gray-700 block">Votre adresse email</label>
                <input
                  v-model="emailRenvoi"
                  type="email"
                  class="w-full px-4 py-3 border border-gray-300 rounded-xl focus:ring-2 focus:ring-custom-green focus:border-transparent transition-all duration-300 bg-gray-50 hover:bg-white"
                  placeholder="votre@email.com"
                />
              </div>

              <!-- Message de succes apres renvoi -->
              <div
                v-if="renvoyeOk"
                class="bg-green-50 border border-green-200 text-green-700 px-4 py-3 rounded-lg text-sm"
              >
                <div class="flex items-center justify-center">
                  <svg class="h-4 w-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
                  </svg>
                  Nouvel email envoyé ! Verifiez votre boite de reception.
                </div>
              </div>

              <!-- Message d'erreur renvoi -->
              <div
                v-if="erreurRenvoi"
                class="bg-red-50 border border-red-200 text-red-600 px-4 py-3 rounded-lg text-sm"
              >
                {{ erreurRenvoi }}
              </div>

              <button
                type="button"
                class="w-full bg-gradient-to-r from-custom-chocolat to-custom-green text-white font-semibold py-3 px-6 rounded-xl hover:shadow-lg transform hover:-translate-y-0.5 transition-all duration-300 disabled:opacity-50 disabled:cursor-not-allowed disabled:transform-none focus:outline-hidden focus:ring-2 focus:ring-custom-green focus:ring-offset-2"
                :disabled="loadingRenvoi || !emailRenvoi || compteurRenvoi > 0"
                @click="handleRenvoyer"
              >
                <div class="flex items-center justify-center">
                  <svg
                    v-if="loadingRenvoi"
                    class="animate-spin -ml-1 mr-3 h-5 w-5 text-white"
                    fill="none"
                    stroke="currentColor"
                    viewBox="0 0 24 24"
                  >
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
                  </svg>
                  <span v-if="loadingRenvoi">Envoi en cours...</span>
                  <span v-else-if="compteurRenvoi > 0">Renvoyer dans {{ compteurRenvoi }}s</span>
                  <span v-else>Renvoyer un nouveau lien</span>
                </div>
              </button>
            </div>
          </div>

          <!-- Lien retour connexion -->
          <div class="mt-6 text-sm text-gray-600">
            <NuxtLink
              to="/login"
              class="text-custom-green hover:text-custom-chocolat font-semibold transition-colors duration-300 hover:underline"
            >
              Retour a la connexion
            </NuxtLink>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
definePageMeta({
  layout: 'auth',
})

useHead({
  title: 'Vérification de l’email | AfricanS',
})

useAOS()

const route = useRoute()
const router = useRouter()
const { verifierEmail, renvoyerVerification, loading } = useAuth()

const etat = ref<'chargement' | 'succes' | 'erreur'>('chargement')
const messageErreur = ref('')
const compteurRedirection = ref(3)

// Renvoi
const emailRenvoi = ref('')
const renvoyeOk = ref(false)
const erreurRenvoi = ref<string | null>(null)
const loadingRenvoi = ref(false)
const compteurRenvoi = ref(0)

// Verifier le token au chargement
onMounted(async () => {
  const token = route.query.token as string

  if (!token) {
    etat.value = 'erreur'
    messageErreur.value = 'Aucun token de verification fourni. Verifiez le lien dans votre email.'
    return
  }

  try {
    await verifierEmail(token)
    etat.value = 'succes'

    // Cible de redirection : ?redirect=<page interne> si present et valide, sinon accueil
    const redirect = route.query.redirect
    const cible
      = typeof redirect === 'string' && redirect.startsWith('/') && !redirect.startsWith('//')
        ? redirect
        : '/'

    // Redirection automatique apres 3 secondes
    const interval = setInterval(() => {
      compteurRedirection.value--
      if (compteurRedirection.value <= 0) {
        clearInterval(interval)
        router.push(cible)
      }
    }, 1000)
  }
  catch (err: any) {
    etat.value = 'erreur'
    messageErreur.value = err?.data?.error || err?.message || 'Token invalide ou expire.'
  }
})

const handleRenvoyer = async () => {
  if (!emailRenvoi.value || compteurRenvoi.value > 0) return
  renvoyeOk.value = false
  erreurRenvoi.value = null
  loadingRenvoi.value = true

  try {
    await renvoyerVerification(emailRenvoi.value)
    renvoyeOk.value = true
    compteurRenvoi.value = 60
    const interval = setInterval(() => {
      compteurRenvoi.value--
      if (compteurRenvoi.value <= 0) clearInterval(interval)
    }, 1000)
  }
  catch (err: any) {
    erreurRenvoi.value = err?.data?.error || err?.message || 'Erreur lors du renvoi'
  }
  finally {
    loadingRenvoi.value = false
  }
}
</script>

<style scoped>
.bg-font-login {
  background-image: url('/images/font_login.jpg');
  background-attachment: fixed;
}

.backdrop-blur-lg {
  backdrop-filter: blur(20px);
  -webkit-backdrop-filter: blur(20px);
}

@media (max-width: 768px) {
  .bg-font-login {
    background-attachment: scroll;
  }
}
</style>
