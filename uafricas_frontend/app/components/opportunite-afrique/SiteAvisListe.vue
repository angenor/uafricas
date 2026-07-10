<script setup lang="ts">
import { useOpportuniteAfrique } from '~/composables/useOpportuniteAfrique'
import { formatDateShort, type AvisSiteListe } from '~/composables/useOpportuniteAfrique'

interface Props {
  siteId: string
  estAuthentifie: boolean
}

const props = defineProps<Props>()

const { listerAvisSite, soumettreAvisSite, resoudreUrlImage, erreur } = useOpportuniteAfrique()
const { redirigerVersConnexion } = useAuth()

const donnees = ref<AvisSiteListe | null>(null)
const chargement = ref(true)
const page = ref(1)
const parPage = 5

// Formulaire de dépôt
const noteSaisie = ref(0)
const noteSurvol = ref(0)
const commentaireSaisi = ref('')
const envoiEnCours = ref(false)
const messageErreur = ref('')
const messageSucces = ref('')

const nombreTotal = computed(() => donnees.value?.nombre_total ?? 0)
const noteMoyenne = computed(() => donnees.value?.note_moyenne ?? null)
const totalPages = computed(() => Math.max(1, Math.ceil(nombreTotal.value / parPage)))

const charger = async () => {
  chargement.value = true
  donnees.value = await listerAvisSite(props.siteId, page.value, parPage)
  chargement.value = false
}

onMounted(charger)
watch(page, charger)

const changerPage = (delta: number) => {
  const cible = page.value + delta
  if (cible >= 1 && cible <= totalPages.value) page.value = cible
}

const soumettre = async () => {
  messageErreur.value = ''
  messageSucces.value = ''
  if (noteSaisie.value < 1 || noteSaisie.value > 5) {
    messageErreur.value = 'Veuillez attribuer une note de 1 à 5 étoiles.'
    return
  }
  if (commentaireSaisi.value.trim().length < 1) {
    messageErreur.value = 'Le commentaire ne peut pas être vide.'
    return
  }
  envoiEnCours.value = true
  const resultat = await soumettreAvisSite(props.siteId, noteSaisie.value, commentaireSaisi.value.trim())
  envoiEnCours.value = false
  if (resultat) {
    messageSucces.value = 'Merci, votre avis a été enregistré.'
    commentaireSaisi.value = ''
    noteSaisie.value = 0
    page.value = 1
    await charger()
  }
  else {
    messageErreur.value = erreur.value || 'Soumission impossible.'
  }
}

const allerConnexion = () => redirigerVersConnexion()
</script>

<template>
  <div class="mt-4 pt-4 border-t border-gray-100">
    <!-- En-tête : note moyenne + nombre -->
    <div class="flex items-center justify-between mb-3">
      <h5 class="font-oswald text-sm font-semibold text-gray-800">Avis des visiteurs</h5>
      <div v-if="noteMoyenne !== null" class="flex items-center gap-1.5 text-sm">
        <font-awesome-icon :icon="['fas', 'star']" class="w-3.5 h-3.5 text-amber-400" />
        <span class="font-semibold text-gray-900">{{ noteMoyenne.toFixed(1) }}</span>
        <span class="text-gray-500">({{ nombreTotal }})</span>
      </div>
    </div>

    <div v-if="chargement" class="space-y-2">
      <div v-for="n in 2" :key="n" class="h-12 bg-gray-100 rounded animate-pulse" />
    </div>

    <p v-else-if="nombreTotal === 0" class="text-xs text-gray-500 mb-3">
      Aucun avis pour l'instant. Soyez le premier à donner le vôtre.
    </p>

    <ul v-else class="space-y-3 mb-3">
      <li v-for="avis in donnees?.avis" :key="avis.id" class="text-sm">
        <div class="flex items-center gap-2 mb-1">
          <img
            v-if="avis.utilisateur.photo_url"
            :src="resoudreUrlImage(avis.utilisateur.photo_url)"
            :alt="avis.utilisateur.prenom"
            class="w-6 h-6 rounded-full object-cover"
          />
          <span v-else class="w-6 h-6 rounded-full bg-gray-200 flex items-center justify-center">
            <font-awesome-icon :icon="['fas', 'user']" class="w-3 h-3 text-gray-500" />
          </span>
          <span class="font-medium text-gray-800">
            {{ avis.utilisateur.prenom }} {{ avis.utilisateur.nom }}
          </span>
          <span class="flex items-center gap-0.5 ml-auto">
            <font-awesome-icon
              v-for="i in 5"
              :key="i"
              :icon="['fas', 'star']"
              class="w-3 h-3"
              :class="i <= avis.note ? 'text-amber-400' : 'text-gray-300'"
            />
          </span>
        </div>
        <p class="text-gray-600 whitespace-pre-line">{{ avis.commentaire }}</p>
        <p class="text-xs text-gray-400 mt-0.5">{{ formatDateShort(avis.created_at) }}</p>
      </li>
    </ul>

    <!-- Pagination -->
    <div v-if="totalPages > 1" class="flex items-center justify-center gap-3 mb-3 text-xs">
      <button
        type="button"
        :disabled="page <= 1"
        class="px-2 py-1 rounded border border-gray-300 text-gray-600 hover:bg-gray-50 disabled:opacity-40 disabled:cursor-not-allowed"
        @click="changerPage(-1)"
      >
        Précédent
      </button>
      <span class="text-gray-500">{{ page }} / {{ totalPages }}</span>
      <button
        type="button"
        :disabled="page >= totalPages"
        class="px-2 py-1 rounded border border-gray-300 text-gray-600 hover:bg-gray-50 disabled:opacity-40 disabled:cursor-not-allowed"
        @click="changerPage(1)"
      >
        Suivant
      </button>
    </div>

    <!-- Dépôt d'avis -->
    <div class="bg-gray-50 rounded-md p-3">
      <template v-if="estAuthentifie">
        <p class="text-xs font-medium text-gray-700 mb-2">Laisser un avis</p>
        <div class="flex items-center gap-1 mb-2" @mouseleave="noteSurvol = 0">
          <button
            v-for="i in 5"
            :key="i"
            type="button"
            class="p-0.5"
            :aria-label="`Attribuer ${i} étoile(s)`"
            @mouseenter="noteSurvol = i"
            @click="noteSaisie = i"
          >
            <font-awesome-icon
              :icon="['fas', 'star']"
              class="w-5 h-5 transition-colors"
              :class="i <= (noteSurvol || noteSaisie) ? 'text-amber-400' : 'text-gray-300'"
            />
          </button>
        </div>
        <textarea
          v-model="commentaireSaisi"
          rows="2"
          maxlength="2000"
          placeholder="Partagez votre expérience…"
          class="w-full px-3 py-2 text-sm border border-gray-300 rounded-md focus:ring-2 focus:ring-custom-chocolat focus:border-transparent resize-y"
        />
        <p v-if="messageErreur" class="text-xs text-red-600 mt-1">{{ messageErreur }}</p>
        <p v-if="messageSucces" class="text-xs text-green-600 mt-1">{{ messageSucces }}</p>
        <button
          type="button"
          :disabled="envoiEnCours"
          class="mt-2 px-3 py-1.5 text-xs font-medium text-white bg-custom-chocolat rounded-md hover:bg-custom-chocolat/90 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
          @click="soumettre"
        >
          {{ envoiEnCours ? 'Envoi…' : 'Publier mon avis' }}
        </button>
      </template>
      <template v-else>
        <p class="text-xs text-gray-600">
          <button type="button" class="text-custom-chocolat font-medium hover:underline" @click="allerConnexion">
            Connectez-vous
          </button>
          pour laisser un avis.
        </p>
      </template>
    </div>
  </div>
</template>
