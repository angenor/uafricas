<script setup lang="ts">
import type {
  BiblioHumaineAPI,
  CommentaireBiblioAPI,
  RecommandationBiblioAPI,
} from '~/composables/useBibliothequeHumaine'
import { useUserStore } from '~/stores/user'

const props = defineProps<{
  biblio: BiblioHumaineAPI
  /** Membre connecté, hors son propre profil */
  peutInteragir: boolean
}>()

const {
  reagirBiblio,
  listerCommentairesBiblio,
  ajouterCommentaireBiblio,
  supprimerCommentaireBiblio,
  recommanderBiblio,
  retirerRecommandationBiblio,
  listerRecommandationsBiblio,
} = useBibliothequeHumaine()

const userStore = useUserStore()
const config = useRuntimeConfig()
const apiBase = config.public.apiBaseUrl as string

const photo = (url: string | null) => {
  if (!url) return null
  return url.startsWith('http') ? url : `${apiBase}${url}`
}

const formatDate = (iso: string) =>
  new Date(iso).toLocaleDateString('fr-FR', { day: 'numeric', month: 'long', year: 'numeric' })

const initiales = (prenom: string, nom: string) =>
  `${prenom[0] ?? ''}${nom[0] ?? ''}`.toUpperCase()

// ── État local des compteurs / état perso (amorcé depuis la biblio) ──────────
const nombreLikes = ref(props.biblio.nombreLikes ?? 0)
const nombreDislikes = ref(props.biblio.nombreDislikes ?? 0)
const nombreRecommandations = ref(props.biblio.nombreRecommandations ?? 0)
const maReaction = ref<'like' | 'dislike' | null>(props.biblio.maReaction ?? null)
const aRecommande = ref(props.biblio.aRecommande ?? false)
const reactionEnCours = ref(false)

// ── Réactions ────────────────────────────────────────────────────────────────
const reagir = async (type: 'like' | 'dislike') => {
  if (!props.peutInteragir || reactionEnCours.value) return
  reactionEnCours.value = true
  const res = await reagirBiblio(props.biblio.userId, type)
  if (res) {
    nombreLikes.value = res.nombreLikes
    nombreDislikes.value = res.nombreDislikes
    maReaction.value = res.maReaction
  }
  reactionEnCours.value = false
}

// ── Recommandation ─────────────────────────────────────────────────────────
const afficherModalReco = ref(false)
const messageReco = ref('')
const recoEnCours = ref(false)

const ouvrirRecommandation = () => {
  messageReco.value = ''
  afficherModalReco.value = true
}

const soumettreRecommandation = async () => {
  recoEnCours.value = true
  const res = await recommanderBiblio(props.biblio.userId, messageReco.value.trim() || undefined)
  if (res) {
    nombreRecommandations.value = res.nombreRecommandations
    aRecommande.value = res.aRecommande
    afficherModalReco.value = false
    await chargerRecommandations()
  }
  recoEnCours.value = false
}

const retirerRecommandation = async () => {
  recoEnCours.value = true
  const res = await retirerRecommandationBiblio(props.biblio.userId)
  if (res) {
    nombreRecommandations.value = res.nombreRecommandations
    aRecommande.value = res.aRecommande
    await chargerRecommandations()
  }
  recoEnCours.value = false
}

// ── Commentaires ─────────────────────────────────────────────────────────────
const commentaires = ref<CommentaireBiblioAPI[]>([])
const nouveauCommentaire = ref('')
const commentaireEnCours = ref(false)

const chargerCommentaires = async () => {
  commentaires.value = await listerCommentairesBiblio(props.biblio.userId)
}

const envoyerCommentaire = async () => {
  const contenu = nouveauCommentaire.value.trim()
  if (!contenu || commentaireEnCours.value) return
  commentaireEnCours.value = true
  const res = await ajouterCommentaireBiblio(props.biblio.userId, contenu)
  if (res) {
    commentaires.value = [res, ...commentaires.value]
    nouveauCommentaire.value = ''
  }
  commentaireEnCours.value = false
}

const supprimerCommentaire = async (id: string) => {
  const ok = await supprimerCommentaireBiblio(props.biblio.userId, id)
  if (ok) commentaires.value = commentaires.value.filter(c => c.id !== id)
}

// ── Recommandations (témoignages) ────────────────────────────────────────────
const recommandations = ref<RecommandationBiblioAPI[]>([])
const chargerRecommandations = async () => {
  recommandations.value = await listerRecommandationsBiblio(props.biblio.userId)
}

onMounted(() => {
  chargerCommentaires()
  chargerRecommandations()
})
</script>

<template>
  <div class="space-y-8">
    <!-- Barre d'actions : aimer / ne pas aimer / recommander -->
    <div class="flex flex-wrap items-center gap-3 pt-2 border-t border-gray-100">
      <button
        type="button"
        :disabled="!peutInteragir || reactionEnCours"
        class="inline-flex items-center gap-2 px-4 py-2 text-sm font-semibold rounded-xl border transition disabled:opacity-60 disabled:cursor-not-allowed"
        :class="maReaction === 'like'
          ? 'bg-custom-green text-white border-custom-green'
          : 'border-gray-200 text-gray-600 hover:border-custom-green hover:text-custom-green'"
        @click="reagir('like')"
      >
        <font-awesome-icon icon="fa-solid fa-thumbs-up" />
        J'aime
        <span class="tabular-nums">{{ nombreLikes }}</span>
      </button>

      <button
        type="button"
        :disabled="!peutInteragir || reactionEnCours"
        class="inline-flex items-center gap-2 px-4 py-2 text-sm font-semibold rounded-xl border transition disabled:opacity-60 disabled:cursor-not-allowed"
        :class="maReaction === 'dislike'
          ? 'bg-gray-700 text-white border-gray-700'
          : 'border-gray-200 text-gray-600 hover:border-gray-400 hover:text-gray-800'"
        @click="reagir('dislike')"
      >
        <font-awesome-icon icon="fa-solid fa-thumbs-down" />
        Je n'aime pas
        <span class="tabular-nums">{{ nombreDislikes }}</span>
      </button>

      <button
        v-if="!aRecommande"
        type="button"
        :disabled="!peutInteragir || recoEnCours"
        class="inline-flex items-center gap-2 px-4 py-2 text-sm font-semibold rounded-xl border border-custom-chocolat text-custom-chocolat hover:bg-custom-chocolat hover:text-white transition disabled:opacity-60 disabled:cursor-not-allowed"
        @click="ouvrirRecommandation"
      >
        <font-awesome-icon icon="fa-solid fa-award" />
        Recommander
        <span class="tabular-nums">{{ nombreRecommandations }}</span>
      </button>
      <button
        v-else
        type="button"
        :disabled="recoEnCours"
        class="inline-flex items-center gap-2 px-4 py-2 text-sm font-semibold rounded-xl bg-custom-chocolat text-white border border-custom-chocolat transition disabled:opacity-60"
        @click="retirerRecommandation"
      >
        <font-awesome-icon icon="fa-solid fa-award" />
        Recommandé
        <span class="tabular-nums">{{ nombreRecommandations }}</span>
      </button>
    </div>

    <p v-if="!peutInteragir" class="text-xs text-gray-400 -mt-4">
      Connectez-vous pour aimer, commenter ou recommander cette bibliothèque humaine.
    </p>

    <!-- Recommandations (témoignages) -->
    <div v-if="recommandations.length > 0">
      <h3 class="text-sm font-semibold text-gray-500 uppercase tracking-wide mb-3 flex items-center gap-2">
        <font-awesome-icon icon="fa-solid fa-award" class="text-custom-chocolat" />
        Recommandations ({{ recommandations.length }})
      </h3>
      <div class="space-y-3">
        <div
          v-for="r in recommandations"
          :key="r.id"
          class="flex gap-3 p-4 rounded-xl bg-custom-chocolat/5 border border-custom-chocolat/10"
        >
          <NuxtLink :to="`/profil/${r.auteurId}`" class="shrink-0">
            <img
              v-if="photo(r.auteurPhotoUrl)"
              :src="photo(r.auteurPhotoUrl)!"
              :alt="r.auteurPrenom + ' ' + r.auteurNom"
              class="w-10 h-10 rounded-full object-cover"
            />
            <div
              v-else
              class="w-10 h-10 rounded-full bg-custom-chocolat text-white flex items-center justify-center text-xs font-bold"
            >
              {{ initiales(r.auteurPrenom, r.auteurNom) }}
            </div>
          </NuxtLink>
          <div class="min-w-0">
            <NuxtLink
              :to="`/profil/${r.auteurId}`"
              class="text-sm font-semibold text-gray-800 hover:text-custom-chocolat"
            >
              {{ r.auteurPrenom }} {{ r.auteurNom }}
            </NuxtLink>
            <span v-if="r.auteurFonction" class="text-xs text-gray-400"> · {{ r.auteurFonction }}</span>
            <p v-if="r.message" class="text-sm text-gray-700 mt-1 whitespace-pre-line italic">« {{ r.message }} »</p>
            <p class="text-xs text-gray-400 mt-1">{{ formatDate(r.createdAt) }}</p>
          </div>
        </div>
      </div>
    </div>

    <!-- Commentaires -->
    <div>
      <h3 class="text-sm font-semibold text-gray-500 uppercase tracking-wide mb-3 flex items-center gap-2">
        <font-awesome-icon icon="fa-solid fa-comment" class="text-custom-chocolat" />
        Commentaires ({{ commentaires.length }})
      </h3>

      <!-- Formulaire -->
      <div v-if="peutInteragir" class="mb-5">
        <textarea
          v-model="nouveauCommentaire"
          rows="3"
          maxlength="2000"
          placeholder="Partagez votre expérience avec cette bibliothèque humaine…"
          class="w-full rounded-xl border border-gray-200 px-4 py-2.5 text-sm text-gray-700 focus:outline-none focus:border-custom-chocolat focus:ring-1 focus:ring-custom-chocolat resize-none"
        ></textarea>
        <div class="flex items-center justify-between mt-2">
          <span class="text-xs text-gray-400">{{ nouveauCommentaire.length }}/2000</span>
          <button
            type="button"
            :disabled="!nouveauCommentaire.trim() || commentaireEnCours"
            class="inline-flex items-center gap-2 px-4 py-2 text-sm font-semibold rounded-xl bg-custom-chocolat text-white hover:shadow-lg transition disabled:opacity-50 disabled:cursor-not-allowed"
            @click="envoyerCommentaire"
          >
            <font-awesome-icon :icon="commentaireEnCours ? 'fa-solid fa-spinner' : 'fa-solid fa-paper-plane'" :class="{ 'animate-spin': commentaireEnCours }" />
            Publier
          </button>
        </div>
      </div>

      <!-- Liste -->
      <div v-if="commentaires.length > 0" class="space-y-3">
        <div
          v-for="c in commentaires"
          :key="c.id"
          class="flex gap-3 p-4 rounded-xl bg-gray-50 border border-gray-100"
        >
          <NuxtLink :to="`/profil/${c.auteurId}`" class="shrink-0">
            <img
              v-if="photo(c.auteurPhotoUrl)"
              :src="photo(c.auteurPhotoUrl)!"
              :alt="c.auteurPrenom + ' ' + c.auteurNom"
              class="w-10 h-10 rounded-full object-cover"
            />
            <div
              v-else
              class="w-10 h-10 rounded-full bg-gray-400 text-white flex items-center justify-center text-xs font-bold"
            >
              {{ initiales(c.auteurPrenom, c.auteurNom) }}
            </div>
          </NuxtLink>
          <div class="min-w-0 flex-1">
            <div class="flex items-center justify-between gap-2">
              <NuxtLink
                :to="`/profil/${c.auteurId}`"
                class="text-sm font-semibold text-gray-800 hover:text-custom-chocolat"
              >
                {{ c.auteurPrenom }} {{ c.auteurNom }}
              </NuxtLink>
              <button
                v-if="c.auteurId === userStore.user?.id"
                type="button"
                class="text-xs text-gray-300 hover:text-red-500 transition"
                title="Supprimer"
                @click="supprimerCommentaire(c.id)"
              >
                <font-awesome-icon icon="fa-solid fa-trash" />
              </button>
            </div>
            <p class="text-sm text-gray-700 mt-1 whitespace-pre-line">{{ c.contenu }}</p>
            <p class="text-xs text-gray-400 mt-1">{{ formatDate(c.createdAt) }}</p>
          </div>
        </div>
      </div>
      <p v-else class="text-sm text-gray-400">Aucun commentaire pour le moment.</p>
    </div>

    <!-- Modale de recommandation -->
    <Teleport to="body">
      <Transition
        enter-active-class="transition duration-200 ease-out"
        enter-from-class="opacity-0"
        enter-to-class="opacity-100"
        leave-active-class="transition duration-150 ease-in"
        leave-from-class="opacity-100"
        leave-to-class="opacity-0"
      >
        <div
          v-if="afficherModalReco"
          class="fixed inset-0 z-[90] flex items-center justify-center p-4"
          @click.self="afficherModalReco = false"
        >
          <div class="absolute inset-0 bg-black/50 backdrop-blur-sm"></div>
          <div class="relative bg-white rounded-2xl shadow-2xl w-full max-w-md p-6">
            <h3 class="text-lg font-semibold text-gray-800 flex items-center gap-2 mb-1">
              <font-awesome-icon icon="fa-solid fa-award" class="text-custom-chocolat" />
              Recommander {{ biblio.prenom }} {{ biblio.nom }}
            </h3>
            <p class="text-sm text-gray-500 mb-4">
              Ajoutez un témoignage (facultatif) pour appuyer votre recommandation.
            </p>
            <textarea
              v-model="messageReco"
              rows="4"
              maxlength="1000"
              placeholder="Ex. : Une rencontre marquante, des conseils précieux…"
              class="w-full rounded-xl border border-gray-200 px-4 py-2.5 text-sm text-gray-700 focus:outline-none focus:border-custom-chocolat focus:ring-1 focus:ring-custom-chocolat resize-none"
            ></textarea>
            <span class="text-xs text-gray-400">{{ messageReco.length }}/1000</span>
            <div class="flex items-center justify-end gap-3 mt-4">
              <button
                type="button"
                class="px-4 py-2 text-sm font-medium text-gray-500 hover:text-gray-700"
                @click="afficherModalReco = false"
              >
                Annuler
              </button>
              <button
                type="button"
                :disabled="recoEnCours"
                class="inline-flex items-center gap-2 px-5 py-2 text-sm font-semibold rounded-xl bg-custom-chocolat text-white hover:shadow-lg transition disabled:opacity-50"
                @click="soumettreRecommandation"
              >
                <font-awesome-icon :icon="recoEnCours ? 'fa-solid fa-spinner' : 'fa-solid fa-check'" :class="{ 'animate-spin': recoEnCours }" />
                Recommander
              </button>
            </div>
          </div>
        </div>
      </Transition>
    </Teleport>
  </div>
</template>
