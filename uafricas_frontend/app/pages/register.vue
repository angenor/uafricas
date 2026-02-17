<template>
  <div
    class="min-h-screen w-screen bg-font-login bg-cover bg-center relative flex items-center justify-center lg:justify-end"
  >
    <!-- Overlay sombre pour ameliorer la lisibilite -->
    <div class="absolute inset-0 bg-black/30"></div>

    <!-- Container principal -->
    <div
      class="relative z-10 w-full max-w-md mx-4 lg:mr-24 lg:mx-0 mt-10"
      data-aos="fade-left"
      data-aos-duration="1200"
    >
      <!-- Carte d'inscription avec effet glassmorphisme -->
      <div
        class="backdrop-blur-lg bg-white/95 rounded-2xl shadow-2xl border border-white/20 overflow-hidden"
      >
        <!-- Header avec logo -->
        <div
          class="bg-gradient-to-br from-gray-800 via-gray-900 to-black p-6 text-center relative overflow-hidden"
        >
          <!-- Motif decoratif en arriere-plan -->
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
              Creer un compte
            </h1>
            <p class="text-gray-300 text-sm mt-1">
              Rejoignez la communaute UAfricas
            </p>
          </div>
        </div>

        <!-- Formulaire -->
        <div class="p-8">
          <form @submit.prevent="handleRegister" class="space-y-5">
            <!-- Prenom et Nom sur la meme ligne -->
            <div class="grid grid-cols-2 gap-4">
              <!-- Champ Prenom -->
              <div class="space-y-2">
                <label class="text-sm font-medium text-gray-700 block">Prenom</label>
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
                        d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"
                      />
                    </svg>
                  </div>
                  <input
                    required
                    v-model="form.prenom"
                    type="text"
                    class="w-full pl-10 pr-4 py-3 border border-gray-300 rounded-xl focus:ring-2 focus:ring-custom-green focus:border-transparent transition-all duration-300 bg-gray-50 hover:bg-white disabled:opacity-50 disabled:cursor-not-allowed"
                    placeholder="Votre prenom"
                    :disabled="loading"
                  />
                </div>
              </div>

              <!-- Champ Nom -->
              <div class="space-y-2">
                <label class="text-sm font-medium text-gray-700 block">Nom</label>
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
                        d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"
                      />
                    </svg>
                  </div>
                  <input
                    required
                    v-model="form.nom"
                    type="text"
                    class="w-full pl-10 pr-4 py-3 border border-gray-300 rounded-xl focus:ring-2 focus:ring-custom-green focus:border-transparent transition-all duration-300 bg-gray-50 hover:bg-white disabled:opacity-50 disabled:cursor-not-allowed"
                    placeholder="Votre nom"
                    :disabled="loading"
                  />
                </div>
              </div>
            </div>

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
                  v-model="form.email"
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
                  v-model="form.mot_de_passe"
                  type="password"
                  minlength="6"
                  class="w-full pl-10 pr-4 py-3 border border-gray-300 rounded-xl focus:ring-2 focus:ring-custom-green focus:border-transparent transition-all duration-300 bg-gray-50 hover:bg-white disabled:opacity-50 disabled:cursor-not-allowed"
                  placeholder="Minimum 6 caracteres"
                  :disabled="loading"
                />
              </div>
            </div>

            <!-- Champ Confirmation mot de passe -->
            <div class="space-y-2">
              <label class="text-sm font-medium text-gray-700 block">Confirmer le mot de passe</label>
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
                  v-model="form.confirmation_mot_de_passe"
                  type="password"
                  minlength="6"
                  class="w-full pl-10 pr-4 py-3 border border-gray-300 rounded-xl focus:ring-2 focus:ring-custom-green focus:border-transparent transition-all duration-300 bg-gray-50 hover:bg-white disabled:opacity-50 disabled:cursor-not-allowed"
                  placeholder="Retapez votre mot de passe"
                  :disabled="loading"
                />
              </div>
            </div>

            <!-- Message d'erreur -->
            <div
              v-if="displayError"
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
                {{ displayError }}
              </div>
            </div>

            <!-- Bouton d'inscription -->
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
                <span v-if="loading">Inscription en cours...</span>
                <span v-else>Creer mon compte</span>
              </div>
            </button>
          </form>

          <!-- Lien vers connexion -->
          <div class="mt-6 text-center">
            <div class="text-sm text-gray-600">
              Deja un compte ?
              <NuxtLink
                to="/login"
                class="text-custom-green hover:text-custom-chocolat font-semibold transition-colors duration-300 hover:underline ml-1"
              >
                Se connecter
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
  title: 'Inscription - UAfricas',
})

// Initialiser AOS
useAOS()

const router = useRouter()
const { register, loading, error } = useAuth()

const form = reactive({
  nom: '',
  prenom: '',
  email: '',
  mot_de_passe: '',
  confirmation_mot_de_passe: '',
})

const validationError = ref<string | null>(null)

const displayError = computed(() => validationError.value || error.value)

const handleRegister = async () => {
  validationError.value = null

  if (form.mot_de_passe.length < 6) {
    validationError.value = 'Le mot de passe doit contenir au moins 6 caracteres'
    return
  }

  if (form.mot_de_passe !== form.confirmation_mot_de_passe) {
    validationError.value = 'Les mots de passe ne correspondent pas'
    return
  }

  try {
    const email = await register(form)
    router.push({ path: '/verification-email-envoyee', query: { email } })
  } catch (err) {
    console.error('Erreur inscription:', err)
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

/* Effet glassmorphisme renforce */
.backdrop-blur-lg {
  backdrop-filter: blur(20px);
  -webkit-backdrop-filter: blur(20px);
}

/* Responsive design ameliore */
@media (max-width: 768px) {
  .bg-font-login {
    background-attachment: scroll;
  }
}
</style>
