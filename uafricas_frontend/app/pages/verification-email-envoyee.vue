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
              Verifiez votre email
            </h1>
          </div>
        </div>

        <!-- Contenu -->
        <div class="p-8 text-center">
          <!-- Icone enveloppe -->
          <div class="mx-auto w-20 h-20 bg-gradient-to-br from-custom-green/10 to-custom-chocolat/10 rounded-full flex items-center justify-center mb-6">
            <svg class="w-10 h-10 text-custom-green" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 8l7.89 5.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z" />
            </svg>
          </div>

          <p class="text-gray-700 text-base mb-2">
            Un email de vérification a été envoye a :
          </p>
          <p class="text-custom-green font-semibold text-lg mb-6">
            {{ email || 'votre adresse email' }}
          </p>

          <p class="text-gray-500 text-sm mb-8 leading-relaxed">
            Cliquez sur le lien dans l'email pour activer votre compte.
            Le lien expire dans 24 heures.
            Pensez a verifier vos spams si vous ne trouvez pas l'email.
          </p>

          <!-- Message de succes apres renvoi -->
          <div
            v-if="envoye"
            class="bg-green-50 border border-green-200 text-green-700 px-4 py-3 rounded-lg text-sm mb-4"
          >
            <div class="flex items-center justify-center">
              <svg class="h-4 w-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7" />
              </svg>
              Nouvel email de vérification envoye !
            </div>
          </div>

          <!-- Message d'erreur -->
          <div
            v-if="erreurRenvoi"
            class="bg-red-50 border border-red-200 text-red-600 px-4 py-3 rounded-lg text-sm mb-4"
          >
            {{ erreurRenvoi }}
          </div>

          <!-- Bouton renvoyer -->
          <button
            type="button"
            class="w-full bg-gradient-to-r from-custom-chocolat to-custom-green text-white font-semibold py-3 px-6 rounded-xl hover:shadow-lg transform hover:-translate-y-0.5 transition-all duration-300 disabled:opacity-50 disabled:cursor-not-allowed disabled:transform-none focus:outline-hidden focus:ring-2 focus:ring-custom-green focus:ring-offset-2 mb-4"
            :disabled="loading || compteur > 0"
            @click="handleRenvoyer"
          >
            <div class="flex items-center justify-center">
              <svg
                v-if="loading"
                class="animate-spin -ml-1 mr-3 h-5 w-5 text-white"
                fill="none"
                stroke="currentColor"
                viewBox="0 0 24 24"
              >
                <path
                  stroke-linecap="round"
                  stroke-linejoin="round"
                  stroke-width="2"
                  d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"
                />
              </svg>
              <span v-if="loading">Envoi en cours...</span>
              <span v-else-if="compteur > 0">Renvoyer dans {{ compteur }}s</span>
              <span v-else>Renvoyer l'email de vérification</span>
            </div>
          </button>

          <!-- Lien retour connexion -->
          <div class="text-sm text-gray-600">
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
  title: 'Verifiez votre email - AfricanS',
})

useAOS()

const route = useRoute()
const { renvoyerVerification, loading } = useAuth()

const email = computed(() => (route.query.email as string) || '')
const envoye = ref(false)
const erreurRenvoi = ref<string | null>(null)
const compteur = ref(0)

const handleRenvoyer = async () => {
  if (!email.value || compteur.value > 0) return
  envoye.value = false
  erreurRenvoi.value = null

  try {
    await renvoyerVerification(email.value)
    envoye.value = true
    // Cooldown de 60 secondes
    compteur.value = 60
    const interval = setInterval(() => {
      compteur.value--
      if (compteur.value <= 0) clearInterval(interval)
    }, 1000)
  }
  catch (err: any) {
    erreurRenvoi.value = err?.data?.error || err?.message || 'Erreur lors du renvoi'
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
