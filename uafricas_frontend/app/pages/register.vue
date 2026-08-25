<script setup lang="ts">
/**
 * Inscription, reconstruite sur les éléments de la refonte.
 *
 * La logique est celle d'avant, inchangée : même `useAuth().register`, mêmes
 * deux validations locales (longueur du mot de passe, correspondance des deux
 * saisies), même redirection vers l'écran d'attente de vérification.
 */
definePageMeta({
  layout: 'auth',
})

useHead({
  title: 'Inscription - AfricanS',
})

const router = useRouter()
const { register, loading, error } = useAuth()

const form = reactive({
  nom: '',
  prenom: '',
  email: '',
  mot_de_passe: '',
  confirmation_mot_de_passe: '',
})

/** Longueur minimale exigée par le serveur, répétée ici pour que le refus
 *  arrive avant l'aller-retour réseau, pas à la place de sa vérification. */
const LONGUEUR_MINIMALE = 6

const validationError = ref<string | null>(null)

const displayError = computed(() => validationError.value || error.value)

/**
 * Signalé pendant la frappe, mais seulement une fois la confirmation
 * commencée : le dire dès le premier caractère saisi ferait clignoter une
 * erreur sur une saisie encore en cours.
 */
const motsDePasseDifferents = computed(() =>
  form.confirmation_mot_de_passe.length > 0
  && form.mot_de_passe !== form.confirmation_mot_de_passe)

const handleRegister = async () => {
  validationError.value = null

  if (form.mot_de_passe.length < LONGUEUR_MINIMALE) {
    validationError.value = `Le mot de passe doit contenir au moins ${LONGUEUR_MINIMALE} caractères`
    return
  }

  if (form.mot_de_passe !== form.confirmation_mot_de_passe) {
    validationError.value = 'Les mots de passe ne correspondent pas'
    return
  }

  try {
    const email = await register(form)
    router.push({ path: '/verification-email-envoyee', query: { email } })
  }
  catch (err) {
    console.error('Erreur inscription:', err)
  }
}
</script>

<template>
  <AfricansCadreAuth titre="Créer un compte" sous-titre="Rejoignez la communauté AfricanS">
    <form class="flex flex-col gap-5" @submit.prevent="handleRegister">
      <div class="grid gap-5 sm:grid-cols-2">
        <AfricansChamp
          v-model="form.prenom"
          libelle="Prénom"
          icone="fa-solid fa-user"
          placeholder="Votre prénom"
          autocomplete="given-name"
          obligatoire
          :desactive="loading"
        />
        <AfricansChamp
          v-model="form.nom"
          libelle="Nom"
          icone="fa-solid fa-user"
          placeholder="Votre nom"
          autocomplete="family-name"
          obligatoire
          :desactive="loading"
        />
      </div>

      <AfricansChamp
        v-model="form.email"
        libelle="Email"
        type="email"
        icone="fa-solid fa-envelope"
        placeholder="votre@email.com"
        autocomplete="email"
        aide="Un lien de vérification y sera envoyé : le compte reste inactif tant qu'il n'est pas ouvert."
        obligatoire
        :desactive="loading"
      />

      <AfricansChamp
        v-model="form.mot_de_passe"
        libelle="Mot de passe"
        type="password"
        icone="fa-solid fa-lock"
        :placeholder="`Minimum ${LONGUEUR_MINIMALE} caractères`"
        autocomplete="new-password"
        obligatoire
        :desactive="loading"
      />

      <AfricansChamp
        v-model="form.confirmation_mot_de_passe"
        libelle="Confirmer le mot de passe"
        type="password"
        icone="fa-solid fa-lock"
        placeholder="Retapez votre mot de passe"
        autocomplete="new-password"
        obligatoire
        :desactive="loading"
      />

      <!-- Dit pendant la frappe plutôt qu'à la soumission : découvrir au bout
           du formulaire que les deux saisies divergent oblige à les refaire
           toutes les deux, un gestionnaire de mots de passe ne les remplissant
           pas deux fois. -->
      <p
        v-if="motsDePasseDifferents"
        class="-mt-2 flex items-center gap-2 text-[12px]/[1.4] text-af-live"
      >
        <font-awesome-icon icon="fa-solid fa-circle-exclamation" />
        Les deux mots de passe ne correspondent pas.
      </p>

      <div
        v-if="displayError"
        role="alert"
        class="flex items-start gap-3 rounded-[10px] border border-af-live/30 bg-af-live/[0.05] px-4 py-3 text-[14px]/[1.4] text-af-corps"
      >
        <font-awesome-icon icon="fa-solid fa-circle-exclamation" class="mt-1 shrink-0 text-af-live" />
        {{ displayError }}
      </div>

      <AfricansBouton
        type="submit"
        pleine-largeur
        :desactive="loading"
        :tourne="loading"
        :icone="loading ? 'fa-solid fa-spinner' : undefined"
      >
        {{ loading ? 'Création en cours…' : 'Créer mon compte' }}
      </AfricansBouton>
    </form>

    <p class="text-center text-[14px]/[1.4] text-af-corps">
      Déjà un compte ?
      <NuxtLink to="/login" class="font-bold text-af-chocolat hover:opacity-70">
        Se connecter
      </NuxtLink>
    </p>
  </AfricansCadreAuth>
</template>
