<script setup lang="ts">
// Fil de commentaires d'un contenu média (FR-024).
// Liste PLATE et paginée — la spécification ne demande pas de fil de réponses,
// et aucun précédent du projet n'en propose. La suppression est réservée à
// l'auteur, le serveur refusant toute autre demande.

import {
  useMediaSocial,
  type CommentaireMediaAPI,
  type TypeMedia,
} from '~/composables/useMediaSocial'

const props = defineProps<{
  typeMedia: TypeMedia
  mediaId: string
  /** Fond sombre (bandeau de section) plutôt que clair (page de détail). */
  sombre?: boolean
}>()

const emit = defineEmits<{
  (e: 'require-login'): void
  (e: 'total', valeur: number): void
}>()

const { listerCommentaires, commenter, supprimerCommentaire, estConnecte } = useMediaSocial()

const MAX = 2000
const commentaires = ref<CommentaireMediaAPI[]>([])
const total = ref(0)
const page = ref(1)
const totalPages = ref(1)
const chargement = ref(false)
const envoiEnCours = ref(false)
const nouveau = ref('')
const erreur = ref('')

const restant = computed(() => MAX - nouveau.value.length)
const resteAcharger = computed(() => page.value < totalPages.value)

const charger = async (numeroPage = 1) => {
  chargement.value = true
  const res = await listerCommentaires(props.typeMedia, props.mediaId, numeroPage)
  chargement.value = false
  if (!res) return

  // Page 1 = rechargement complet ; les suivantes s'ajoutent à la suite.
  commentaires.value = numeroPage === 1 ? res.commentaires : [...commentaires.value, ...res.commentaires]
  total.value = res.total
  page.value = res.page
  totalPages.value = res.total_pages
  emit('total', res.total)
}

const chargerSuite = () => {
  if (!resteAcharger.value || chargement.value) return
  charger(page.value + 1)
}

const envoyer = async () => {
  if (!estConnecte()) {
    emit('require-login')
    return
  }
  const contenu = nouveau.value.trim()
  if (!contenu) {
    erreur.value = 'Votre commentaire est vide.'
    return
  }
  if (contenu.length > MAX) {
    erreur.value = `Le commentaire ne doit pas dépasser ${MAX} caractères.`
    return
  }

  erreur.value = ''
  envoiEnCours.value = true
  try {
    const cree = await commenter(props.typeMedia, props.mediaId, contenu)
    if (cree) {
      commentaires.value = [cree, ...commentaires.value]
      total.value += 1
      emit('total', total.value)
      nouveau.value = ''
    }
    else {
      erreur.value = 'Erreur lors de l’envoi. Veuillez réessayer.'
    }
  }
  catch {
    erreur.value = 'Erreur lors de l’envoi. Veuillez réessayer.'
  }
  envoiEnCours.value = false
}

const supprimer = async (commentaire: CommentaireMediaAPI) => {
  if (!commentaire.est_mien) return
  const ok = await supprimerCommentaire(commentaire.id)
  if (ok) {
    commentaires.value = commentaires.value.filter(c => c.id !== commentaire.id)
    total.value = Math.max(0, total.value - 1)
    emit('total', total.value)
  }
}

const nomAuteur = (c: CommentaireMediaAPI) =>
  `${c.auteur.prenom ?? ''} ${c.auteur.nom ?? ''}`.trim() || 'Membre'

const initiales = (c: CommentaireMediaAPI) =>
  ((c.auteur.prenom ?? '').charAt(0) + (c.auteur.nom ?? '').charAt(0)).toUpperCase() || '?'

const config = useRuntimeConfig()
const apiBase = config.public.apiBaseUrl as string
const photo = (c: CommentaireMediaAPI) => {
  const url = c.auteur.photo_url
  if (!url) return null
  return url.startsWith('http') ? url : `${apiBase}${url}`
}

const dateFormatee = (iso: string) =>
  new Intl.DateTimeFormat('fr-FR', { day: 'numeric', month: 'short', year: 'numeric' })
    .format(new Date(iso))

// Recharge si le parent change de contenu sans démonter le composant.
watch(() => [props.typeMedia, props.mediaId], () => charger(1))
onMounted(() => charger(1))
</script>

<template>
  <section :class="sombre ? 'text-white' : 'text-gray-900'">
    <h3 class="font-oswald text-lg font-bold mb-4">
      Commentaires
      <span :class="sombre ? 'text-white/60' : 'text-gray-400'" class="font-normal">({{ total }})</span>
    </h3>

    <!-- Saisie -->
    <div v-if="estConnecte()" class="mb-6">
      <textarea
        v-model="nouveau"
        rows="3"
        :maxlength="MAX"
        placeholder="Partagez votre avis sur ce contenu…"
        class="w-full px-3.5 py-2.5 rounded-lg text-sm resize-none focus:ring-2 focus:ring-custom-green focus:border-transparent"
        :class="sombre
          ? 'bg-white/10 border border-white/20 text-white placeholder-white/40'
          : 'bg-white border border-gray-300 text-gray-900'"
        :disabled="envoiEnCours"
      ></textarea>
      <div class="flex items-center justify-between mt-2">
        <p v-if="erreur" class="text-sm text-red-500">{{ erreur }}</p>
        <span v-else></span>
        <div class="flex items-center gap-3">
          <span class="text-xs" :class="restant < 0 ? 'text-red-500' : (sombre ? 'text-white/50' : 'text-gray-400')">
            {{ restant }}
          </span>
          <button
            type="button"
            class="px-4 py-2 text-sm font-medium text-white bg-custom-green rounded-lg hover:bg-custom-green/90 transition-colors cursor-pointer disabled:opacity-60 inline-flex items-center gap-2"
            :disabled="envoiEnCours"
            @click="envoyer"
          >
            <font-awesome-icon v-if="envoiEnCours" :icon="['fas', 'spinner']" class="w-4 h-4 animate-spin" />
            <font-awesome-icon v-else :icon="['fas', 'paper-plane']" class="w-4 h-4" />
            Publier
          </button>
        </div>
      </div>
    </div>

    <p v-else class="mb-6 text-sm" :class="sombre ? 'text-white/70' : 'text-gray-500'">
      <NuxtLink to="/login" class="text-custom-green font-medium hover:underline">Connectez-vous</NuxtLink>
      pour laisser un commentaire.
    </p>

    <!-- Liste -->
    <p
      v-if="!chargement && commentaires.length === 0"
      class="text-sm py-6 text-center"
      :class="sombre ? 'text-white/60' : 'text-gray-500'"
    >
      Aucun commentaire pour l’instant — soyez le premier à réagir.
    </p>

    <ul v-else class="space-y-4">
      <li
        v-for="commentaire in commentaires"
        :key="commentaire.id"
        class="flex gap-3 rounded-xl p-4"
        :class="sombre ? 'bg-white/5' : 'bg-gray-50'"
      >
        <div class="shrink-0">
          <img
            v-if="photo(commentaire)"
            :src="photo(commentaire)!"
            :alt="nomAuteur(commentaire)"
            class="w-10 h-10 rounded-full object-cover"
          >
          <div
            v-else
            class="w-10 h-10 rounded-full bg-linear-to-br from-custom-chocolat to-amber-600 flex items-center justify-center text-white font-bold text-xs"
          >
            {{ initiales(commentaire) }}
          </div>
        </div>

        <div class="min-w-0 flex-1">
          <div class="flex items-center gap-2 flex-wrap">
            <span class="font-semibold text-sm">{{ nomAuteur(commentaire) }}</span>
            <span class="text-xs" :class="sombre ? 'text-white/50' : 'text-gray-400'">
              {{ dateFormatee(commentaire.created_at) }}
            </span>
            <button
              v-if="commentaire.est_mien"
              type="button"
              class="ml-auto text-xs text-red-500 hover:underline cursor-pointer"
              @click="supprimer(commentaire)"
            >
              Supprimer
            </button>
          </div>
          <p class="text-sm mt-1 whitespace-pre-line" :class="sombre ? 'text-white/85' : 'text-gray-700'">
            {{ commentaire.contenu }}
          </p>
        </div>
      </li>
    </ul>

    <button
      v-if="resteAcharger"
      type="button"
      class="mt-4 w-full py-2.5 text-sm font-medium rounded-lg transition-colors cursor-pointer disabled:opacity-60"
      :class="sombre
        ? 'bg-white/10 text-white hover:bg-white/20'
        : 'bg-gray-100 text-gray-700 hover:bg-gray-200'"
      :disabled="chargement"
      @click="chargerSuite"
    >
      {{ chargement ? 'Chargement…' : 'Afficher plus de commentaires' }}
    </button>
  </section>
</template>
