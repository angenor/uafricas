<script setup lang="ts">
/**
 * Connexion, reconstruite sur les éléments de la refonte.
 *
 * La logique est celle d'avant, inchangée : même `useAuth().login`, même cible
 * de redirection bornée aux chemins internes, même détection de l'erreur de
 * vérification d'e-mail. Seule la présentation bascule.
 *
 * Un lien a disparu : « Mot de passe oublié ? ». Aucune réinitialisation
 * n'existe : ni route serveur, ni page, ni composable, et il pointait sur
 * `/`, qui sert désormais le fil d'actualité. Il déposait donc l'utilisateur
 * sur le mur en lui laissant croire qu'il allait récupérer son compte.
 */
definePageMeta({
  layout: 'auth',
})

useHead({
  title: 'Connexion - AfricanS',
})

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
  }
  catch (err) {
    console.error('Erreur de connexion:', err)
  }
}

</script>

<template>
  <AfricansCadreAuth titre="Bienvenue" sous-titre="Connectez-vous à votre compte">
    <form class="flex flex-col gap-5" @submit.prevent="handleLogin">
      <AfricansChamp
        v-model="loginForm.email"
        libelle="Email"
        type="email"
        icone="fa-solid fa-envelope"
        placeholder="votre@email.com"
        autocomplete="email"
        obligatoire
        :desactive="loading"
      />

      <AfricansChamp
        v-model="loginForm.password"
        libelle="Mot de passe"
        type="password"
        icone="fa-solid fa-lock"
        placeholder="Votre mot de passe"
        autocomplete="current-password"
        obligatoire
        :desactive="loading"
      />

      <!-- Le message d'erreur est rendu avec `role="alert"` : sans lui, un
           lecteur d'écran ne signale rien à qui vient de soumettre. -->
      <div
        v-if="error"
        role="alert"
        class="flex flex-col gap-2 rounded-[10px] border border-af-live/30 bg-af-live/[0.05] px-4 py-3"
      >
        <p class="flex items-start gap-3 text-[14px]/[1.4] text-af-corps">
          <font-awesome-icon icon="fa-solid fa-circle-exclamation" class="mt-1 shrink-0 text-af-live" />
          {{ error }}
        </p>
        <!-- Sortie de secours : un compte non vérifié ne se débloque que par
             un nouvel envoi, et c'est le seul endroit où le dire. -->
        <NuxtLink
          v-if="estErreurVerification"
          :to="{ path: '/verification-email-envoyee', query: { email: loginForm.email } }"
          class="text-[14px]/[1.4] font-bold text-af-chocolat underline underline-offset-4 hover:opacity-70"
        >
          Renvoyer l'e-mail de vérification
        </NuxtLink>
      </div>

      <AfricansBouton
        type="submit"
        pleine-largeur
        :desactive="loading"
        :tourne="loading"
        :icone="loading ? 'fa-solid fa-spinner' : undefined"
      >
        {{ loading ? 'Connexion en cours…' : 'Se connecter' }}
      </AfricansBouton>
    </form>

    <div class="flex items-center gap-4">
      <span class="h-px flex-1 bg-af-bordure" />
      <span class="text-[12px]/[1.4] text-af-atone">Ou continuer avec</span>
      <span class="h-px flex-1 bg-af-bordure" />
    </div>

    <!-- Conservé tel quel : le bouton est désactivé ET dit qu'il l'est. Il
         annonce une intention sans rien promettre de faux. -->
    <button
      type="button"
      disabled
      class="flex h-10 w-full cursor-not-allowed items-center justify-center gap-3 rounded-lg border border-af-bordure bg-white text-base font-bold text-af-atone-2"
    >
      <font-awesome-icon icon="fa-brands fa-google" />
      Google : bientôt disponible
    </button>

    <p class="text-center text-[14px]/[1.4] text-af-corps">
      Pas encore de compte ?
      <NuxtLink to="/register" class="font-bold text-af-chocolat hover:opacity-70">
        Créer un compte
      </NuxtLink>
    </p>
  </AfricansCadreAuth>
</template>
