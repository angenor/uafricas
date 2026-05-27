<template>
  <div
    class="min-h-screen w-screen bg-font-login bg-cover bg-center relative flex items-center justify-center lg:justify-end"
  >
    <!-- Overlay sombre pour améliorer la lisibilité -->
    <div class="absolute inset-0 bg-black/30"></div>

    <!-- Container principal -->
    <div
      class="relative z-10 w-full max-w-md mx-4 lg:mr-24 lg:mx-0 mt-20"
      data-aos="fade-left"
      data-aos-duration="1200"
    >
      <!-- Carte de connexion avec effet glassmorphisme -->
      <div
        class="backdrop-blur-lg bg-white/95 rounded-2xl shadow-2xl border border-white/20 overflow-hidden"
      >
        <!-- Header avec logo -->
        <div
          class="bg-gradient-to-br from-gray-800 via-gray-900 to-black p-6 text-center relative overflow-hidden"
        >
          <!-- Motif décoratif en arrière-plan -->
          <div
            class="absolute inset-0 bg-gradient-to-r from-custom-green/10 to-custom-chocolat/10"
          ></div>
          <div
            class="absolute top-0 right-0 w-32 h-32 bg-custom-green/5 rounded-full -translate-y-16 translate-x-16"
          ></div>
          <div
            class="absolute bottom-0 left-0 w-24 h-24 bg-custom-chocolat/5 rounded-full translate-y-12 -translate-x-12"
          ></div>

          <div class="relative z-10">
            <NuxtLink
              to="/"
              class="inline-block transform hover:scale-105 transition-transform duration-300"
            >
              <img
                class="h-14 mx-auto filter drop-shadow-2xl"
                src="/logos/logo_uafracas.png"
                alt="UAfricas"
              />
            </NuxtLink>
            <h1 class="text-white text-xl font-bold mt-3 tracking-wide">
              Bienvenue
            </h1>
            <p class="text-gray-300 text-sm mt-1">
              Connectez-vous à votre compte
            </p>
          </div>
        </div>

        <!-- Formulaire -->
        <div class="p-8">
          <form @submit.prevent="handleLogin" class="space-y-6">
            <!-- Champ Email -->
            <div class="space-y-2">
              <label class="text-sm font-medium text-gray-700 block">Email</label>
              <div class="relative">
                <div
                  class="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none"
                >
                  <svg
                    class="h-5 w-5 text-gray-400"
                    fill="none"
                    stroke="currentColor"
                    viewBox="0 0 24 24"
                  >
                    <path
                      stroke-linecap="round"
                      stroke-linejoin="round"
                      stroke-width="2"
                      d="M16 12a4 4 0 10-8 0 4 4 0 008 0zm0 0v1.5a2.5 2.5 0 005 0V12a9 9 0 10-9 9m4.5-1.206a8.959 8.959 0 01-4.5 1.207"
                    />
                  </svg>
                </div>
                <input
                  required
                  v-model="loginForm.email"
                  type="email"
                  class="w-full pl-10 pr-4 py-3 border border-gray-300 rounded-xl focus:ring-2 focus:ring-custom-green focus:border-transparent transition-all duration-300 bg-gray-50 hover:bg-white disabled:opacity-50 disabled:cursor-not-allowed"
                  placeholder="votre@email.com"
                  :disabled="loading"
                />
              </div>
            </div>

            <!-- Champ Mot de passe -->
            <div class="space-y-2">
              <label class="text-sm font-medium text-gray-700 block">Mot de passe</label>
              <div class="relative">
                <div
                  class="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none"
                >
                  <svg
                    class="h-5 w-5 text-gray-400"
                    fill="none"
                    stroke="currentColor"
                    viewBox="0 0 24 24"
                  >
                    <path
                      stroke-linecap="round"
                      stroke-linejoin="round"
                      stroke-width="2"
                      d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z"
                    />
                  </svg>
                </div>
                <input
                  required
                  v-model="loginForm.password"
                  type="password"
                  class="w-full pl-10 pr-4 py-3 border border-gray-300 rounded-xl focus:ring-2 focus:ring-custom-green focus:border-transparent transition-all duration-300 bg-gray-50 hover:bg-white disabled:opacity-50 disabled:cursor-not-allowed"
                  placeholder="••••••••"
                  :disabled="loading"
                />
              </div>
            </div>

            <!-- Message d'erreur -->
            <div
              v-if="error"
              class="bg-red-50 border border-red-200 text-red-600 px-4 py-3 rounded-lg text-sm"
            >
              <div class="flex items-center">
                <svg
                  class="h-4 w-4 mr-2 shrink-0"
                  fill="none"
                  stroke="currentColor"
                  viewBox="0 0 24 24"
                >
                  <path
                    stroke-linecap="round"
                    stroke-linejoin="round"
                    stroke-width="2"
                    d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
                  />
                </svg>
                <span>{{ error }}</span>
              </div>
              <!-- Lien pour renvoyer la verification si compte non verifie -->
              <div v-if="estErreurVerification" class="mt-2 text-center">
                <NuxtLink
                  :to="{ path: '/verification-email-envoyee', query: { email: loginForm.email } }"
                  class="text-custom-green hover:text-custom-chocolat font-semibold transition-colors duration-300 hover:underline text-sm"
                >
                  Renvoyer l'email de verification
                </NuxtLink>
              </div>
            </div>

            <!-- Bouton de connexion -->
            <button
              type="submit"
              class="w-full bg-gradient-to-r from-custom-chocolat to-custom-green text-white font-semibold py-3 px-6 rounded-xl hover:shadow-lg transform hover:-translate-y-0.5 transition-all duration-300 disabled:opacity-50 disabled:cursor-not-allowed disabled:transform-none focus:outline-hidden focus:ring-2 focus:ring-custom-green focus:ring-offset-2"
              :disabled="loading"
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
                <span v-if="loading">Connexion en cours...</span>
                <span v-else>Se connecter</span>
              </div>
            </button>
          </form>

          <!-- Divider -->
          <div class="mt-6 mb-6">
            <div class="relative">
              <div class="absolute inset-0 flex items-center">
                <div class="w-full border-t border-gray-300"></div>
              </div>
              <div class="relative flex justify-center text-sm">
                <span class="px-2 bg-white text-gray-500">Ou continuer avec</span>
              </div>
            </div>
          </div>

          <!-- Bouton Google (bientot disponible) -->
          <button
            type="button"
            class="w-full bg-white border border-gray-300 text-gray-400 font-medium py-3 px-6 rounded-xl cursor-not-allowed opacity-60"
            disabled
          >
            <div class="flex items-center justify-center">
              <img
                class="h-5 w-5 mr-3 grayscale"
                src="https://upload.wikimedia.org/wikipedia/commons/thumb/c/c1/Google_%22G%22_logo.svg/768px-Google_%22G%22_logo.svg.png"
                alt="Google"
              />
              <span>Google — Bientot disponible</span>
            </div>
          </button>

          <!-- Liens -->
          <div class="mt-6 text-center space-y-3">
            <NuxtLink
              to="/"
              class="block text-sm text-custom-green hover:text-custom-chocolat transition-colors duration-300 hover:underline"
            >
              Mot de passe oublié ?
            </NuxtLink>
            <div class="text-sm text-gray-600">
              Pas encore de compte ?
              <NuxtLink
                to="/register"
                class="text-custom-green hover:text-custom-chocolat font-semibold transition-colors duration-300 hover:underline ml-1"
              >
                Créer un compte
              </NuxtLink>
            </div>
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
  title: 'Connexion - UAfricas',
})

// Initialiser AOS
useAOS()

const router = useRouter()
const route = useRoute()
const { login, loading, error } = useAuth()

/** Cible de redirection apres connexion (chemin interne uniquement) */
const cibleRedirection = computed(() => {
  const redirect = route.query.redirect
  if (typeof redirect === 'string' && redirect.startsWith('/') && !redirect.startsWith('//')) {
    return redirect
  }
  return '/'
})

const loginForm = reactive({
  email: '',
  password: '',
})

const estErreurVerification = computed(() => {
  return error.value?.includes('verifier votre adresse email') || false
})

const handleLogin = async () => {
  try {
    await login(loginForm.email, loginForm.password)
    router.push(cibleRedirection.value)
  } catch (err) {
    console.error('Erreur de connexion:', err)
  }
}

</script>

<style scoped>
.bg-font-login {
  background-image: url('/images/font_login.jpg');
  background-attachment: fixed;
}

/* Animation pour les inputs au focus */
input:focus {
  box-shadow: 0 0 0 3px rgba(34, 139, 34, 0.1);
}

/* Animation pour les boutons */
button:not(:disabled):hover {
  transform: translateY(-1px);
}

/* Effet glassmorphisme renforcé */
.backdrop-blur-lg {
  backdrop-filter: blur(20px);
  -webkit-backdrop-filter: blur(20px);
}

/* Responsive design amélioré */
@media (max-width: 768px) {
  .bg-font-login {
    background-attachment: scroll;
  }
}
</style>
