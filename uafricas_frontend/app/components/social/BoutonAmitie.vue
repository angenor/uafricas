<script setup lang="ts">
import type { EtatRelation } from '~/composables/useAmis'
import { useUserStore } from '~/stores/user'

const props = withDefaults(defineProps<{
  /** Identifiant du membre ciblé */
  utilisateurId: string
  /** État courant de la relation (chargé par le parent, éventuellement en lot) */
  etat?: EtatRelation
  /** Identifiant de la demande en attente, le cas échéant */
  demandeId?: string | null
  /** Taille du bouton */
  taille?: 'sm' | 'md'
}>(), {
  etat: 'aucune',
  demandeId: null,
  taille: 'md',
})

const emit = defineEmits<{
  /** Émis après un changement d'état (le parent peut rafraîchir) */
  (e: 'update', etat: EtatRelation): void
}>()

const userStore = useUserStore()
const route = useRoute()
const { envoyerDemande, debloquer, codeErreur } = useAmis()

// État local synchronisé avec la prop (mise à jour optimiste après action)
const etatLocal = ref<EtatRelation>(props.etat)
watch(() => props.etat, v => { etatLocal.value = v })

const enCours = ref(false)
const message = ref<string | null>(null)

// Ne pas afficher le bouton sur sa propre carte
const estMoi = computed(() => userStore.user?.id === props.utilisateurId)

const classesTaille = computed(() =>
  props.taille === 'sm'
    ? 'px-3 py-1.5 text-xs gap-1.5'
    : 'px-5 py-2.5 text-sm gap-2',
)

const messageErreur = (code: number | null): string => {
  switch (code) {
    case 409: return 'Vous êtes déjà en relation avec ce membre.'
    case 429: return 'Limite de demandes atteinte, réessayez plus tard.'
    case 403: return 'Action impossible avec ce membre.'
    case 400:
    case 422: return 'Ce membre n\'est pas disponible.'
    default: return 'Une erreur est survenue.'
  }
}

const demander = async () => {
  message.value = null

  // Non connecté → redirection vers la connexion avec retour
  if (!userStore.isAuthenticated) {
    await navigateTo(`/login?redirect=${encodeURIComponent(route.fullPath)}`)
    return
  }

  enCours.value = true
  const res = await envoyerDemande(props.utilisateurId)
  enCours.value = false

  if (res) {
    etatLocal.value = res.statut === 'amis' ? 'amis' : 'demande_envoyee'
    emit('update', etatLocal.value)
  }
  else {
    message.value = messageErreur(codeErreur.value)
  }
}

const debloquerMembre = async () => {
  message.value = null
  enCours.value = true
  const ok = await debloquer(props.utilisateurId)
  enCours.value = false

  if (ok) {
    etatLocal.value = 'aucune'
    emit('update', etatLocal.value)
  }
  else {
    message.value = messageErreur(codeErreur.value)
  }
}
</script>

<template>
  <div v-if="!estMoi && etatLocal !== 'indisponible'" class="inline-flex flex-col items-start">
    <!-- Aucune relation : proposer l'amitié -->
    <button
      v-if="etatLocal === 'aucune'"
      type="button"
      :disabled="enCours"
      class="inline-flex items-center justify-center rounded-xl font-semibold text-white bg-linear-to-r from-custom-chocolat to-custom-green hover:shadow-lg transition disabled:opacity-60"
      :class="classesTaille"
      @click="demander"
    >
      <font-awesome-icon :icon="enCours ? 'fa-solid fa-spinner' : 'fa-solid fa-user-plus'" :class="{ 'animate-spin': enCours }" />
      Demander en ami
    </button>

    <!-- Demande envoyée : en attente -->
    <span
      v-else-if="etatLocal === 'demande_envoyee'"
      class="inline-flex items-center justify-center rounded-xl font-semibold text-gray-500 bg-gray-100 cursor-default"
      :class="classesTaille"
    >
      <font-awesome-icon icon="fa-solid fa-user-clock" />
      Demande envoyée
    </span>

    <!-- Demande reçue : inviter à répondre -->
    <NuxtLink
      v-else-if="etatLocal === 'demande_recue'"
      to="/mon-compte/amis"
      class="inline-flex items-center justify-center rounded-xl font-semibold text-white bg-custom-chocolat hover:shadow-lg transition"
      :class="classesTaille"
    >
      <font-awesome-icon icon="fa-solid fa-user-check" />
      Répondre
    </NuxtLink>

    <!-- Déjà amis -->
    <span
      v-else-if="etatLocal === 'amis'"
      class="inline-flex items-center justify-center rounded-xl font-semibold text-custom-green bg-custom-green/10 cursor-default"
      :class="classesTaille"
    >
      <font-awesome-icon icon="fa-solid fa-user-check" />
      Amis
    </span>

    <!-- Membre bloqué par l'utilisateur courant : permettre le déblocage (US4, FR-013) -->
    <button
      v-else-if="etatLocal === 'bloque_par_moi'"
      type="button"
      :disabled="enCours"
      class="inline-flex items-center justify-center rounded-xl font-semibold text-red-600 bg-red-50 hover:bg-red-100 transition disabled:opacity-60"
      :class="classesTaille"
      @click="debloquerMembre"
    >
      <font-awesome-icon :icon="enCours ? 'fa-solid fa-spinner' : 'fa-solid fa-ban'" :class="{ 'animate-spin': enCours }" />
      Débloquer
    </button>

    <p v-if="message" class="text-xs text-red-600 mt-1.5">{{ message }}</p>
  </div>
</template>
