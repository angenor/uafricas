<template>
  <div class="min-h-screen bg-gray-900">
    <!-- Chargement -->
    <div v-if="chargement" class="flex min-h-screen items-center justify-center">
      <div class="text-center text-white">
        <font-awesome-icon icon="fa-solid fa-spinner" class="mb-4 inline-block animate-spin text-5xl text-custom-green" />
        <p class="text-gray-300">Connexion au direct…</p>
      </div>
    </div>

    <!-- Erreur d'accès (403 non inscrit, 409 capacité/non ouvert, 422 hors fenêtre) -->
    <div v-else-if="erreur" class="flex min-h-screen items-center justify-center p-6">
      <div class="max-w-md rounded-2xl bg-gray-800 p-8 text-center text-white">
        <font-awesome-icon icon="fa-solid fa-circle-exclamation" class="mb-4 text-4xl text-amber-400" />
        <h3 class="mb-2 text-lg font-semibold">Accès au direct impossible</h3>
        <p class="mb-6 text-sm text-gray-400">{{ erreur }}</p>
        <div class="flex flex-col items-center gap-3">
          <a
            v-if="lienEnLigne"
            :href="lienEnLigne"
            target="_blank"
            rel="noopener"
            class="inline-flex items-center gap-2 rounded-lg bg-custom-green px-5 py-2.5 font-semibold text-white transition hover:brightness-110"
          >
            <font-awesome-icon icon="fa-solid fa-arrow-up-right-from-square" />
            Rejoindre via le lien externe
          </a>
          <NuxtLink
            :to="`/evenements/${evenementId}`"
            class="inline-flex items-center gap-2 text-sm text-gray-300 hover:text-white"
          >
            <font-awesome-icon icon="fa-solid fa-arrow-left" />
            Retour à l'événement
          </NuxtLink>
        </div>
      </div>
    </div>

    <!-- Salle de direct -->
    <EvenementDirectRoom
      v-else-if="config"
      :token="config.token"
      :room-name="config.room_name"
      :livekit-url="config.livekit_url"
      :role="config.role"
      :evenement-id="evenementId"
      :mon-identite="monIdentite"
      :nom="nom"
      :organisateur-id="organisateurId"
      :lien-en-ligne="lienEnLigne"
      :demandes-initiales="demandesInitiales"
      :titre="titre"
      @quitter="quitter"
    />
  </div>
</template>

<script setup lang="ts">
import { useEvenements, type TokenDirect, type DemandeParole } from '~/composables/useEvenements'
import { useUserStore } from '~/stores/user'

definePageMeta({ layout: false })

const route = useRoute()
const router = useRouter()
const evenementId = route.params.id as string
const userStore = useUserStore()

const { rejoindreDirect, quitterDirect, obtenirEvenement, obtenirEtatDirect } = useEvenements()

const chargement = ref(true)
const erreur = ref<string | null>(null)
const config = ref<TokenDirect | null>(null)
const demandesInitiales = ref<DemandeParole[]>([])
const organisateurId = ref('')
const lienEnLigne = ref<string | null>(null)
const titre = ref<string | null>(null)

const monIdentite = computed(() => userStore.user?.id ?? '')
const nom = computed(() => userStore.fullName || 'Invité')

onMounted(async () => {
  if (!userStore.accessToken) {
    router.replace(`/login?redirect=${encodeURIComponent(`/evenements/${evenementId}/direct`)}`)
    return
  }

  // Métadonnées de l'événement (titre, lien de repli, organisateur).
  const evt = await obtenirEvenement(evenementId)
  if (evt) {
    organisateurId.value = evt.user.uid
    lienEnLigne.value = evt.lien_en_ligne
    titre.value = evt.titre
  }

  try {
    config.value = await rejoindreDirect(evenementId)
  }
  catch (e) {
    erreur.value = (e as Error).message
    chargement.value = false
    return
  }

  // Demandes de parole initiales (vue organisateur).
  if (config.value.role === 'organisateur') {
    const etat = await obtenirEtatDirect(evenementId)
    demandesInitiales.value = etat?.demandes_parole ?? []
  }

  chargement.value = false
})

const quitter = async (): Promise<void> => {
  await quitterDirect(evenementId)
  router.push(`/evenements/${evenementId}`)
}

useHead({ title: computed(() => (titre.value ? `Direct · ${titre.value}` : 'Direct | UAfricas')) })
</script>
